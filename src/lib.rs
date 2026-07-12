// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! # redmine-wrapper-rs
//!
//! Wrapper Rust tipado para a API REST do Redmine.
//!
//! Fornece acesso completo a todos os recursos da API:
//! issues, projetos, usuários, apontamentos de horas, wiki, anexos, etc.
//!
//! ## Exemplo rápido
//!
//! ```rust,ignore
//! use redmine_wrapper::RedmineClient;
//! use redmine_wrapper::core::config::RedmineConfig;
//!
//! let client = RedmineClient::new(RedmineConfig {
//!     base_url: "https://redmine.example.com".into(),
//!     token: Some("sua-chave-api".into()),
//!     ..Default::default()
//! })?;
//!
//! let issues = client.issues.list(None, None)?;
//! println!("Total de issues: {}", issues.len());
//! ```

pub mod client;
pub mod core;
pub mod http;
pub mod resources;
pub mod types;
pub mod utils;

pub use client::RedmineClient;
pub use core::config::{AuthMethod, RedmineConfig, RedmineConfigBuilder, ResolvedConfig};
pub use core::errors::{ErrorCategory, ErrorContext, RedmineError};
