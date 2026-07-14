// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Exemplo de uso do wrapper Redmine em Rust.
//!
//! Uso: REDMINE_URL=https://redmine.example.com REDMINE_TOKEN=seu-token cargo run --example demo
#![allow(clippy::unwrap_used, clippy::pedantic)]

use std::env;

use redmine_wrapper::RedmineClient;
use redmine_wrapper::RedmineConfigBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let base_url = env::var("REDMINE_URL").unwrap_or_else(|_| "https://redmine.example.com".into());
    let token = env::var("REDMINE_TOKEN").ok();

    let mut cfg_builder = RedmineConfigBuilder::default().base_url(base_url);
    if let Some(ref t) = token {
        cfg_builder = cfg_builder.token(t);
    }
    let client = RedmineClient::new(cfg_builder.build()?)?;

    println!("Cliente Redmine configurado para: {}", client.config.base_url);

    // Lista projetos
    match client.projects.list().await {
        Ok(projects) => {
            println!("Projetos encontrados: {}", projects.len());
            for p in &projects {
                println!("  - #{}: {}", p.id, p.name.as_deref().unwrap_or("sem nome"));
            }
        }
        Err(e) => eprintln!("Erro ao listar projetos: {e}"),
    }

    // Lista status de issue
    match client.issue_statuses.list().await {
        Ok(statuses) => {
            println!("Status de issue: {}", statuses.len());
            for s in &statuses {
                println!("  - #{}: {}", s.id, s.name.as_deref().unwrap_or("sem nome"));
            }
        }
        Err(e) => eprintln!("Erro ao listar status: {e}"),
    }

    // Lista trackers
    match client.trackers.list().await {
        Ok(trackers) => {
            println!("Trackers: {}", trackers.len());
            for t in &trackers {
                println!("  - #{}: {}", t.id, t.name.as_deref().unwrap_or("sem nome"));
            }
        }
        Err(e) => eprintln!("Erro ao listar trackers: {e}"),
    }

    // Conta do usuário autenticado
    match client.my_account.get().await {
        Ok(account) => {
            println!("Usuário autenticado: #{} {} {}", account.id,
                account.firstname.as_deref().unwrap_or(""),
                account.lastname.as_deref().unwrap_or(""));
        }
        Err(e) => eprintln!("Erro ao obter conta: {e}"),
    }

    // Issues atribuídas a mim
    use redmine_wrapper::types::issue::IssueFilter;
    let filter = IssueFilter {
        assigned_to_id: Some("me".into()),
        status_id: Some("open".into()),
        ..Default::default()
    };

    match client.issues.list(Some(&filter)).await {
        Ok(issues) => {
            println!("Issues abertas atribuídas a mim: {}", issues.len());
            for i in &issues {
                println!("  - #{}: {}", i.id, i.subject.as_deref().unwrap_or("sem assunto"));
            }
        }
        Err(e) => eprintln!("Erro ao listar issues: {e}"),
    }

    // Lista enumerações
    match client.enumerations.list_issue_priorities().await {
        Ok(priorities) => {
            println!("Prioridades: {}", priorities.len());
            for p in &priorities {
                println!("  - #{}: {} (padrão: {})", p.id,
                    p.name.as_deref().unwrap_or(""),
                    p.is_default.unwrap_or(false));
            }
        }
        Err(e) => eprintln!("Erro ao listar prioridades: {e}"),
    }

    // Campos personalizados
    match client.custom_fields.list().await {
        Ok(fields) => {
            println!("Campos personalizados: {}", fields.len());
            for f in &fields {
                println!("  - #{}: {}", f.id, f.name.as_deref().unwrap_or("sem nome"));
            }
        }
        Err(e) => eprintln!("Erro ao listar campos personalizados: {e}"),
    }

    println!("Demo concluída com sucesso!");
    Ok(())
}
