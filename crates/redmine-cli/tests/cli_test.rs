//! Testes de integração da CLI contra um servidor HTTP falso.

use std::process::Stdio;

use serde_json::{Value, json};
use tokio::io::AsyncWriteExt;
use tokio::process::Command;
use wiremock::matchers::{header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

struct Run {
    code: i32,
    stdout: String,
    stderr: String,
}

async fn run_with(
    url: Option<&str>,
    token: Option<&str>,
    args: &[&str],
    stdin: Option<&str>,
) -> Run {
    let mut command = Command::new(env!("CARGO_BIN_EXE_redmine"));
    if let Some(url) = url {
        command.arg("--url").arg(url);
    }
    if let Some(token) = token {
        command.arg("--token").arg(token);
    }
    command
        .args(args)
        .env_remove("REDMINE_URL")
        .env_remove("REDMINE_TOKEN")
        .env_remove("REDMINE_TIMEOUT")
        .env_remove("REDMINE_MAX_RPS")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child = command.spawn().expect("spawn da CLI");
    if let Some(input) = stdin {
        let mut pipe = child.stdin.take().expect("stdin aberto");
        pipe.write_all(input.as_bytes())
            .await
            .expect("escrever stdin");
        drop(pipe);
    } else {
        drop(child.stdin.take());
    }

    let output = tokio::time::timeout(std::time::Duration::from_secs(15), child.wait_with_output())
        .await
        .expect("CLI não terminou no prazo")
        .expect("falha ao aguardar a CLI");

    Run {
        code: output.status.code().unwrap_or(-1),
        stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
    }
}

async fn run(server: &MockServer, args: &[&str], stdin: Option<&str>) -> Run {
    run_with(Some(&server.uri()), Some("test-token"), args, stdin).await
}

#[tokio::test]
async fn issues_list_sends_token_and_params() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/issues.json"))
        .and(header("X-Redmine-API-Key", "test-token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "issues": [{"id": 1, "subject": "Bug"}],
            "total_count": 1,
        })))
        .mount(&server)
        .await;

    let output = run(
        &server,
        &[
            "issues",
            "list",
            "--params",
            r#"{"status_id":"open","set_filter":1}"#,
        ],
        None,
    )
    .await;

    assert_eq!(output.code, 0, "stderr: {}", output.stderr);
    let data: Value = serde_json::from_str(&output.stdout).expect("JSON de saída");
    assert_eq!(data, json!([{"id": 1, "subject": "Bug"}]));

    let requests = server.received_requests().await.expect("requisições");
    let url = requests[0].url.as_str();
    assert!(url.contains("status_id=open"), "url: {url}");
    assert!(url.contains("set_filter=1"), "url: {url}");
}

#[tokio::test]
async fn list_honors_explicit_limit_with_single_request() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/issues.json"))
        .and(query_param("limit", "5"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "issues": [{"id": 1}, {"id": 2}],
            "total_count": 100,
        })))
        .mount(&server)
        .await;

    let output = run(
        &server,
        &["issues", "list", "--params", r#"{"limit":5}"#],
        None,
    )
    .await;

    assert_eq!(output.code, 0, "stderr: {}", output.stderr);
    let data: Value = serde_json::from_str(&output.stdout).expect("JSON de saída");
    assert_eq!(data, json!([{"id": 1}, {"id": 2}]));

    let requests = server.received_requests().await.expect("requisições");
    assert_eq!(requests.len(), 1, "limit explícito deve evitar paginação");
    let url = requests[0].url.as_str();
    assert!(url.contains("limit=5"), "url: {url}");
    assert!(!url.contains("offset="), "url: {url}");
}

#[tokio::test]
async fn issue_get_extracts_wrapper_and_emits_envelope() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/issues/42.json"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "issue": {"id": 42, "subject": "Falha"},
        })))
        .mount(&server)
        .await;

    let output = run(&server, &["--json", "issues", "get", "--id", "42"], None).await;

    assert_eq!(output.code, 0, "stderr: {}", output.stderr);
    let envelope: Value = serde_json::from_str(&output.stdout).expect("envelope JSON");
    assert_eq!(envelope["success"], json!(true));
    assert_eq!(envelope["command"], json!("issues.get"));
    assert_eq!(envelope["data"], json!({"id": 42, "subject": "Falha"}));
}

#[tokio::test]
async fn issue_create_reads_body_from_stdin() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/issues.json"))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({
            "issue": {"id": 7, "subject": "Nova"},
        })))
        .mount(&server)
        .await;

    let output = run(
        &server,
        &["issues", "create"],
        Some(r#"{"subject":"Nova","project_id":1}"#),
    )
    .await;

    assert_eq!(output.code, 0, "stderr: {}", output.stderr);
    let data: Value = serde_json::from_str(&output.stdout).expect("JSON de saída");
    assert_eq!(data, json!({"id": 7, "subject": "Nova"}));

    let requests = server.received_requests().await.expect("requisições");
    let body: Value = serde_json::from_slice(&requests[0].body).expect("corpo JSON");
    assert_eq!(body, json!({"issue": {"subject": "Nova", "project_id": 1}}));
}

#[tokio::test]
async fn search_uses_positional_query() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/search.json"))
        .and(query_param("q", "bug crítico"))
        .and(query_param("scope", "all"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "results": [{"id": 1, "title": "Bug"}],
            "total_count": 1,
        })))
        .mount(&server)
        .await;

    let output = run(
        &server,
        &["search", "bug crítico", "--params", r#"{"scope":"all"}"#],
        None,
    )
    .await;

    assert_eq!(output.code, 0, "stderr: {}", output.stderr);
    let data: Value = serde_json::from_str(&output.stdout).expect("JSON de saída");
    assert_eq!(data, json!([{"id": 1, "title": "Bug"}]));
}

#[tokio::test]
async fn batch_reports_each_item_and_fails_best_effort() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/issues/1.json"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "issue": {"id": 1},
        })))
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/issues/2.json"))
        .respond_with(ResponseTemplate::new(404).set_body_json(json!({
            "error": "não encontrada",
        })))
        .mount(&server)
        .await;

    let output = run(
        &server,
        &["issues", "get", "--batch", "-"],
        Some("{\"id\":1}\n{\"id\":2}\n"),
    )
    .await;

    assert_eq!(output.code, 1, "stderr: {}", output.stderr);
    let lines: Vec<Value> = output
        .stdout
        .lines()
        .map(|line| serde_json::from_str(line).expect("linha NDJSON"))
        .collect();
    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0]["index"], json!(0));
    assert_eq!(lines[0]["ok"], json!(true));
    assert_eq!(lines[0]["data"], json!({"id": 1}));
    assert_eq!(lines[1]["index"], json!(1));
    assert_eq!(lines[1]["ok"], json!(false));
    assert_eq!(lines[1]["error"]["code"], json!("not_found"));
}

#[tokio::test]
async fn batch_rejects_unknown_keys_without_sending() {
    let server = MockServer::start().await;
    let output = run(
        &server,
        &["issues", "get", "--batch", "-"],
        Some("{\"id\":1,\"bogus\":true}\n"),
    )
    .await;

    assert_eq!(output.code, 1, "stderr: {}", output.stderr);
    let line: Value = serde_json::from_str(output.stdout.trim()).expect("linha NDJSON");
    assert_eq!(line["ok"], json!(false));
    assert_eq!(line["error"]["code"], json!("invalid_input"));
    assert!(
        server
            .received_requests()
            .await
            .unwrap_or_default()
            .is_empty(),
        "nada deveria ser enviado"
    );
}

#[tokio::test]
async fn wiki_path_is_percent_encoded() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/projects/my%20proj%2Fx/wiki/Guia%20de%20Uso.json"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "wiki_page": {"title": "Guia de Uso"},
        })))
        .mount(&server)
        .await;

    let output = run(
        &server,
        &[
            "wiki",
            "get",
            "--project",
            "my proj/x",
            "--title",
            "Guia de Uso",
        ],
        None,
    )
    .await;

    assert_eq!(output.code, 0, "stderr: {}", output.stderr);
    let data: Value = serde_json::from_str(&output.stdout).expect("JSON de saída");
    assert_eq!(data, json!({"title": "Guia de Uso"}));
}

#[tokio::test]
async fn api_executes_arbitrary_get() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/custom.json"))
        .and(query_param("foo", "bar"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"ok": true})))
        .mount(&server)
        .await;

    let output = run(
        &server,
        &[
            "api",
            "--method",
            "GET",
            "--path",
            "/custom.json",
            "--params",
            r#"{"foo":"bar"}"#,
        ],
        None,
    )
    .await;

    assert_eq!(output.code, 0, "stderr: {}", output.stderr);
    let data: Value = serde_json::from_str(&output.stdout).expect("JSON de saída");
    assert_eq!(data, json!({"ok": true}));
}

#[tokio::test]
async fn api_post_reads_raw_body_from_stdin() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/custom.json"))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({"created": true})))
        .mount(&server)
        .await;

    let output = run(
        &server,
        &[
            "api",
            "--method",
            "POST",
            "--path",
            "/custom.json",
            "--params",
            "-",
        ],
        Some(r#"{"subject":"Olá","done":true}"#),
    )
    .await;

    assert_eq!(output.code, 0, "stderr: {}", output.stderr);
    let data: Value = serde_json::from_str(&output.stdout).expect("JSON de saída");
    assert_eq!(data, json!({"created": true}));

    let requests = server.received_requests().await.expect("requisições");
    let body: Value = serde_json::from_slice(&requests[0].body).expect("corpo JSON");
    assert_eq!(body, json!({"subject": "Olá", "done": true}));
}

#[tokio::test]
async fn delete_with_204_prints_ok() {
    let server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path("/issues/9.json"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&server)
        .await;

    let output = run(&server, &["issues", "delete", "--id", "9"], None).await;

    assert_eq!(output.code, 0, "stderr: {}", output.stderr);
    assert_eq!(output.stdout.trim(), "ok");
}

#[tokio::test]
async fn dry_run_sends_nothing() {
    let server = MockServer::start().await;
    let output = run(&server, &["issues", "list", "--dry-run"], None).await;

    assert_eq!(output.code, 0, "stderr: {}", output.stderr);
    let data: Value = serde_json::from_str(&output.stdout).expect("JSON de saída");
    assert_eq!(data["dry_run"], json!(true));
    assert_eq!(data["request"]["method"], json!("GET"));
    assert!(
        server
            .received_requests()
            .await
            .unwrap_or_default()
            .is_empty(),
        "dry-run não deve enviar requisições"
    );
}

#[tokio::test]
async fn missing_token_is_usage_error() {
    let output = run_with(Some("http://127.0.0.1:1"), None, &["issues", "list"], None).await;

    assert_eq!(output.code, 2);
    assert!(output.stderr.contains("token"), "stderr: {}", output.stderr);
}

#[tokio::test]
async fn attachment_upload_encodes_filename_and_returns_token() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/uploads.json"))
        .and(query_param("filename", "nota fiscal.txt"))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({
            "upload": {"token": "tok-123"},
        })))
        .mount(&server)
        .await;

    let file = std::env::temp_dir().join(format!("redmine-cli-upload-{}.txt", std::process::id()));
    std::fs::write(&file, b"conteudo").expect("arquivo temporário");

    let output = run(
        &server,
        &[
            "attachments",
            "upload",
            "--file",
            file.to_str().expect("caminho UTF-8"),
            "--filename",
            "nota fiscal.txt",
        ],
        None,
    )
    .await;

    let _ignored = std::fs::remove_file(&file);

    assert_eq!(output.code, 0, "stderr: {}", output.stderr);
    let data: Value = serde_json::from_str(&output.stdout).expect("JSON de saída");
    assert_eq!(data, json!({"token": "tok-123"}));
}

#[tokio::test]
async fn token_file_precedes_token_flag() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/issues.json"))
        .and(header("X-Redmine-API-Key", "file-token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "issues": [],
            "total_count": 0,
        })))
        .mount(&server)
        .await;

    let token_path =
        std::env::temp_dir().join(format!("redmine-cli-token-{}.txt", std::process::id()));
    std::fs::write(&token_path, "file-token\n").expect("token temporário");
    let token_arg = token_path.to_str().expect("caminho UTF-8").to_string();

    let output = run_with(
        Some(&server.uri()),
        Some("flag-token"),
        &["--token-file", token_arg.as_str(), "issues", "list"],
        None,
    )
    .await;

    let _ignored = std::fs::remove_file(&token_path);

    assert_eq!(output.code, 0, "stderr: {}", output.stderr);
    let requests = server.received_requests().await.expect("requisições");
    let header = requests[0]
        .headers
        .get("X-Redmine-API-Key")
        .expect("header de token");
    assert_eq!(header, "file-token");
}

#[tokio::test]
async fn batch_search_and_api_use_line_metadata() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/search.json"))
        .and(query_param("q", "erro"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "results": [{"id": 5, "title": "Erro"}],
            "total_count": 1,
        })))
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/custom.json"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"ok": true})))
        .mount(&server)
        .await;

    let search = run(
        &server,
        &["search", "--batch", "-"],
        Some("{\"query\":\"erro\"}\n"),
    )
    .await;
    assert_eq!(search.code, 0, "stderr: {}", search.stderr);
    let line: Value = serde_json::from_str(search.stdout.trim()).expect("linha NDJSON");
    assert_eq!(line["ok"], json!(true));
    assert_eq!(line["data"], json!([{"id": 5, "title": "Erro"}]));

    let api = run(
        &server,
        &["api", "--method", "GET", "--batch", "-"],
        Some("{\"path\":\"/custom.json\"}\n"),
    )
    .await;
    assert_eq!(api.code, 0, "stderr: {}", api.stderr);
    let line: Value = serde_json::from_str(api.stdout.trim()).expect("linha NDJSON");
    assert_eq!(line["ok"], json!(true));
    assert_eq!(line["data"], json!({"ok": true}));
}

#[tokio::test]
async fn file_upload_chains_token_and_association() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/uploads.json"))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({
            "upload": {"token": "tok-xyz"},
        })))
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path("/projects/demo/files.json"))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({
            "file": {"id": 3, "filename": "relatorio.txt"},
        })))
        .mount(&server)
        .await;

    let file = std::env::temp_dir().join(format!("redmine-cli-file-{}.txt", std::process::id()));
    std::fs::write(&file, b"relatorio").expect("arquivo temporário");

    let output = run(
        &server,
        &[
            "files",
            "upload",
            "--project",
            "demo",
            "--file",
            file.to_str().expect("caminho UTF-8"),
            "--filename",
            "relatorio.txt",
            "--description",
            "Entrega",
        ],
        None,
    )
    .await;

    let _ignored = std::fs::remove_file(&file);

    assert_eq!(output.code, 0, "stderr: {}", output.stderr);
    let data: Value = serde_json::from_str(&output.stdout).expect("JSON de saída");
    assert_eq!(data, json!({"id": 3, "filename": "relatorio.txt"}));

    let requests = server.received_requests().await.expect("requisições");
    let association = requests
        .iter()
        .find(|request| request.url.path() == "/projects/demo/files.json")
        .expect("POST de associação");
    let body: Value = serde_json::from_slice(&association.body).expect("corpo JSON");
    assert_eq!(
        body,
        json!({
            "file": {
                "token": "tok-xyz",
                "filename": "relatorio.txt",
                "description": "Entrega",
            },
        })
    );
}
