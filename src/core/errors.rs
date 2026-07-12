// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::fmt;
use std::time::Duration;

use uuid::Uuid;

/// Categoria semântica do erro, mapeada a partir do código HTTP.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ErrorCategory {
    /// 401 — Chave de API ausente ou inválida.
    AuthenticationFailed,
    /// 403 — Acesso negado ao recurso.
    AuthorizationDenied,
    /// 404 — Recurso não encontrado.
    ResourceNotFound,
    /// 422 — Erro de validação nos dados enviados.
    ValidationError,
    /// 409 — Conflito (ex: versão de wiki).
    Conflict,
    /// 429 — Taxa de requisições excedida.
    RateLimited,
    /// 412 — Impersonação inválida.
    ImpersonationFailed,
    /// 413 — Upload excede tamanho máximo.
    UploadTooLarge,
    /// 504 — Tempo limite da requisição excedido.
    Timeout,
    /// 503 — Erro de rede ou serviço indisponível.
    NetworkError,
    /// 500 — Resposta JSON inválida.
    ParseError,
    /// 500 — Erro interno não categorizado.
    InternalError,
}

impl ErrorCategory {
    /// Tenta converter um código HTTP numa [`ErrorCategory`].
    ///
    /// Retorna `None` para códigos sem mapeamento direto (ex: 200, 302, 500).
    pub fn from_status(status: u16) -> Option<Self> {
        match status {
            401 => Some(Self::AuthenticationFailed),
            403 => Some(Self::AuthorizationDenied),
            404 => Some(Self::ResourceNotFound),
            409 => Some(Self::Conflict),
            412 => Some(Self::ImpersonationFailed),
            413 => Some(Self::UploadTooLarge),
            422 => Some(Self::ValidationError),
            429 => Some(Self::RateLimited),
            504 => Some(Self::Timeout),
            503 => Some(Self::NetworkError),
            _ => None,
        }
    }

    /// Retorna o código HTTP associado à categoria.
    pub fn http_status(&self) -> u16 {
        match self {
            Self::AuthenticationFailed => 401,
            Self::AuthorizationDenied => 403,
            Self::ResourceNotFound => 404,
            Self::Conflict => 409,
            Self::ImpersonationFailed => 412,
            Self::UploadTooLarge => 413,
            Self::ValidationError => 422,
            Self::RateLimited => 429,
            Self::Timeout => 504,
            Self::NetworkError => 503,
            Self::ParseError | Self::InternalError => 500,
        }
    }

    /// Descrição curta da categoria em kebab-case.
    pub fn description(&self) -> &'static str {
        match self {
            Self::AuthenticationFailed => "authentication-failed",
            Self::AuthorizationDenied => "authorization-denied",
            Self::ResourceNotFound => "resource-not-found",
            Self::Conflict => "conflict",
            Self::ImpersonationFailed => "impersonation-failed",
            Self::UploadTooLarge => "upload-too-large",
            Self::ValidationError => "validation-error",
            Self::RateLimited => "rate-limited",
            Self::Timeout => "timeout",
            Self::NetworkError => "network-error",
            Self::ParseError => "parse-error",
            Self::InternalError => "internal-error",
        }
    }
}

impl fmt::Display for ErrorCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.description())
    }
}

/// Contexto adicional associado a um erro.
#[derive(Debug, Clone, Default)]
pub struct ErrorContext {
    /// Nome da operação que gerou o erro (ex: `"issues.list"`).
    pub operation: Option<String>,

    /// Código HTTP retornado (se aplicável).
    pub http_status: Option<u16>,

    /// Lista de mensagens de erro da API.
    pub api_errors: Option<Vec<String>>,

    /// Corpo da resposta bruta.
    pub response_body: Option<String>,

    /// Parâmetros adicionais.
    pub extra: std::collections::HashMap<String, String>,
}

/// Erro principal da biblioteca.
#[derive(Debug, thiserror::Error)]
pub enum RedmineError {
    /// Erro retornado pela API do Redmine.
    #[error("[{category}] {detail} (instance: {instance})")]
    Api {
        /// Categoria do erro.
        category: ErrorCategory,
        /// Código HTTP.
        status: u16,
        /// Mensagem descritiva.
        detail: String,
        /// UUID v7 para correlação.
        instance: String,
        /// Contexto adicional.
        context: Box<ErrorContext>,
    },

    /// Erro de transporte HTTP (reqwest).
    #[error("erro HTTP: {0}")]
    Http(#[from] reqwest::Error),

    /// Taxa de requisições excedida.
    #[error("rate limited (retry after: {retry_after:?})")]
    RateLimited {
        /// Tempo sugerido para aguardar (segundos).
        retry_after: Option<u64>,
        /// Contexto adicional.
        context: Box<ErrorContext>,
    },

    /// Tempo limite excedido.
    #[error("timeout após {duration:?}")]
    Timeout {
        /// Duração do timeout configurado.
        duration: Duration,
        /// Contexto adicional.
        context: Box<ErrorContext>,
    },

    /// URL inválida.
    #[error("URL inválida: {0}")]
    Url(String),

    /// Erro de serialização/desserialização JSON.
    #[error("erro JSON: {0}")]
    Serialization(#[from] serde_json::Error),

    /// Erro de configuração.
    #[error("erro de configuração: {0}")]
    Config(String),
}

impl RedmineError {
    /// Constrói um [`RedmineError::Api`] com UUID v7 gerado automaticamente para correlação.
    pub fn api(
        category: ErrorCategory,
        status: u16,
        detail: impl Into<String>,
        context: ErrorContext,
    ) -> Self {
        Self::Api {
            category,
            status,
            detail: detail.into(),
            instance: Uuid::now_v7().to_string(),
            context: Box::new(context),
        }
    }

    /// Extrai a [`ErrorCategory`] do erro, quando aplicável.
    ///
    /// Retorna `Some` para as variantes `Api`, `RateLimited`, `Timeout` e
    /// para `Http` quando `is_timeout()` é `true`.
    /// Retorna `None` para as demais variantes.
    pub fn category(&self) -> Option<ErrorCategory> {
        match self {
            Self::Api { category, .. } => Some(*category),
            Self::RateLimited { .. } => Some(ErrorCategory::RateLimited),
            Self::Timeout { .. } => Some(ErrorCategory::Timeout),
            Self::Http(e) if e.is_timeout() => Some(ErrorCategory::Timeout),
            _ => None,
        }
    }
}
