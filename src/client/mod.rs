// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod resources;

use std::ops::Deref;
use std::sync::Arc;

/// Agrupa todos os recursos disponíveis no cliente Redmine.
///
/// Cada campo fornece acesso a um domínio da API (issues, projetos,
/// usuários, etc.). O acesso aos recursos é feito diretamente via
/// `RedmineClient` graças à implementação de `Deref`.
pub use resources::ResourceGroup;

use crate::core::config::{RedmineConfig, ResolvedConfig};
use crate::core::errors::RedmineError;
use crate::http::client::HttpClient;

/// Cliente principal para a API REST do Redmine.
///
/// Cria-se uma instância via `RedmineClient::new(config)` e acessa-se
/// os recursos por meio dos campos (ex: `client.issues.list(...)`).
///
/// # Exemplo
///
/// ```rust,ignore
/// use redmine_wrapper::{RedmineClient, core::config::RedmineConfig};
///
/// let client = RedmineClient::new(RedmineConfig {
///     base_url: "https://redmine.example.com".into(),
///     token: Some("seu-api-key".into()),
///     ..Default::default()
/// })?;
///
/// let issues = client.issues.list(None, None)?;
/// ```
#[derive(Debug)]
pub struct RedmineClient {
    /// Configuração resolvida do cliente.
    pub config: ResolvedConfig,
    inner: ResourceGroup,
}

impl RedmineClient {
    /// Cria um novo cliente Redmine a partir da configuração fornecida.
    ///
    /// Este é o principal ponto de entrada da biblioteca. A configuração
    /// inclui a URL base do Redmine, a chave de API (token) e parâmetros
    /// opcionais como timeout e limite de requisições por segundo.
    ///
    /// # Exemplo
    ///
    /// ```rust,ignore
    /// use redmine_wrapper::{RedmineClient, core::config::RedmineConfig};
    ///
    /// let client = RedmineClient::new(RedmineConfig {
    ///     base_url: "https://redmine.example.com".into(),
    ///     token: Some("sua-chave-api".into()),
    ///     ..Default::default()
    /// })?;
    /// ```
    #[must_use]
    pub fn new(config: RedmineConfig) -> Result<Self, RedmineError> {
        let resolved = ResolvedConfig::from_config(&config)?;
        let http = Arc::new(HttpClient::new(resolved.clone())?);
        Ok(Self {
            config: resolved,
            inner: ResourceGroup::new(http),
        })
    }

    /// Retorna uma referência à configuração resolvida do cliente.
    pub fn config(&self) -> &ResolvedConfig {
        &self.config
    }
}

impl Deref for RedmineClient {
    type Target = ResourceGroup;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
