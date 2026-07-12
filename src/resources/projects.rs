// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::sync::Arc;

use crate::core::errors::RedmineError;
use crate::http::client::HttpClient;
use crate::types::base::RedmineId;
use crate::types::project::*;

/// Recurso para operações com projetos do Redmine.
#[derive(Debug)]
pub struct ProjectsResource {
    http: Arc<HttpClient>,
}

impl ProjectsResource {
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Lista todos os projetos.
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// let projects = client.projects.list()?;
    /// ```
    #[must_use]
    pub fn list(&self) -> Result<Vec<Project>, RedmineError> {
        self.http.get_all_paginated("/projects.json", "projects", &[], "projects.list")
    }

    /// Retorna um projeto pelo ID.
    ///
    /// # Parâmetros
    /// - `id` — ID numérico do projeto
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// let project = client.projects.get(1)?;
    /// ```
    #[must_use]
    pub fn get(&self, id: RedmineId) -> Result<Project, RedmineError> {
        let path = format!("/projects/{}.json", id);
        self.http.get_single(&path, "project", &[], "projects.get")
    }

    /// Retorna um projeto com associações (trackers, issue_categories, etc.).
    ///
    /// # Parâmetros
    /// - `id` — ID numérico do projeto
    /// - `includes` — Lista de associações a incluir (ex: `&["trackers", "issue_categories"]`)
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// let project = client.projects.get_with_includes(1, &["trackers", "issue_categories"])?;
    /// ```
    #[must_use]
    pub fn get_with_includes(&self, id: RedmineId, includes: &[&str]) -> Result<Project, RedmineError> {
        let path = format!("/projects/{}.json", id);
        let query = vec![("include", includes.join(","))];
        self.http.get_single(&path, "project", &query, "projects.get_with_includes")
    }

    /// Cria um novo projeto.
    ///
    /// # Parâmetros
    /// - `payload` — Dados do novo projeto (nome, identificador, descrição, etc.)
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// let payload = CreateProjectPayload { name: "Meu Projeto".into(), identifier: "meu-projeto".into(), ..Default::default() };
    /// let project = client.projects.create(&payload)?;
    /// ```
    #[must_use]
    pub fn create(&self, payload: &CreateProjectPayload) -> Result<Project, RedmineError> {
        self.http.post_single("/projects.json", "project", &CreateProjectWrapper { project: payload.clone() }, "projects.create")
    }

    /// Atualiza um projeto existente.
    ///
    /// # Parâmetros
    /// - `id` — ID numérico do projeto
    /// - `payload` — Dados atualizados do projeto
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// let payload = UpdateProjectPayload { name: Some("Novo Nome".into()), ..Default::default() };
    /// client.projects.update(1, &payload)?;
    /// ```
    #[must_use]
    pub fn update(&self, id: RedmineId, payload: &UpdateProjectPayload) -> Result<(), RedmineError> {
        let path = format!("/projects/{}.json", id);
        self.http.put::<serde_json::Value, _>(&path, &UpdateProjectWrapper { project: payload.clone() }, "projects.update")?;
        Ok(())
    }

    /// Exclui um projeto permanentemente.
    ///
    /// # Parâmetros
    /// - `id` — ID numérico do projeto
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// client.projects.delete(1)?;
    /// ```
    #[must_use]
    pub fn delete(&self, id: RedmineId) -> Result<(), RedmineError> {
        let path = format!("/projects/{}.json", id);
        self.http.delete(&path, &[], "projects.delete")
    }

    /// Arquivar um projeto (torna-o somente leitura).
    ///
    /// # Parâmetros
    /// - `id` — ID numérico do projeto
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// client.projects.archive(1)?;
    /// ```
    #[must_use]
    pub fn archive(&self, id: RedmineId) -> Result<(), RedmineError> {
        let path = format!("/projects/{}/archive.json", id);
        self.http.put::<serde_json::Value, _>(&path, &serde_json::json!({}), "projects.archive")?;
        Ok(())
    }

    /// Desarquivar um projeto (restaura o estado ativo).
    ///
    /// # Parâmetros
    /// - `id` — ID numérico do projeto
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// client.projects.unarchive(1)?;
    /// ```
    #[must_use]
    pub fn unarchive(&self, id: RedmineId) -> Result<(), RedmineError> {
        let path = format!("/projects/{}/unarchive.json", id);
        self.http.put::<serde_json::Value, _>(&path, &serde_json::json!({}), "projects.unarchive")?;
        Ok(())
    }
}
