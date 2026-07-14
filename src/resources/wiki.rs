// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::sync::Arc;

use crate::core::errors::RedmineError;
use crate::http::client::HttpClient;
use crate::types::base::RedmineId;
use crate::types::wiki::*;

/// Recurso para operações com páginas wiki do Redmine.
#[derive(Debug)]
pub struct WikiResource {
    http: Arc<HttpClient>,
}

impl WikiResource {
    #[must_use]
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Lista páginas wiki de um projeto.
    ///
    /// # Parâmetros
    /// - `project_id` — Identificador do projeto
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// let pages = client.wiki.list(1).await?;
    /// ```
    pub async fn list(&self, project_id: RedmineId) -> Result<Vec<WikiPageSummary>, RedmineError> {
        let path = format!("/projects/{}/wiki/index.json", project_id);
        let (items, _total) = self.http.get_paginated(&path, "wiki_pages", None, &[], "wiki.list").await?;
        Ok(items)
    }

    /// Retorna uma página wiki pelo título.
    ///
    /// # Parâmetros
    /// - `project_id` — Identificador do projeto
    /// - `title` — Título da página wiki
    /// - `includes` — Campos adicionais opcionais (ex: `&["attachments", "versions"]`)
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// let page = client.wiki.get(1, "PáginaInicial", Some(&["attachments"])).await?;
    /// ```
    pub async fn get(&self, project_id: RedmineId, title: &str, includes: Option<&[&str]>) -> Result<WikiPage, RedmineError> {
        let path = format!("/projects/{}/wiki/{}.json", project_id, title);
        let mut query = Vec::new();
        if let Some(inc) = includes {
            query.push(("include", inc.join(",")));
        }
        self.http.get_single(&path, "wiki_page", &query, "wiki.get").await
    }

    /// Retorna uma versão específica de uma página wiki.
    ///
    /// # Parâmetros
    /// - `project_id` — Identificador do projeto
    /// - `title` — Título da página wiki
    /// - `version` — Número da versão desejada
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// let page = client.wiki.get_version(1, "PáginaInicial", 3).await?;
    /// ```
    pub async fn get_version(&self, project_id: RedmineId, title: &str, version: u32) -> Result<WikiPage, RedmineError> {
        let path = format!("/projects/{}/wiki/{}/{}.json", project_id, title, version);
        self.http.get_single(&path, "wiki_page", &[], "wiki.get_version").await
    }

    /// Cria ou atualiza uma página wiki.
    ///
    /// Se a página já existir, o conteúdo é atualizado; caso contrário, é criada.
    ///
    /// # Parâmetros
    /// - `project_id` — Identificador do projeto
    /// - `title` — Título da página wiki
    /// - `payload` — Dados da página (texto, comentários, etc.)
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// let payload = CreateWikiPagePayload { text: "Novo conteúdo".into(), ..Default::default() };
    /// client.wiki.create_or_update(1, "PáginaInicial", &payload).await?;
    /// ```
    pub async fn create_or_update(&self, project_id: RedmineId, title: &str, payload: &CreateWikiPagePayload) -> Result<(), RedmineError> {
        let path = format!("/projects/{}/wiki/{}.json", project_id, title);
        let body = serde_json::json!({ "wiki_page": payload });
        self.http.put::<serde_json::Value, _>(&path, &body, "wiki.create_or_update").await?;
        Ok(())
    }

    /// Exclui uma página wiki.
    ///
    /// # Parâmetros
    /// - `project_id` — Identificador do projeto
    /// - `title` — Título da página wiki a ser excluída
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// client.wiki.delete(1, "PáginaAntiga").await?;
    /// ```
    pub async fn delete(&self, project_id: RedmineId, title: &str) -> Result<(), RedmineError> {
        let path = format!("/projects/{}/wiki/{}.json", project_id, title);
        self.http.delete(&path, &[], "wiki.delete").await
    }
}
