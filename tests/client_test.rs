// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use redmine_wrapper::core::config::RedmineConfig;
use redmine_wrapper::core::errors::ErrorCategory;
use redmine_wrapper::RedmineClient;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

/// Configura um mock server e retorna o server + cliente configurado.
fn setup() -> (MockServer, RedmineClient) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let server = rt.block_on(MockServer::start());

    let config = RedmineConfig {
        base_url: server.uri(),
        token: Some("test-token".into()),
        ..Default::default()
    };

    let client = RedmineClient::new(config).unwrap();
    (server, client)
}

#[test]
fn test_client_creation() {
    let config = RedmineConfig {
        base_url: "https://redmine.example.com".into(),
        token: Some("token".into()),
        ..Default::default()
    };
    let client = RedmineClient::new(config).unwrap();
    assert_eq!(client.config.base_url, "https://redmine.example.com");
}

#[test]
fn test_client_creation_fails_without_url() {
    let config = RedmineConfig {
        base_url: "".into(),
        token: Some("token".into()),
        ..Default::default()
    };
    let result = RedmineClient::new(config);
    assert!(result.is_err());
}

#[test]
fn test_client_creation_succeeds_without_token() {
    let config = RedmineConfig {
        base_url: "https://redmine.example.com".into(),
        token: None,
        ..Default::default()
    };
    let client = RedmineClient::new(config).unwrap();
    assert!(client.config.token.is_none());
}

#[test]
fn test_auth_header_sent() {
    let (server, client) = setup();

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        Mock::given(method("GET"))
            .and(path("/issues.json"))
            .and(header("X-Redmine-API-Key", "test-token"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string(r#"{"issues":[],"total_count":0,"limit":25,"offset":0}"#),
            )
            .mount(&server)
            .await;
    });

    let result = client.issues.list(None);
    assert!(result.is_ok());
}

#[test]
fn test_issues_list() {
    let (server, client) = setup();

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        Mock::given(method("GET"))
            .and(path("/issues.json"))
            .respond_with(
                ResponseTemplate::new(200).set_body_string(
                    r#"{"issues":[{"id":1,"subject":"Test Issue"}],"total_count":1,"limit":25,"offset":0}"#,
                ),
            )
            .mount(&server)
            .await;
    });

    let result = client.issues.list(None).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].id, 1);
}

#[test]
fn test_issues_get() {
    let (server, client) = setup();

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        Mock::given(method("GET"))
            .and(path("/issues/1.json"))
            .respond_with(
                ResponseTemplate::new(200).set_body_string(
                    r#"{"issue":{"id":1,"subject":"Test","project":{"id":1,"name":"P1"}}}"#,
                ),
            )
            .mount(&server)
            .await;
    });

    let result = client.issues.get(1).unwrap();
    assert_eq!(result.id, 1);
    assert_eq!(result.subject.unwrap(), "Test");
}

#[test]
fn test_404_error() {
    let (server, client) = setup();

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        Mock::given(method("GET"))
            .and(path("/issues/999.json"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;
    });

    let result = client.issues.get(999);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.category(), Some(ErrorCategory::ResourceNotFound));
}

#[test]
fn test_401_error() {
    let (server, client) = setup();

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        Mock::given(method("GET"))
            .and(path("/issues.json"))
            .respond_with(ResponseTemplate::new(401))
            .mount(&server)
            .await;
    });

    let result = client.issues.list(None);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.category(), Some(ErrorCategory::AuthenticationFailed));
}

#[test]
fn test_validation_error() {
    let (server, client) = setup();

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        Mock::given(method("POST"))
            .and(path("/issues.json"))
            .respond_with(
                ResponseTemplate::new(422)
                    .set_body_string(r#"{"errors":["Subject não pode ficar em branco"]}"#),
            )
            .mount(&server)
            .await;
    });

    let payload = redmine_wrapper::types::issue::CreateIssuePayload {
        project_id: 1,
        subject: "".into(),
        tracker_id: None,
        status_id: None,
        priority_id: None,
        description: None,
        category_id: None,
        assigned_to_id: None,
        parent_issue_id: None,
        fixed_version_id: None,
        estimated_hours: None,
        done_ratio: None,
        is_private: None,
        custom_fields: None,
        uploads: None,
        watcher_user_ids: None,
    };

    let result = client.issues.create(&payload);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.category(), Some(ErrorCategory::ValidationError));
}

#[test]
fn test_projects_list() {
    let (server, client) = setup();

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        Mock::given(method("GET"))
            .and(path("/projects.json"))
            .respond_with(
                ResponseTemplate::new(200).set_body_string(
                    r#"{"projects":[{"id":1,"name":"Project 1","identifier":"proj1"}],"total_count":1,"limit":25,"offset":0}"#,
                ),
            )
            .mount(&server)
            .await;
    });

    let result = client.projects.list().unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].name.as_deref(), Some("Project 1"));
}

#[test]
fn test_auth_method_default() {
    let config = RedmineConfig {
        base_url: "https://redmine.example.com".into(),
        token: Some("test".into()),
        ..Default::default()
    };
    let client = RedmineClient::new(config).unwrap();
    assert_eq!(client.config.auth_method, redmine_wrapper::AuthMethod::Header);
}
