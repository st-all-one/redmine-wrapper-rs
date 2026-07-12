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

/// Configuração fornecida pelo usuário para criar um cliente Redmine.
///
/// # Exemplo com Builder
///
/// ```rust,ignore
/// use redmine_wrapper::core::config::RedmineConfigBuilder;
///
/// let config = RedmineConfigBuilder::default()
///     .base_url("https://redmine.example.com")
///     .token("seu-api-key")
///     .build()?;
/// ```
#[derive(Debug, Clone, Default)]
pub struct RedmineConfig {
    /// URL base da instância Redmine (ex: `https://redmine.example.com`).
    pub base_url: String,

    /// Chave de API do Redmine para autenticação via header `X-Redmine-API-Key`.
    /// Opcional — permite acesso anônimo a recursos públicos.
    pub token: Option<String>,

    /// Método de autenticação para a API do Redmine.
    /// Opcional — usa `Header` (único suportado atualmente) quando `None`.
    pub auth_method: Option<AuthMethod>,

    /// Nome de usuário para impersonação (requer permissão de administrador no Redmine).
    /// Opcional — ativa o header `X-Redmine-Switch-User` se informado.
    pub switch_user: Option<String>,

    /// Timeout máximo para requisições HTTP.
    /// Opcional — padrão: `DEFAULT_TIMEOUT_SECS` (30 segundos).
    pub timeout: Option<Duration>,

    /// Máximo de requisições por segundo (rate limiting).
    /// Opcional — padrão: `DEFAULT_MAX_RPS` (10 requisições/s).
    pub max_rps: Option<u32>,
}

/// Builder para [`RedmineConfig`] com validação na construção.
///
/// # Exemplo
///
/// ```rust,ignore
/// use redmine_wrapper::core::config::RedmineConfigBuilder;
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
    #[must_use]
    pub fn build(self) -> Result<RedmineConfig, crate::core::errors::RedmineError> {
        let base_url = self.base_url.ok_or_else(|| {
            crate::core::errors::RedmineError::Config("base_url é obrigatória".into())
        })?;
        if base_url.trim().is_empty() {
            return Err(crate::core::errors::RedmineError::Config(
                "base_url não pode estar vazia".into(),
            ));
        }
        Ok(RedmineConfig {
            base_url,
            token: self.token,
            auth_method: self.auth_method,
            switch_user: self.switch_user,
            timeout: self.timeout,
            max_rps: self.max_rps,
        })
    }
}

/// Configuração resolvida com valores padrão aplicados a partir de [`RedmineConfig`].
///
/// Gerada internamente por [`ResolvedConfig::from_config`] e consumida pelo
/// [`HttpClient`](crate::http::client::HttpClient).
#[derive(Clone)]
pub struct ResolvedConfig {
    /// URL base normalizada (sem barra final).
    pub base_url: String,

    /// Chave de API para autenticação via header `X-Redmine-API-Key`.
    /// `None` indica acesso anônimo a recursos públicos.
    pub token: Option<String>,

    /// Método de autenticação resolvido (padrão: `Header`).
    pub auth_method: AuthMethod,

    /// Nome de usuário para impersonação, se configurado.
    /// `None` quando não há impersonação.
    pub switch_user: Option<String>,

    /// Timeout resolvido para requisições HTTP.
    pub timeout: Duration,

    /// Máximo de requisições por segundo resolvido (rate limiting).
    pub max_rps: u32,
}

impl fmt::Debug for ResolvedConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ResolvedConfig")
            .field("base_url", &self.base_url)
            .field("token", &self.token.as_deref().map(|_| "***"))
            .field("auth_method", &self.auth_method)
            .field("switch_user", &self.switch_user)
            .field("timeout", &self.timeout)
            .field("max_rps", &self.max_rps)
            .finish()
    }
}

impl ResolvedConfig {
    /// Converte um [`RedmineConfig`] em [`ResolvedConfig`], aplicando valores padrão
    /// onde o usuário não forneceu valor.
    ///
    /// # Validações
    /// - Remove a barra final de `base_url`.
    /// - Retorna [`RedmineError::Config`](crate::core::errors::RedmineError::Config)
    ///   se `base_url` estiver vazia.
    /// - Campos `None` recebem os padrões definidos em [`constants`](crate::core::constants).
    #[must_use]
    pub(crate) fn from_config(config: &RedmineConfig) -> Result<Self, crate::core::errors::RedmineError> {
        use crate::core::errors::RedmineError;

        let base_url = config.base_url.trim_end_matches('/').to_string();
        if base_url.is_empty() {
            return Err(RedmineError::Config("base_url não pode estar vazia".into()));
        }

        Ok(Self {
            base_url,
            token: config.token.clone(),
            auth_method: config.auth_method.unwrap_or_default(),
            switch_user: config.switch_user.clone(),
            timeout: config.timeout.unwrap_or(Duration::from_secs(DEFAULT_TIMEOUT_SECS)),
            max_rps: config.max_rps.unwrap_or(DEFAULT_MAX_RPS),
        })
    }

    /// Concatena a `base_url` com o `path` informado, produzindo a URL absoluta do endpoint.
    ///
    /// # Exemplo
    /// Se `base_url` é `https://redmine.example.com` e `path` é `/issues.json`,
    /// retorna `https://redmine.example.com/issues.json`.
    pub(crate) fn api_url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }
}
