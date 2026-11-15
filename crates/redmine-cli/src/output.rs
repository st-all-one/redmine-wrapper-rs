//! Contrato de saída: stdout = dados, stderr = diagnóstico, EPIPE = sucesso.

use std::io::Write;
use std::process::ExitCode;

use serde_json::{Value, json};

use crate::cli::Globals;
use crate::error::CliError;

/// Conteúdo de uma execução bem-sucedida.
pub enum Payload {
    /// Um único valor (objeto, array ou `null`).
    Single(Value),
    /// Linhas NDJSON de um lote (`--batch`); cada item já é um objeto de resultado.
    Lines(Vec<Value>),
}

/// Resultado de um comando, pronto para emissão.
pub struct Output {
    /// Nome canônico (`issues.get`, `search`, ...).
    pub command: String,
    /// Dados a emitir.
    pub payload: Payload,
    /// Avisos não fatais.
    pub warnings: Vec<String>,
    /// `true` se algum item do lote falhou (define exit 1).
    pub failed: bool,
}

impl Output {
    /// Saída de um valor único.
    #[must_use]
    pub fn single(command: impl Into<String>, data: Value) -> Self {
        Self {
            command: command.into(),
            payload: Payload::Single(data),
            warnings: Vec::new(),
            failed: false,
        }
    }

    /// Saída de um lote (NDJSON), com o estado agregado.
    #[must_use]
    pub fn lines(command: impl Into<String>, lines: Vec<Value>, failed: bool) -> Self {
        Self {
            command: command.into(),
            payload: Payload::Lines(lines),
            warnings: Vec::new(),
            failed,
        }
    }
}

/// Escreve em stdout; `EPIPE` é sucesso (pipe fechado, ex.: `| head`).
pub fn emit_stdout(text: &str) -> ExitCode {
    let mut stdout = std::io::stdout().lock();
    match stdout
        .write_all(text.as_bytes())
        .and_then(|()| stdout.flush())
    {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) if error.kind() == std::io::ErrorKind::BrokenPipe => ExitCode::SUCCESS,
        Err(_) => ExitCode::from(5),
    }
}

/// Escreve em stderr; falhas de log nunca quebram o comando.
pub fn emit_stderr(text: &str) {
    let mut stderr = std::io::stderr().lock();
    let _ignored = stderr.write_all(text.as_bytes());
    let _ignored = stderr.flush();
}

/// Emite o resultado segundo `--json`/modo texto.
pub fn emit_output(globals: &Globals, output: &Output) -> ExitCode {
    match &output.payload {
        Payload::Lines(lines) => {
            let mut text = String::new();
            for line in lines {
                text.push_str(&compact(line));
                text.push('\n');
            }
            let code = emit_stdout(&text);
            if output.failed {
                ExitCode::from(1)
            } else {
                code
            }
        }
        Payload::Single(data) => {
            let text = if globals.json {
                envelope_success(&output.command, data, &output.warnings)
            } else {
                for warning in &output.warnings {
                    emit_stderr(&format!("aviso: {warning}\n"));
                }
                render_data(data)
            };
            emit_stdout(&format!("{text}\n"))
        }
    }
}

/// Emite um erro (stderr em modo texto; envelope em stdout com `--json`).
pub fn emit_error(globals: &Globals, command: &str, error: &CliError) -> ExitCode {
    if globals.json {
        let text = compact(&json!({
            "success": false,
            "command": command,
            "error": error_body(error),
        }));
        let _ignored = emit_stdout(&format!("{text}\n"));
    } else {
        emit_stderr(&format!("erro: {error}\n"));
    }
    ExitCode::from(error.exit_code())
}

/// Corpo estruturado de erro usado no envelope e nos itens de lote.
#[must_use]
pub fn error_body(error: &CliError) -> Value {
    let mut body = json!({
        "code": error.code(),
        "message": error.to_string(),
        "retryable": error.retryable(),
    });
    if let Some(status) = error.status() {
        body["status"] = json!(status);
    }
    body
}

fn envelope_success(command: &str, data: &Value, warnings: &[String]) -> String {
    let mut envelope = json!({
        "success": true,
        "command": command,
        "data": data,
    });
    if !warnings.is_empty() {
        envelope["warnings"] = json!(warnings);
    }
    compact(&envelope)
}

fn render_data(data: &Value) -> String {
    if data.is_null() {
        return "ok".to_string();
    }
    serde_json::to_string_pretty(data).unwrap_or_else(|_| "null".to_string())
}

fn compact(value: &Value) -> String {
    serde_json::to_string(value).unwrap_or_else(|_| "{}".to_string())
}
