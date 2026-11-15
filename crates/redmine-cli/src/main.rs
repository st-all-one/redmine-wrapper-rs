//! Ponto de entrada da CLI `redmine`.

// Binário sem superfície pública: `pub` aqui é visibilidade interna.
#![allow(unreachable_pub)]

mod cli;
mod commands;
mod error;
mod output;

use std::process::ExitCode;

use clap::error::ErrorKind;

use crate::output::{emit_error, emit_output};

#[tokio::main]
async fn main() -> ExitCode {
    let parsed = match cli::parse() {
        Ok(parsed) => parsed,
        Err(error) => return handle_clap(&error),
    };

    let command = parsed.target.command_name();
    match commands::dispatch(&parsed.globals, parsed.target).await {
        Ok(output) => emit_output(&parsed.globals, &output),
        Err(error) => emit_error(&parsed.globals, &command, &error),
    }
}

fn handle_clap(error: &clap::Error) -> ExitCode {
    match error.kind() {
        ErrorKind::DisplayHelp
        | ErrorKind::DisplayVersion
        | ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand => {
            let _ignored = error.print();
            ExitCode::SUCCESS
        }
        _ => {
            let _ignored = error.print();
            ExitCode::from(2)
        }
    }
}
