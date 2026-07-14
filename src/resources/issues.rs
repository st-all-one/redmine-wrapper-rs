// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::sync::Arc;

use crate::core::errors::RedmineError;
use crate::http::client::HttpClient;
use crate::utils::query::filter_to_query;
use crate::types::issue::*;
use crate::types::base::RedmineId;

/// Recurso para operações com issues do Redmine.
#[derive(Debug)]
pub struct IssuesResource {
    http: Arc<HttpClient>,
}

impl IssuesResource {
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Lista issues com filtros opcionais.
    ///
    /// # Parâmetros
    /// - `filter` — Filtros opcionais (projeto, status, assignee, etc.)
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// let issues = client.issues.list(Some(&filter)).await?;
    /// ```
    pub async fn list(&self, filter: Option<&IssueFilter>) -> Result<Vec<Issue>, RedmineError> {
        let base = filter_to_query(filter);
        let query: Vec<(&str, String)> = base.iter().map(|(k, v)| (k.as_str(), v.clone())).collect();
        self.http.get_all_paginated("/issues.json", "issues", &query, "issues.list").await
    }

    /// Retorna uma issue pelo ID.
    ///
    /// # Parâmetros
    /// - `id` — ID numérico da issue
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// let issue = client.issues.get(123).await?;
    /// ```
    pub async fn get(&self, id: RedmineId) -> Result<Issue, RedmineError> {
        let path = format!("/issues/{}.json", id);
        self.http.get_single(&path, "issue", &[], "issues.get").await
    }

    /// Retorna uma issue com associações (journals, attachments, relations, etc.).
    ///
    /// # Parâmetros
    /// - `id` — ID numérico da issue
    /// - `includes` — Lista de associações a incluir (ex: `&["journals", "attachments"]`)
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// let issue = client.issues.get_with_includes(123, &["journals", "attachments"]).await?;
    /// ```
    pub async fn get_with_includes(&self, id: RedmineId, includes: &[&str]) -> Result<Issue, RedmineError> {
        let path = format!("/issues/{}.json", id);
        let query = vec![("include", includes.join(","))];
        self.http.get_single(&path, "issue", &query, "issues.get_with_includes").await
    }

    /// Retorna os status permitidos para transição de uma issue.
    ///
    /// # Parâmetros
    /// - `id` — ID numérico da issue
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// let statuses = client.issues.get_allowed_statuses(123).await?;
    /// ```
    pub async fn get_allowed_statuses(&self, id: RedmineId) -> Result<Vec<AllowedStatus>, RedmineError> {
        let path = format!("/issues/{}.json", id);
        let query = vec![("include", "allowed_statuses".to_string())];
        let issue: Issue = self.http.get_single(&path, "issue", &query, "issues.get_allowed_statuses").await?;
        Ok(issue.allowed_statuses.unwrap_or_default())
    }

    /// Cria uma nova issue.
    ///
    /// # Parâmetros
    /// - `payload` — Dados da nova issue (projeto, tracker, assunto, etc.)
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// let payload = CreateIssuePayload { project_id: 1.into(), subject: "Erro no login".into(), ..Default::default() };
    /// let issue = client.issues.create(&payload).await?;
    /// ```
    pub async fn create(&self, payload: &CreateIssuePayload) -> Result<Issue, RedmineError> {
        self.http.post_single("/issues.json", "issue", &CreateIssueWrapper { issue: payload.clone() }, "issues.create").await
    }

    /// Atualiza uma issue existente.
    ///
    /// # Parâmetros
    /// - `id` — ID numérico da issue
    /// - `payload` — Dados parciais para atualização (assunto, status, prioridade, etc.)
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// let payload = UpdateIssuePayload { subject: Some("Novo assunto".into()), ..Default::default() };
    /// client.issues.update(123, &payload).await?;
    /// ```
    pub async fn update(&self, id: RedmineId, payload: &UpdateIssuePayload) -> Result<(), RedmineError> {
        let path = format!("/issues/{}.json", id);
        self.http.put::<serde_json::Value, _>(&path, &UpdateIssueWrapper { issue: payload.clone() }, "issues.update").await?;
        Ok(())
    }

    /// Exclui uma issue.
    ///
    /// # Parâmetros
    /// - `id` — ID numérico da issue a ser excluída
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// client.issues.delete(123).await?;
    /// ```
    pub async fn delete(&self, id: RedmineId) -> Result<(), RedmineError> {
        let path = format!("/issues/{}.json", id);
        self.http.delete(&path, &[], "issues.delete").await
    }

    /// Adiciona um watcher a uma issue.
    ///
    /// # Parâmetros
    /// - `issue_id` — ID da issue
    /// - `user_id` — ID do usuário a ser adicionado como watcher
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// client.issues.add_watcher(123, 456).await?;
    /// ```
    pub async fn add_watcher(&self, issue_id: RedmineId, user_id: RedmineId) -> Result<(), RedmineError> {
        let path = format!("/issues/{}/watchers.json", issue_id);
        let body = serde_json::json!({ "user_id": user_id });
        self.http.post::<serde_json::Value, _>(&path, &body, "issues.add_watcher").await?;
        Ok(())
    }

    /// Remove um watcher de uma issue.
    ///
    /// # Parâmetros
    /// - `issue_id` — ID da issue
    /// - `user_id` — ID do usuário a ser removido dos watchers
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// client.issues.remove_watcher(123, 456).await?;
    /// ```
    pub async fn remove_watcher(&self, issue_id: RedmineId, user_id: RedmineId) -> Result<(), RedmineError> {
        let path = format!("/issues/{}/watchers/{}.json", issue_id, user_id);
        self.http.delete(&path, &[], "issues.remove_watcher").await
    }
}
