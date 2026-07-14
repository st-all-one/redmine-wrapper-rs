// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! # redmine-wrapper-rs
//!
//! Wrapper Rust tipado para a API REST do Redmine.
//!
//! Fornece acesso completo a todos os recursos da API:
//! issues, projetos, usuarios, apontamentos de horas, wiki, anexos, etc.
//!
//! ## Exemplo rapido
//!
//! ```rust,ignore
//! use redmine_wrapper::RedmineClient;
//! use redmine_wrapper::RedmineConfigBuilder;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = RedmineClient::new(
//!         RedmineConfigBuilder::default()
//!             .base_url("https://redmine.example.com")
//!             .token("sua-chave-api")
//!             .build()?,
//!     )?;
//!
//!     let issues = client.issues.list(None).await?;
//!     println!("Total de issues: {}", issues.len());
//!     Ok(())
//! }
//! ```

/// Modulo cliente: [`RedmineClient`] e inicializacao.
pub mod client;
/// Modulo core: configuracao, constantes e tipos de erro.
pub mod core;
/// Modulo HTTP: cliente assincrono, paginacao e rate limiter.
pub mod http;
/// Modulo resources: 22 recursos da API (issues, projects, etc.).
pub mod resources;
/// Modulo types: tipos serde para todos os recursos da API.
pub mod types;
/// Modulo utils: utilitarios (filtros, query helpers).
pub mod utils;

pub use client::RedmineClient;
pub use core::config::{AuthMethod, RedmineConfig, RedmineConfigBuilder};
pub use core::errors::{ErrorCategory, ErrorContext, RedmineError};
