// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::sync::Arc;

use crate::core::errors::RedmineError;
use crate::http::client::HttpClient;
use crate::types::base::RedmineId;
use crate::types::news::*;

/// Recurso para operações com notícias do Redmine.
#[derive(Debug)]
pub struct NewsResource {
    http: Arc<HttpClient>,
}

impl NewsResource {
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Lista notícias globais.
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// let all_news = client.news.list().await?;
    /// ```
    pub async fn list(&self) -> Result<Vec<News>, RedmineError> {
        let (items, _total) = self.http.get_paginated("/news.json", "news", None, &[], "news.list").await?;
        Ok(items)
    }

    /// Lista notícias de um projeto específico.
    ///
    /// # Parâmetros
    /// - `project_id` — Identificador do projeto
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// let project_news = client.news.list_by_project(1).await?;
    /// ```
    pub async fn list_by_project(&self, project_id: RedmineId) -> Result<Vec<News>, RedmineError> {
        let path = format!("/projects/{}/news.json", project_id);
        let (items, _total) = self.http.get_paginated(&path, "news", None, &[], "news.list_by_project").await?;
        Ok(items)
    }

    /// Retorna uma notícia pelo ID.
    ///
    /// # Parâmetros
    /// - `id` — Identificador único da notícia
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// let news = client.news.get(8).await?;
    /// ```
    pub async fn get(&self, id: RedmineId) -> Result<News, RedmineError> {
        let path = format!("/news/{}.json", id);
        self.http.get_single(&path, "news", &[], "news.get").await
    }

    /// Cria uma notícia em um projeto.
    ///
    /// # Parâmetros
    /// - `project_id` — Identificador do projeto
    /// - `payload` — Dados da notícia a ser criada
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// let payload = CreateNewsPayload { title: "Novidade".into(), summary: "Resumo".into(), ..Default::default() };
    /// let news = client.news.create(1, &payload).await?;
    /// ```
    pub async fn create(&self, project_id: RedmineId, payload: &CreateNewsPayload) -> Result<News, RedmineError> {
        let path = format!("/projects/{}/news.json", project_id);
        let body = serde_json::json!({ "news": payload });
        self.http.post_single(&path, "news", &body, "news.create").await
    }

    /// Atualiza uma notícia.
    ///
    /// # Parâmetros
    /// - `id` — Identificador único da notícia
    /// - `payload` — Dados atualizados da notícia
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// let payload = UpdateNewsPayload { title: Some("Título Atualizado".into()), ..Default::default() };
    /// client.news.update(8, &payload).await?;
    /// ```
    pub async fn update(&self, id: RedmineId, payload: &UpdateNewsPayload) -> Result<(), RedmineError> {
        let path = format!("/news/{}.json", id);
        let body = serde_json::json!({ "news": payload });
        self.http.put::<serde_json::Value, _>(&path, &body, "news.update").await?;
        Ok(())
    }

    /// Exclui uma notícia.
    ///
    /// # Parâmetros
    /// - `id` — Identificador único da notícia a ser excluída
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// client.news.delete(8).await?;
    /// ```
    pub async fn delete(&self, id: RedmineId) -> Result<(), RedmineError> {
        let path = format!("/news/{}.json", id);
        self.http.delete(&path, &[], "news.delete").await
    }
}
