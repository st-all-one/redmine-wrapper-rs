//! Execução dos comandos: cliente, validação de token e despacho.

pub mod engine;
pub mod input;
pub mod params;
pub mod special;

use redmine_wrapper::{RedmineClient, RedmineConfigBuilder};

use crate::cli::{Globals, Target};
use crate::error::CliError;
use crate::output::Output;

/// Resolve o cliente e executa o alvo.
///
/// # Errors
/// Token ausente/ilegível, configuração inválida ou erro da operação.
pub async fn dispatch(globals: &Globals, target: Target) -> Result<Output, CliError> {
    let client = build_client(globals)?;
    match target {
        Target::Action(invocation) => engine::run_action(&client, invocation).await,
        Target::Search(invocation) => special::search(&client, invocation).await,
        Target::Api(invocation) => special::api(&client, invocation).await,
    }
}

fn build_client(globals: &Globals) -> Result<RedmineClient, CliError> {
    let token = match &globals.token_file {
        Some(path) => {
            let content = std::fs::read_to_string(path)
                .map_err(|error| CliError::Io(format!("ler {}", path.display()), error))?;
            let trimmed = content.trim();
            if trimmed.is_empty() {
                return Err(CliError::Usage(format!(
                    "arquivo de token vazio: {}",
                    path.display()
                )));
            }
            trimmed.to_string()
        }
        None => globals.token.clone().ok_or_else(|| {
            CliError::Usage("informe --token, --token-file ou REDMINE_TOKEN".into())
        })?,
    };

    let url = globals
        .url
        .clone()
        .ok_or_else(|| CliError::Usage("informe --url ou REDMINE_URL".into()))?;

    let mut builder = RedmineConfigBuilder::default()
        .base_url(url)
        .token(token)
        .timeout_secs(globals.timeout)
        .max_rps(globals.max_rps);
    if let Some(user) = &globals.switch_user {
        builder = builder.switch_user(user.clone());
    }

    let config = builder.build()?;
    RedmineClient::new(config).map_err(CliError::from)
}
