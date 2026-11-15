//! Comandos de topo que não são tabelados: `search` e `api`.

use std::io::IsTerminal;

use redmine_wrapper::RedmineClient;

use crate::cli::spec;
use crate::cli::{ApiInvocation, SearchInvocation};
use crate::commands::{engine, input};
use crate::error::CliError;
use crate::output::Output;

/// Executa `search`, lendo a consulta do stdin quando necessário.
///
/// # Errors
/// Consulta ausente, stdin ambíguo ou erro da API.
pub async fn search(
    client: &RedmineClient,
    invocation: SearchInvocation,
) -> Result<Output, CliError> {
    let SearchInvocation {
        mut context,
        params,
        batch,
        dry_run,
    } = invocation;

    if batch.is_none() {
        let query_reads_stdin = context.query.as_deref() == Some("-")
            || (context.query.is_none() && !std::io::stdin().is_terminal());
        if query_reads_stdin && params.as_deref() == Some("-") {
            return Err(CliError::Usage(
                "stdin já é usado pela consulta; --params - seria ambíguo".into(),
            ));
        }
        context.query = Some(input::resolve_query(context.query.as_deref())?);
    }

    engine::run_def(
        client,
        &spec::SEARCH,
        &context,
        params.as_deref(),
        batch.as_deref(),
        dry_run,
        "search",
    )
    .await
}

/// Executa `api` (requisição JSON arbitrária).
///
/// # Errors
/// Método/caminho inválidos, JSON malformado ou erro da API.
pub async fn api(client: &RedmineClient, invocation: ApiInvocation) -> Result<Output, CliError> {
    let ApiInvocation {
        context,
        params,
        batch,
        dry_run,
    } = invocation;

    engine::run_def(
        client,
        &spec::API,
        &context,
        params.as_deref(),
        batch.as_deref(),
        dry_run,
        "api",
    )
    .await
}
