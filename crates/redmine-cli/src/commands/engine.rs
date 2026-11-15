//! Motor declarativo: resolve a requisição, executa (ou imprime o plano) e extrai o valor.

use std::path::Path;

use redmine_wrapper::{HttpMethod, RedmineClient};
use serde_json::{Map, Value, json};

use crate::cli::Context;
use crate::cli::spec::{ActionDef, BodySpec, Extract, ParamsMode, PathSpec, Special};
use crate::commands::input::{self, Item};
use crate::commands::params;
use crate::error::CliError;
use crate::output::{Output, error_body};

/// Requisição resolvida, independente de transporte.
struct RequestPlan {
    method: HttpMethod,
    path: String,
    query: params::QueryPairs,
    body: Option<Value>,
    extract: Extract,
    paginated: Option<&'static str>,
}

enum BatchEntry {
    Data(Value),
    Dry(Value),
}

/// Executa uma ação de grupo.
///
/// # Errors
/// Uso inválido, erro de I/O, JSON malformado ou erro da API Redmine.
pub async fn run_action(
    client: &RedmineClient,
    invocation: crate::cli::ActionInvocation,
) -> Result<Output, CliError> {
    let command = format!("{}.{}", invocation.group, invocation.def.name);
    run_def(
        client,
        invocation.def,
        &invocation.context,
        invocation.params.as_deref(),
        invocation.batch.as_deref(),
        invocation.dry_run,
        &command,
    )
    .await
}

/// Núcleo compartilhado por ações, `search` e `api`.
pub async fn run_def(
    client: &RedmineClient,
    def: &ActionDef,
    context: &Context,
    params_raw: Option<&str>,
    batch_raw: Option<&str>,
    dry_run: bool,
    command: &str,
) -> Result<Output, CliError> {
    match def.special {
        Special::AttachmentUpload => {
            return attachment_upload(client, context, dry_run, command).await;
        }
        Special::FileUpload => {
            return file_upload(client, context, dry_run, command).await;
        }
        _ => {}
    }

    if let Some(source) = batch_raw {
        if params_raw.is_some() {
            return Err(CliError::Usage("--batch e --params são exclusivos".into()));
        }
        if context.query.as_deref() == Some("-") {
            return Err(CliError::Usage(
                "a consulta não pode ser `-` com --batch; use a chave query nas linhas".into(),
            ));
        }
        return run_batch(client, def, context, source, dry_run, command).await;
    }

    let implicit =
        def.special != Special::Api && body_needs_params(def.body) && params_raw.is_none();
    let params = input::read_params(params_raw, implicit)?;

    let mut item = Item::from_context(context, params);
    if def.special == Special::Search {
        item.query = Some(input::resolve_query(item.query.as_deref())?);
    }

    if context.ids.len() > 1 {
        let mut values = Vec::with_capacity(context.ids.len());
        for id in &context.ids {
            let mut item = item.clone();
            item.id = Some(id.clone());
            let plan = build_request(def, context, &item)?;
            values.push(execute_or_plan(client, dry_run, &plan).await?);
        }
        return Ok(Output::single(command.to_string(), Value::Array(values)));
    }

    let plan = build_request(def, context, &item)?;
    let data = execute_or_plan(client, dry_run, &plan).await?;
    Ok(Output::single(command.to_string(), data))
}

fn body_needs_params(body: BodySpec) -> bool {
    matches!(body, BodySpec::Wrapped(_) | BodySpec::Raw)
}

async fn execute_or_plan(
    client: &RedmineClient,
    dry_run: bool,
    plan: &RequestPlan,
) -> Result<Value, CliError> {
    if dry_run {
        Ok(json!({"dry_run": true, "request": plan_json(plan)}))
    } else {
        execute_request(client, plan).await
    }
}

async fn run_batch(
    client: &RedmineClient,
    def: &ActionDef,
    context: &Context,
    source: &str,
    dry_run: bool,
    command: &str,
) -> Result<Output, CliError> {
    let lines = input::read_batch(source)?;
    let mut results = Vec::with_capacity(lines.len());
    let mut failed = false;

    for (index, text) in lines.iter().enumerate() {
        let outcome = match input::parse_item(text, index) {
            Ok(item) => match build_request(def, context, &item) {
                Ok(plan) => {
                    if dry_run {
                        Ok(BatchEntry::Dry(plan_json(&plan)))
                    } else {
                        execute_request(client, &plan).await.map(BatchEntry::Data)
                    }
                }
                Err(error) => Err(error),
            },
            Err(error) => Err(error),
        };

        match outcome {
            Ok(BatchEntry::Data(data)) => {
                results.push(json!({"index": index, "ok": true, "data": data}));
            }
            Ok(BatchEntry::Dry(request)) => {
                results
                    .push(json!({"index": index, "ok": true, "dry_run": true, "request": request}));
            }
            Err(error) => {
                failed = true;
                results.push(json!({"index": index, "ok": false, "error": error_body(&error)}));
            }
        }
    }

    Ok(Output::lines(command.to_string(), results, failed))
}

fn build_request(def: &ActionDef, context: &Context, item: &Item) -> Result<RequestPlan, CliError> {
    match def.special {
        Special::Search => build_search(context, item),
        Special::Api => build_api(context, item),
        _ => build_crud(def, context, item),
    }
}

fn build_search(context: &Context, item: &Item) -> Result<RequestPlan, CliError> {
    let query_text = item
        .query
        .clone()
        .or_else(|| context.query.clone())
        .ok_or_else(|| CliError::Usage("informe a consulta de busca".into()))?;
    let mut query = match &item.params {
        Some(params) => params::params_to_query(params)?,
        None => Vec::new(),
    };
    if query.iter().any(|(key, _)| key == "q") {
        return Err(CliError::Usage(
            "--params não aceita `q`; a consulta é o posicional".into(),
        ));
    }
    params::set(&mut query, "q", query_text);
    Ok(RequestPlan {
        method: HttpMethod::Get,
        path: "/search.json".into(),
        query,
        body: None,
        extract: Extract::Paginated("results"),
        paginated: Some("results"),
    })
}

fn build_api(context: &Context, item: &Item) -> Result<RequestPlan, CliError> {
    let method_raw = context
        .method
        .as_deref()
        .ok_or_else(|| CliError::Usage("informe --method".into()))?;
    let method = HttpMethod::parse(method_raw)?;
    let path = item
        .path
        .clone()
        .or_else(|| context.path.clone())
        .ok_or_else(|| CliError::Usage("informe --path".into()))?;
    if !path.starts_with('/') {
        return Err(CliError::Usage("--path deve começar por `/`".into()));
    }
    if path.contains('?') {
        return Err(CliError::Usage(
            "--path não deve conter query string; use --params".into(),
        ));
    }

    let (query, body) = match method {
        HttpMethod::Get | HttpMethod::Delete => (
            match &item.params {
                Some(params) => params::params_to_query(params)?,
                None => Vec::new(),
            },
            None,
        ),
        HttpMethod::Post | HttpMethod::Put | HttpMethod::Patch => (
            Vec::new(),
            Some(item.params.clone().unwrap_or_else(|| json!({}))),
        ),
        _ => {
            return Err(CliError::Usage(format!(
                "método não suportado: {method_raw}"
            )));
        }
    };

    Ok(RequestPlan {
        method,
        path,
        query,
        body,
        extract: Extract::Whole,
        paginated: None,
    })
}

fn build_crud(def: &ActionDef, context: &Context, item: &Item) -> Result<RequestPlan, CliError> {
    let id = item.id.clone().or_else(|| context.ids.first().cloned());
    let project = item.project.clone().or_else(|| context.project.clone());
    let title = item.title.clone().or_else(|| context.title.clone());
    let user = item.user.clone().or_else(|| context.user.clone());
    let version = context.version.map(|value| value.to_string());

    let mut vars: Vec<(&str, String)> = Vec::new();
    if let Some(value) = &id {
        vars.push(("id", value.clone()));
    }
    if let Some(value) = &project {
        vars.push(("project", value.clone()));
    }
    if let Some(value) = &title {
        vars.push(("title", value.clone()));
    }
    if let Some(value) = &user {
        vars.push(("user", value.clone()));
    }
    if let Some(value) = &version {
        vars.push(("version", value.clone()));
    }

    let path = crud_path(def, project.is_some(), &vars)?;
    let mut query = crud_query(def, item)?;
    params::apply_fixed(&mut query, def.fixed_query);
    if let Some(reassign) = &context.reassign_to {
        params::set(&mut query, "reassign_to_id", reassign.clone());
    }
    let body = crud_body(def, item, user)?;

    let paginated = match def.extract {
        Extract::Paginated(key) => Some(key),
        _ => None,
    };

    Ok(RequestPlan {
        method: def.method,
        path,
        query,
        body,
        extract: def.extract,
        paginated,
    })
}

fn crud_path(
    def: &ActionDef,
    has_project: bool,
    vars: &[(&str, String)],
) -> Result<String, CliError> {
    match def.path {
        PathSpec::Static(value) => Ok(value.to_string()),
        PathSpec::Target(template) => params::resolve_path(template, vars),
        PathSpec::OptionalScope { global, scoped } => {
            if has_project {
                params::resolve_path(scoped, vars)
            } else {
                Ok(global.to_string())
            }
        }
    }
}

fn crud_query(def: &ActionDef, item: &Item) -> Result<params::QueryPairs, CliError> {
    match def.params {
        ParamsMode::Query => match &item.params {
            Some(params) => params::params_to_query(params),
            None => Ok(Vec::new()),
        },
        ParamsMode::Body | ParamsMode::Api => Ok(Vec::new()),
        ParamsMode::None => {
            if item.params.is_some() {
                return Err(CliError::Usage("esta ação não aceita --params".into()));
            }
            Ok(Vec::new())
        }
    }
}

fn crud_body(
    def: &ActionDef,
    item: &Item,
    user: Option<String>,
) -> Result<Option<Value>, CliError> {
    match def.body {
        BodySpec::None => Ok(None),
        BodySpec::Empty => {
            if item.params.is_some() {
                return Err(CliError::Usage("esta ação não aceita --params".into()));
            }
            Ok(Some(json!({})))
        }
        BodySpec::Wrapped(key) => {
            let payload = item.params.as_ref().ok_or_else(|| {
                CliError::Usage("informe --params ou envie o objeto JSON pelo stdin".into())
            })?;
            if !payload.is_object() {
                return Err(CliError::Usage("--params deve ser um objeto JSON".into()));
            }
            let mut wrapped = Map::new();
            wrapped.insert((*key).to_string(), payload.clone());
            Ok(Some(Value::Object(wrapped)))
        }
        BodySpec::UserId => {
            if item.params.is_some() {
                return Err(CliError::Usage("esta ação não aceita --params".into()));
            }
            let user = user.ok_or_else(|| CliError::Usage("informe --user".into()))?;
            Ok(Some(json!({"user_id": user})))
        }
        BodySpec::Fixed(text) => {
            if item.params.is_some() {
                return Err(CliError::Usage("esta ação não aceita --params".into()));
            }
            let body = serde_json::from_str(text)
                .map_err(|error| CliError::Json("corpo fixo".into(), error))?;
            Ok(Some(body))
        }
        BodySpec::Raw => Ok(Some(item.params.clone().unwrap_or_else(|| json!({})))),
    }
}

async fn execute_request(client: &RedmineClient, plan: &RequestPlan) -> Result<Value, CliError> {
    let query = as_query(&plan.query);
    if let Some(key) = plan.paginated {
        // `limit`/`offset` explícitos pedem uma página só: não paginar tudo.
        if plan
            .query
            .iter()
            .any(|(name, _)| name == "limit" || name == "offset")
        {
            let value = client
                .request_json(plan.method, &plan.path, &query, None)
                .await?;
            return Ok(value
                .get(key)
                .cloned()
                .unwrap_or_else(|| Value::Array(Vec::new())));
        }
        let items = client.get_all_json(&plan.path, key, &query).await?;
        return Ok(Value::Array(items));
    }
    let value = client
        .request_json(plan.method, &plan.path, &query, plan.body.as_ref())
        .await?;
    extract_value(value, plan.extract)
}

fn as_query(pairs: &params::QueryPairs) -> Vec<(&str, String)> {
    pairs
        .iter()
        .map(|(key, value)| (key.as_str(), value.clone()))
        .collect()
}

fn extract_value(value: Value, extract: Extract) -> Result<Value, CliError> {
    match extract {
        Extract::Whole | Extract::Paginated(_) => Ok(value),
        Extract::Path(path) => {
            let mut current = value;
            for key in path {
                current = current.get(*key).cloned().ok_or_else(|| {
                    CliError::Internal(format!("campo `{key}` ausente na resposta"))
                })?;
            }
            Ok(current)
        }
    }
}

fn plan_json(plan: &RequestPlan) -> Value {
    let query: Vec<Value> = plan
        .query
        .iter()
        .map(|(key, value)| json!([key, value]))
        .collect();
    json!({
        "method": plan.method.as_str(),
        "path": plan.path,
        "query": query,
        "body": plan.body,
    })
}

fn upload_name(context: &Context, file: &Path) -> String {
    context
        .filename
        .clone()
        .or_else(|| {
            file.file_name()
                .and_then(|name| name.to_str())
                .map(str::to_string)
        })
        .unwrap_or_else(|| "arquivo".to_string())
}

async fn attachment_upload(
    client: &RedmineClient,
    context: &Context,
    dry_run: bool,
    command: &str,
) -> Result<Output, CliError> {
    let file = context
        .file
        .as_deref()
        .ok_or_else(|| CliError::Usage("informe --file".into()))?;
    let filename = upload_name(context, file);

    if dry_run {
        return Ok(Output::single(
            command.to_string(),
            json!({
                "dry_run": true,
                "request": {
                    "method": "POST",
                    "path": "/uploads.json",
                    "query": [["filename", filename]],
                    "body": "<binário>",
                },
            }),
        ));
    }

    let data = std::fs::read(file)
        .map_err(|error| CliError::Io(format!("ler {}", file.display()), error))?;
    let token = client.attachments.upload(&filename, &data).await?;
    Ok(Output::single(command.to_string(), json!({"token": token})))
}

async fn file_upload(
    client: &RedmineClient,
    context: &Context,
    dry_run: bool,
    command: &str,
) -> Result<Output, CliError> {
    let file = context
        .file
        .as_deref()
        .ok_or_else(|| CliError::Usage("informe --file".into()))?;
    let project = context
        .project
        .as_deref()
        .ok_or_else(|| CliError::Usage("informe --project".into()))?;
    let filename = upload_name(context, file);
    let path = format!("/projects/{}/files.json", params::encode_segment(project));

    if dry_run {
        return Ok(Output::single(
            command.to_string(),
            json!({
                "dry_run": true,
                "requests": [
                    {
                        "method": "POST",
                        "path": "/uploads.json",
                        "query": [["filename", filename]],
                    },
                    {
                        "method": "POST",
                        "path": path,
                        "body": {
                            "file": {
                                "token": "<token>",
                                "filename": filename,
                                "description": context.description,
                            },
                        },
                    },
                ],
            }),
        ));
    }

    let data = std::fs::read(file)
        .map_err(|error| CliError::Io(format!("ler {}", file.display()), error))?;
    let token = client.attachments.upload(&filename, &data).await?;

    let mut meta = Map::new();
    meta.insert("token".to_string(), Value::String(token));
    meta.insert("filename".to_string(), Value::String(filename));
    if let Some(description) = &context.description {
        meta.insert(
            "description".to_string(),
            Value::String(description.clone()),
        );
    }
    let mut root = Map::new();
    root.insert("file".to_string(), Value::Object(meta));
    let body = Value::Object(root);

    let value = client
        .request_json(HttpMethod::Post, &path, &[], Some(&body))
        .await?;
    let file = extract_value(value, Extract::Path(&["file"]))?;
    Ok(Output::single(command.to_string(), file))
}
