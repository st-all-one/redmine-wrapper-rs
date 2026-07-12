// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::sync::Arc;

use crate::core::errors::RedmineError;
use crate::http::client::HttpClient;
use crate::types::base::RedmineId;
use crate::types::version::*;

/// Recurso para operações com versões do Redmine.
#[derive(Debug)]
pub struct VersionsResource {
    http: Arc<HttpClient>,
}

impl VersionsResource {
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Retorna uma versão pelo ID.
    ///
    /// # Parâmetros
    /// - `id` — Identificador único da versão
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// let version = client.versions.get(3)?;
    /// ```
    #[must_use]
    pub fn get(&self, id: RedmineId) -> Result<Version, RedmineError> {
        let path = format!("/versions/{}.json", id);
        self.http.get_single(&path, "version", &[], "versions.get")
    }

    /// Lista versões de um projeto.
    ///
    /// # Parâmetros
    /// - `project_id` — Identificador do projeto
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// let versions = client.versions.list_by_project(1)?;
    /// ```
    #[must_use]
    pub fn list_by_project(&self, project_id: RedmineId) -> Result<Vec<Version>, RedmineError> {
        let path = format!("/projects/{}/versions.json", project_id);
        let (items, _total) = self.http.get_paginated(&path, "versions", None, &[], "versions.list_by_project")?;
        Ok(items)
    }

    /// Cria uma versão em um projeto.
    ///
    /// # Parâmetros
    /// - `project_id` — Identificador do projeto
    /// - `payload` — Dados da versão a ser criada
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// let payload = CreateVersionPayload { name: "v2.0".into(), ..Default::default() };
    /// let version = client.versions.create_on_project(1, &payload)?;
    /// ```
    #[must_use]
    pub fn create_on_project(&self, project_id: RedmineId, payload: &CreateVersionPayload) -> Result<Version, RedmineError> {
        let path = format!("/projects/{}/versions.json", project_id);
        let body = serde_json::json!({ "version": payload });
        self.http.post_single(&path, "version", &body, "versions.create_on_project")
    }

    /// Atualiza uma versão.
    ///
    /// # Parâmetros
    /// - `id` — Identificador único da versão
    /// - `payload` — Dados atualizados da versão
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// let payload = UpdateVersionPayload { name: Some("v2.1".into()), ..Default::default() };
    /// client.versions.update(3, &payload)?;
    /// ```
    #[must_use]
    pub fn update(&self, id: RedmineId, payload: &UpdateVersionPayload) -> Result<(), RedmineError> {
        let path = format!("/versions/{}.json", id);
        let body = serde_json::json!({ "version": payload });
        self.http.put::<serde_json::Value, _>(&path, &body, "versions.update")?;
        Ok(())
    }

    /// Exclui uma versão.
    ///
    /// # Parâmetros
    /// - `id` — Identificador único da versão a ser excluída
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// client.versions.delete(3)?;
    /// ```
    #[must_use]
    pub fn delete(&self, id: RedmineId) -> Result<(), RedmineError> {
        let path = format!("/versions/{}.json", id);
        self.http.delete(&path, &[], "versions.delete")
    }
}
