// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::fmt;
use std::time::Duration;

use crate::core::constants::{DEFAULT_MAX_RPS, DEFAULT_TIMEOUT_SECS};

/// Método de autenticação para a API do Redmine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AuthMethod {
    /// Envia a chave via header `X-Redmine-API-Key`.
    #[default]
    Header,
}

/// Configuração do cliente Redmine com valores padrão aplicados.
///
/// Construída via [`RedmineConfigBuilder`] ou diretamente com
/// `RedmineConfig { ..Default::default() }`.
///
/// # Exemplo
///
/// ```rust,ignore
/// use redmine_wrapper::RedmineConfigBuilder;
///
/// let config = RedmineConfigBuilder::default()
///     .base_url("https://redmine.example.com")
///     .token("seu-api-key")
///     .build()?;
/// ```
#[derive(Clone)]
pub struct RedmineConfig {
    /// URL base normalizada (sem barra final).
    pub base_url: String,

    /// Chave de API para autenticação via header `X-Redmine-API-Key`.
    /// `None` indica acesso anônimo a recursos públicos.
    pub token: Option<String>,

    /// Método de autenticação (padrão: `Header`).
    pub auth_method: AuthMethod,

    /// Nome de usuário para impersonação, se configurado.
    /// `None` quando não há impersonação.
    pub switch_user: Option<String>,

    /// Timeout para requisições HTTP (padrão: 30s).
    pub timeout: Duration,

    /// Máximo de requisições por segundo (padrão: 10).
    pub max_rps: u32,
}

impl fmt::Debug for RedmineConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RedmineConfig")
            .field("base_url", &self.base_url)
            .field("token", &self.token.as_deref().map(|_| "***"))
            .field("auth_method", &self.auth_method)
            .field("switch_user", &self.switch_user)
            .field("timeout", &self.timeout)
            .field("max_rps", &self.max_rps)
            .finish()
    }
}

impl Default for RedmineConfig {
    fn default() -> Self {
        Self {
            base_url: String::new(),
            token: None,
            auth_method: AuthMethod::default(),
            switch_user: None,
            timeout: Duration::from_secs(DEFAULT_TIMEOUT_SECS),
            max_rps: DEFAULT_MAX_RPS,
        }
    }
}

impl RedmineConfig {
    /// Retorna um builder para configurar e validar uma [`RedmineConfig`].
    pub fn builder() -> RedmineConfigBuilder {
        RedmineConfigBuilder::default()
    }

    /// Concatena a `base_url` com o `path` informado.
    pub(crate) fn api_url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }
}

/// Builder para [`RedmineConfig`] com validação na construção.
///
/// # Exemplo
///
/// ```rust,ignore
/// use redmine_wrapper::RedmineConfigBuilder;
///
/// let config = RedmineConfigBuilder::default()
///     .base_url("https://redmine.example.com")
///     .token("abc123")
///     .switch_user("admin")
///     .timeout_secs(60)
///     .max_rps(5)
///     .build()?;
/// ```
#[derive(Debug, Clone, Default)]
pub struct RedmineConfigBuilder {
    base_url: Option<String>,
    token: Option<String>,
    auth_method: Option<AuthMethod>,
    switch_user: Option<String>,
    timeout: Option<Duration>,
    max_rps: Option<u32>,
}

impl RedmineConfigBuilder {
    /// Define a URL base da instância Redmine (obrigatório).
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = Some(url.into());
        self
    }

    /// Define a chave de API para autenticação.
    pub fn token(mut self, token: impl Into<String>) -> Self {
        self.token = Some(token.into());
        self
    }

    /// Define o método de autenticação.
    pub fn auth_method(mut self, method: AuthMethod) -> Self {
        self.auth_method = Some(method);
        self
    }

    /// Define usuário para impersonação (header `X-Redmine-Switch-User`).
    pub fn switch_user(mut self, user: impl Into<String>) -> Self {
        self.switch_user = Some(user.into());
        self
    }

    /// Define o timeout em segundos para requisições HTTP.
    pub fn timeout_secs(mut self, secs: u64) -> Self {
        self.timeout = Some(Duration::from_secs(secs));
        self
    }

    /// Define o máximo de requisições por segundo.
    pub fn max_rps(mut self, rps: u32) -> Self {
        self.max_rps = Some(rps);
        self
    }

    /// Constrói o [`RedmineConfig`] validado.
    ///
    /// Retorna [`RedmineError::Config`] se `base_url` estiver vazia.
    pub fn build(self) -> Result<RedmineConfig, crate::core::errors::RedmineError> {
        let base_url = self.base_url.ok_or_else(|| {
            crate::core::errors::RedmineError::Config("base_url é obrigatória".into())
        })?;
        let base_url = base_url.trim_end_matches('/').to_string();
        if base_url.is_empty() {
            return Err(crate::core::errors::RedmineError::Config(
                "base_url não pode estar vazia".into(),
            ));
        }
        Ok(RedmineConfig {
            base_url,
            token: self.token,
            auth_method: self.auth_method.unwrap_or_default(),
            switch_user: self.switch_user,
            timeout: self.timeout.unwrap_or(Duration::from_secs(DEFAULT_TIMEOUT_SECS)),
            max_rps: self.max_rps.unwrap_or(DEFAULT_MAX_RPS),
        })
    }
}
