// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::sync::Arc;

use crate::core::errors::RedmineError;
use crate::http::client::HttpClient;
use crate::types::base::RedmineId;
use crate::types::relation::*;

/// Recurso para operações com relações entre issues.
#[derive(Debug)]
pub struct RelationsResource {
    http: Arc<HttpClient>,
}

impl RelationsResource {
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Retorna uma relação pelo ID.
    ///
    /// # Parâmetros
    /// - `id` — Identificador único da relação
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// let rel = client.relations.get(42).await?;
    /// ```
    pub async fn get(&self, id: RedmineId) -> Result<Relation, RedmineError> {
        let path = format!("/relations/{}.json", id);
        self.http.get_single(&path, "relation", &[], "relations.get").await
    }

    /// Lista relações de uma issue específica.
    ///
    /// # Parâmetros
    /// - `issue_id` — Identificador da issue
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// let rels = client.relations.list_by_issue(10).await?;
    /// ```
    pub async fn list_by_issue(&self, issue_id: RedmineId) -> Result<Vec<Relation>, RedmineError> {
        let path = format!("/issues/{}/relations.json", issue_id);
        let (items, _total) = self.http.get_paginated(&path, "relations", None, &[], "relations.list_by_issue").await?;
        Ok(items)
    }

    /// Cria uma relação em uma issue.
    ///
    /// # Parâmetros
    /// - `issue_id` — Identificador da issue de origem
    /// - `payload` — Dados da relação a ser criada
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// let payload = CreateRelationPayload { relation_type: "relates".into(), issue_to_id: 20 };
    /// let rel = client.relations.create_on_issue(10, &payload).await?;
    /// ```
    pub async fn create_on_issue(&self, issue_id: RedmineId, payload: &CreateRelationPayload) -> Result<Relation, RedmineError> {
        let path = format!("/issues/{}/relations.json", issue_id);
        let body = serde_json::json!({ "relation": payload });
        self.http.post_single(&path, "relation", &body, "relations.create_on_issue").await
    }

    /// Exclui uma relação.
    ///
    /// # Parâmetros
    /// - `id` — Identificador único da relação a ser excluída
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// client.relations.delete(42).await?;
    /// ```
    pub async fn delete(&self, id: RedmineId) -> Result<(), RedmineError> {
        let path = format!("/relations/{}.json", id);
        self.http.delete(&path, &[], "relations.delete").await
    }
}
