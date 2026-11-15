//! Erros da CLI e mapeamento para códigos de saída estáveis.

use std::fmt;

use redmine_wrapper::{ErrorCategory, RedmineError};

/// Erro de execução da CLI.
#[derive(Debug)]
pub enum CliError {
    /// Uso incorreto: falta de token, `--params` inválido, flag exclusiva etc. (exit 2).
    Usage(String),
    /// Falha de I/O local (arquivo de token, lote, upload). (exit 5)
    Io(String, std::io::Error),
    /// JSON inválido fornecido pelo usuário. (exit 2)
    Json(String, serde_json::Error),
    /// Falha interna da CLI (resposta sem o campo esperado, serialização). (exit 70)
    Internal(String),
    /// Erro devolvido pela biblioteca/cliente Redmine.
    Redmine(RedmineError),
}

impl CliError {
    /// Código estável de máquina exposto no envelope `--json`.
    #[must_use]
    pub fn code(&self) -> &'static str {
        match self {
            Self::Usage(_) | Self::Json(..) => "invalid_input",
            Self::Io(..) => "io",
            Self::Internal(_) => "internal",
            Self::Redmine(error) => match error {
                RedmineError::Api { category, .. } => match category {
                    ErrorCategory::AuthenticationFailed | ErrorCategory::AuthorizationDenied => {
                        "auth"
                    }
                    ErrorCategory::ResourceNotFound => "not_found",
                    ErrorCategory::Conflict
                    | ErrorCategory::ValidationError
                    | ErrorCategory::ImpersonationFailed
                    | ErrorCategory::UploadTooLarge => "conflict",
                    ErrorCategory::RateLimited => "rate_limited",
                    ErrorCategory::Timeout | ErrorCategory::NetworkError => "network",
                    // ParseError, InternalError e categorias futuras.
                    _ => "internal",
                },
                RedmineError::RateLimited { .. } => "rate_limited",
                RedmineError::Timeout { .. } => "timeout",
                RedmineError::Http(_) => "network",
                RedmineError::Url(_) | RedmineError::Config(_) => "config",
                RedmineError::Serialization(_) => "internal",
            },
        }
    }

    /// Código de saída do processo.
    #[must_use]
    pub fn exit_code(&self) -> u8 {
        match self {
            Self::Usage(_) | Self::Json(..) => 2,
            Self::Io(..) => 5,
            Self::Internal(_) => 70,
            Self::Redmine(error) => match error {
                RedmineError::Api { category, .. } => match category {
                    ErrorCategory::AuthenticationFailed | ErrorCategory::AuthorizationDenied => 6,
                    ErrorCategory::ResourceNotFound => 3,
                    ErrorCategory::Conflict
                    | ErrorCategory::ValidationError
                    | ErrorCategory::ImpersonationFailed
                    | ErrorCategory::UploadTooLarge => 4,
                    ErrorCategory::RateLimited
                    | ErrorCategory::Timeout
                    | ErrorCategory::NetworkError => 5,
                    // ParseError, InternalError e categorias futuras.
                    _ => 70,
                },
                RedmineError::RateLimited { .. }
                | RedmineError::Timeout { .. }
                | RedmineError::Http(_) => 5,
                RedmineError::Url(_) | RedmineError::Config(_) => 2,
                RedmineError::Serialization(_) => 70,
            },
        }
    }

    /// Se a operação pode ser repetida sem mudança de estado.
    #[must_use]
    pub fn retryable(&self) -> bool {
        match self {
            Self::Redmine(error) => match error {
                RedmineError::Api { category, .. } => matches!(
                    category,
                    ErrorCategory::RateLimited
                        | ErrorCategory::Timeout
                        | ErrorCategory::NetworkError
                ),
                RedmineError::RateLimited { .. }
                | RedmineError::Timeout { .. }
                | RedmineError::Http(_) => true,
                _ => false,
            },
            _ => false,
        }
    }

    /// Código HTTP associado, quando houver.
    #[must_use]
    pub fn status(&self) -> Option<u16> {
        match self {
            Self::Redmine(RedmineError::Api { status, .. }) => Some(*status),
            _ => None,
        }
    }
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Usage(message) | Self::Internal(message) => f.write_str(message),
            Self::Io(context, error) => write!(f, "{context}: {error}"),
            Self::Json(context, error) => write!(f, "{context}: {error}"),
            Self::Redmine(error) => write!(f, "{error}"),
        }
    }
}

impl From<RedmineError> for CliError {
    fn from(error: RedmineError) -> Self {
        Self::Redmine(error)
    }
}

impl From<serde_json::Error> for CliError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json("JSON inválido".into(), error)
    }
}
