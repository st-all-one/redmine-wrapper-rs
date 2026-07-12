// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::sync::Arc;

use crate::core::errors::RedmineError;
use crate::http::client::HttpClient;
use crate::types::base::RedmineId;
use crate::types::issue_category::*;

/// Recurso para operações com categorias de issue.
#[derive(Debug)]
pub struct IssueCategoriesResource {
    http: Arc<HttpClient>,
}

impl IssueCategoriesResource {
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Lista categorias de um projeto.
    ///
    /// # Parâmetros
    /// - `project_id` — Identificador do projeto
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// let categories = client.issue_categories.list_by_project(1)?;
    /// ```
    #[must_use]
    pub fn list_by_project(&self, project_id: RedmineId) -> Result<Vec<IssueCategory>, RedmineError> {
        let path = format!("/projects/{}/issue_categories.json", project_id);
        let (items, _total) = self.http.get_paginated(&path, "issue_categories", None, &[], "issue_categories.list_by_project")?;
        Ok(items)
    }

    /// Retorna uma categoria pelo ID.
    ///
    /// # Parâmetros
    /// - `id` — Identificador único da categoria
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// let cat = client.issue_categories.get(7)?;
    /// ```
    #[must_use]
    pub fn get(&self, id: RedmineId) -> Result<IssueCategory, RedmineError> {
        let path = format!("/issue_categories/{}.json", id);
        self.http.get_single(&path, "issue_category", &[], "issue_categories.get")
    }

    /// Cria uma categoria em um projeto.
    ///
    /// # Parâmetros
    /// - `project_id` — Identificador do projeto
    /// - `payload` — Dados da categoria a ser criada
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// let payload = CreateIssueCategoryPayload { name: "Bug".into() };
    /// let cat = client.issue_categories.create(1, &payload)?;
    /// ```
    #[must_use]
    pub fn create(&self, project_id: RedmineId, payload: &CreateIssueCategoryPayload) -> Result<IssueCategory, RedmineError> {
        let path = format!("/projects/{}/issue_categories.json", project_id);
        let body = serde_json::json!({ "issue_category": payload });
        self.http.post_single(&path, "issue_category", &body, "issue_categories.create")
    }

    /// Atualiza uma categoria.
    ///
    /// # Parâmetros
    /// - `id` — Identificador único da categoria
    /// - `payload` — Dados atualizados da categoria
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// let payload = UpdateIssueCategoryPayload { name: Some("Melhoria".into()) };
    /// client.issue_categories.update(7, &payload)?;
    /// ```
    #[must_use]
    pub fn update(&self, id: RedmineId, payload: &UpdateIssueCategoryPayload) -> Result<(), RedmineError> {
        let path = format!("/issue_categories/{}.json", id);
        let body = serde_json::json!({ "issue_category": payload });
        self.http.put::<serde_json::Value, _>(&path, &body, "issue_categories.update")?;
        Ok(())
    }

    /// Exclui uma categoria, opcionalmente reassignando issues para outra categoria.
    ///
    /// # Parâmetros
    /// - `id` — Identificador único da categoria a ser excluída
    /// - `reassign_to_id` — ID opcional de outra categoria para reassinar as issues vinculadas
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// // Excluir sem reassinar
    /// client.issue_categories.delete(7, None)?;
    /// // Excluir e reassinar issues para a categoria 10
    /// client.issue_categories.delete(7, Some(10))?;
    /// ```
    #[must_use]
    pub fn delete(&self, id: RedmineId, reassign_to_id: Option<RedmineId>) -> Result<(), RedmineError> {
        let path = format!("/issue_categories/{}.json", id);
        let mut query = Vec::new();
        if let Some(v) = reassign_to_id {
            query.push(("reassign_to_id", v.to_string()));
        }
        self.http.delete(&path, &query, "issue_categories.delete")
    }
}
