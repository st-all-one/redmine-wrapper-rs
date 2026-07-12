// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use redmine_wrapper::core::errors::{ErrorCategory, ErrorContext, RedmineError};

#[test]
fn test_error_category_from_status() {
    assert_eq!(ErrorCategory::from_status(401), Some(ErrorCategory::AuthenticationFailed));
    assert_eq!(ErrorCategory::from_status(403), Some(ErrorCategory::AuthorizationDenied));
    assert_eq!(ErrorCategory::from_status(404), Some(ErrorCategory::ResourceNotFound));
    assert_eq!(ErrorCategory::from_status(409), Some(ErrorCategory::Conflict));
    assert_eq!(ErrorCategory::from_status(412), Some(ErrorCategory::ImpersonationFailed));
    assert_eq!(ErrorCategory::from_status(413), Some(ErrorCategory::UploadTooLarge));
    assert_eq!(ErrorCategory::from_status(422), Some(ErrorCategory::ValidationError));
    assert_eq!(ErrorCategory::from_status(429), Some(ErrorCategory::RateLimited));
    assert_eq!(ErrorCategory::from_status(504), Some(ErrorCategory::Timeout));
    assert_eq!(ErrorCategory::from_status(503), Some(ErrorCategory::NetworkError));
    assert_eq!(ErrorCategory::from_status(200), None);
    assert_eq!(ErrorCategory::from_status(500), None);
}

#[test]
fn test_error_category_http_status() {
    assert_eq!(ErrorCategory::AuthenticationFailed.http_status(), 401);
    assert_eq!(ErrorCategory::RateLimited.http_status(), 429);
    assert_eq!(ErrorCategory::ParseError.http_status(), 500);
}

#[test]
fn test_error_category_description() {
    assert_eq!(ErrorCategory::AuthenticationFailed.description(), "authentication-failed");
    assert_eq!(ErrorCategory::ValidationError.description(), "validation-error");
    assert_eq!(ErrorCategory::RateLimited.description(), "rate-limited");
    assert_eq!(ErrorCategory::ResourceNotFound.description(), "resource-not-found");
}

#[test]
fn test_error_category_display() {
    let text = format!("{}", ErrorCategory::AuthenticationFailed);
    assert_eq!(text, "authentication-failed");
}

#[test]
fn test_api_error_creation() {
    let err = RedmineError::api(
        ErrorCategory::ValidationError,
        422,
        "campo obrigatório ausente",
        ErrorContext {
            operation: Some("issues.create".into()),
            http_status: Some(422),
            ..Default::default()
        },
    );

    assert!(err.to_string().contains("validation-error"));
    assert!(err.to_string().contains("campo obrigatório ausente"));

    let category = err.category();
    assert_eq!(category, Some(ErrorCategory::ValidationError));
}

#[test]
fn test_api_error_has_instance() {
    let err = RedmineError::api(
        ErrorCategory::ResourceNotFound,
        404,
        "não encontrado",
        ErrorContext::default(),
    );

    let display = err.to_string();
    assert!(display.contains("instance:"));
}

#[test]
fn test_config_error() {
    let err = RedmineError::Config("url inválida".into());
    assert_eq!(err.to_string(), "erro de configuração: url inválida");
}

#[test]
fn test_url_error() {
    let err = RedmineError::Url("bad url".into());
    assert_eq!(err.to_string(), "URL inválida: bad url");
}

#[test]
fn test_serialization_error_from() {
    let json_err = serde_json::from_str::<i32>("not a number").unwrap_err();
    let err: RedmineError = json_err.into();
    assert!(matches!(err, RedmineError::Serialization(_)));
}

#[test]
fn test_rate_limited_error() {
    let err = RedmineError::RateLimited {
        retry_after: Some(30),
        context: Box::new(ErrorContext {
            operation: Some("issues.list".into()),
            ..Default::default()
        }),
    };

    assert!(err.to_string().contains("rate limited"));
    assert_eq!(err.category(), Some(ErrorCategory::RateLimited));
}

#[test]
fn test_error_context_default() {
    let ctx = ErrorContext::default();
    assert!(ctx.operation.is_none());
    assert!(ctx.http_status.is_none());
    assert!(ctx.api_errors.is_none());
    assert!(ctx.response_body.is_none());
}
