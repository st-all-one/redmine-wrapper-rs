//! Leitura de stdin, `--params` e lotes JSONL.

use std::io::{IsTerminal, Read};
use std::path::Path;

use serde_json::Value;

use crate::cli::Context;
use crate::error::CliError;

/// Teto de itens por lote (proteção contra arquivos gigantes).
pub const BATCH_MAX: usize = 1000;

/// Item de um lote: metadados reservados + `params` opcional.
#[derive(Debug, Clone, Default)]
pub struct Item {
    /// `id` do recurso.
    pub id: Option<String>,
    /// `project` (ID ou identificador).
    pub project: Option<String>,
    /// `title` (wiki).
    pub title: Option<String>,
    /// `user` (watchers/grupos).
    pub user: Option<String>,
    /// `path` (api).
    pub path: Option<String>,
    /// `query` (search).
    pub query: Option<String>,
    /// Objeto de filtros/corpo.
    pub params: Option<Value>,
}

impl Item {
    /// Item de uma invocação única, a partir das flags de contexto.
    #[must_use]
    pub fn from_context(context: &Context, params: Option<Value>) -> Self {
        Self {
            id: context.ids.first().cloned(),
            project: context.project.clone(),
            title: context.title.clone(),
            user: context.user.clone(),
            path: context.path.clone(),
            query: context.query.clone(),
            params,
        }
    }
}

/// Lê todo o stdin como UTF-8.
///
/// # Errors
/// Propaga falhas de I/O do stdin.
pub fn read_stdin() -> Result<String, CliError> {
    let mut buffer = String::new();
    std::io::stdin()
        .read_to_string(&mut buffer)
        .map_err(|error| CliError::Io("stdin".into(), error))?;
    Ok(buffer)
}

fn read_file(path: &Path) -> Result<String, CliError> {
    std::fs::read_to_string(path)
        .map_err(|error| CliError::Io(format!("ler {}", path.display()), error))
}

/// Resolve a consulta posicional: `-`/pipe/heredoc leem o stdin.
///
/// # Errors
/// Falta de consulta em TTY, stdin vazio ou falha de I/O.
pub fn resolve_query(raw: Option<&str>) -> Result<String, CliError> {
    if let Some(value) = raw
        && value != "-"
    {
        return Ok(value.to_string());
    }
    if std::io::stdin().is_terminal() {
        return Err(CliError::Usage(
            "informe a consulta (ex.: redmine search \"bug\") ou envie pelo stdin".into(),
        ));
    }
    let text = read_stdin()?;
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return Err(CliError::Usage("consulta vazia no stdin".into()));
    }
    Ok(trimmed.to_string())
}

/// Lê `--params` (ou o stdin implícito de uma escrita).
///
/// # Errors
/// `--params -` com stdin inválido/vazio, JSON malformado ou stdin vazio.
pub fn read_params(raw: Option<&str>, implicit: bool) -> Result<Option<Value>, CliError> {
    if let Some(text) = raw {
        let text = if text == "-" {
            let value = read_stdin()?;
            if value.trim().is_empty() {
                return Err(CliError::Usage("stdin vazio em --params -".into()));
            }
            value
        } else {
            text.to_string()
        };
        return Ok(Some(parse_json(&text, "--params")?));
    }

    if implicit && !std::io::stdin().is_terminal() {
        let text = read_stdin()?;
        if text.trim().is_empty() {
            return Err(CliError::Usage(
                "stdin vazio: envie o objeto JSON ou use --params".into(),
            ));
        }
        return Ok(Some(parse_json(&text, "stdin")?));
    }

    Ok(None)
}

fn parse_json(text: &str, source: &str) -> Result<Value, CliError> {
    serde_json::from_str(text).map_err(|error| CliError::Json(source.into(), error))
}

/// Lê as linhas não vazias de um lote (`-` = stdin), com teto de tamanho.
///
/// # Errors
/// Falha de leitura ou lote acima de [`BATCH_MAX`].
pub fn read_batch(source: &str) -> Result<Vec<String>, CliError> {
    let text = if source == "-" {
        read_stdin()?
    } else {
        read_file(Path::new(source))?
    };
    let lines: Vec<String> = text
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(str::to_string)
        .collect();
    if lines.len() > BATCH_MAX {
        return Err(CliError::Usage(format!(
            "lote com {} linhas excede o teto de {BATCH_MAX}",
            lines.len()
        )));
    }
    Ok(lines)
}

/// Converte uma linha JSONL em [`Item`], rejeitando chaves desconhecidas.
///
/// # Errors
/// JSON inválido, linha que não é objeto ou chave/valor fora do contrato.
pub fn parse_item(text: &str, index: usize) -> Result<Item, CliError> {
    let value = parse_json(text, &format!("linha {}", index + 1))?;
    let object = value
        .as_object()
        .ok_or_else(|| CliError::Usage(format!("linha {}: esperado um objeto JSON", index + 1)))?;

    let mut item = Item::default();
    for (key, value) in object {
        match key.as_str() {
            "id" => item.id = Some(scalar(value, key, index)?),
            "project" => item.project = Some(scalar(value, key, index)?),
            "title" => item.title = Some(string(value, key, index)?),
            "user" => item.user = Some(scalar(value, key, index)?),
            "path" => item.path = Some(string(value, key, index)?),
            "query" => item.query = Some(string(value, key, index)?),
            "params" => item.params = Some(value.clone()),
            other => {
                return Err(CliError::Usage(format!(
                    "linha {}: chave desconhecida `{other}` (use id, project, title, user, path, query, params)",
                    index + 1
                )));
            }
        }
    }
    Ok(item)
}

fn scalar(value: &Value, key: &str, index: usize) -> Result<String, CliError> {
    match value {
        Value::String(text) => Ok(text.clone()),
        Value::Number(number) => Ok(number.to_string()),
        _ => Err(CliError::Usage(format!(
            "linha {}: `{key}` deve ser string ou número",
            index + 1
        ))),
    }
}

fn string(value: &Value, key: &str, index: usize) -> Result<String, CliError> {
    value
        .as_str()
        .map(str::to_string)
        .ok_or_else(|| CliError::Usage(format!("linha {}: `{key}` deve ser string", index + 1)))
}
