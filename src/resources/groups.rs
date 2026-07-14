// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::sync::Arc;

use serde_json::json;

use crate::core::errors::RedmineError;
use crate::http::client::HttpClient;
use crate::types::base::RedmineId;
use crate::types::group::*;

/// Recurso para operações com grupos do Redmine.
#[derive(Debug)]
pub struct GroupsResource {
    http: Arc<HttpClient>,
}

impl GroupsResource {
    #[must_use]
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Lista todos os grupos.
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// let groups = client.groups.list().await?;
    /// ```
    pub async fn list(&self) -> Result<Vec<Group>, RedmineError> {
        self.http.get_all_paginated("/groups.json", "groups", &[], "groups.list").await
    }

    /// Retorna um grupo pelo ID.
    ///
    /// # Parâmetros
    /// - `id` — Identificador único do grupo
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// let group = client.groups.get(5).await?;
    /// ```
    pub async fn get(&self, id: RedmineId) -> Result<Group, RedmineError> {
        let path = format!("/groups/{}.json", id);
        self.http.get_single(&path, "group", &[], "groups.get").await
    }

    /// Retorna um grupo com includes (users, memberships).
    ///
    /// # Parâmetros
    /// - `id` — Identificador único do grupo
    /// - `includes` — Lista de campos adicionais (ex: `&["users", "memberships"]`)
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// let group = client.groups.get_with_includes(5, &["users"]).await?;
    /// ```
    pub async fn get_with_includes(&self, id: RedmineId, includes: &[&str]) -> Result<Group, RedmineError> {
        let path = format!("/groups/{}.json", id);
        let query = vec![("include", includes.join(","))];
        self.http.get_single(&path, "group", &query, "groups.get_with_includes").await
    }

    /// Cria um novo grupo.
    ///
    /// # Parâmetros
    /// - `payload` — Dados do grupo a ser criado
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// let payload = CreateGroupPayload { name: "Desenvolvedores".into() };
    /// let group = client.groups.create(&payload).await?;
    /// ```
    pub async fn create(&self, payload: &CreateGroupPayload) -> Result<Group, RedmineError> {
        let body = json!({ "group": payload });
        self.http.post_single("/groups.json", "group", &body, "groups.create").await
    }

    /// Atualiza um grupo existente.
    ///
    /// # Parâmetros
    /// - `id` — Identificador único do grupo
    /// - `payload` — Dados atualizados do grupo
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// let payload = UpdateGroupPayload { name: Some("Devs".into()) };
    /// client.groups.update(5, &payload).await?;
    /// ```
    pub async fn update(&self, id: RedmineId, payload: &UpdateGroupPayload) -> Result<(), RedmineError> {
        let path = format!("/groups/{}.json", id);
        let body = json!({ "group": payload });
        self.http.put::<serde_json::Value, _>(&path, &body, "groups.update").await?;
        Ok(())
    }

    /// Exclui um grupo.
    ///
    /// # Parâmetros
    /// - `id` — Identificador único do grupo a ser excluído
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// client.groups.delete(5).await?;
    /// ```
    pub async fn delete(&self, id: RedmineId) -> Result<(), RedmineError> {
        let path = format!("/groups/{}.json", id);
        self.http.delete(&path, &[], "groups.delete").await
    }

    /// Adiciona um usuário a um grupo.
    ///
    /// # Parâmetros
    /// - `group_id` — Identificador do grupo
    /// - `user_id` — Identificador do usuário a ser adicionado
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// client.groups.add_user(5, 10).await?;
    /// ```
    pub async fn add_user(&self, group_id: RedmineId, user_id: RedmineId) -> Result<(), RedmineError> {
        let path = format!("/groups/{}/users.json", group_id);
        let body = json!({ "user_id": user_id });
        self.http.post::<serde_json::Value, _>(&path, &body, "groups.add_user").await?;
        Ok(())
    }

    /// Remove um usuário de um grupo.
    ///
    /// # Parâmetros
    /// - `group_id` — Identificador do grupo
    /// - `user_id` — Identificador do usuário a ser removido
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// client.groups.remove_user(5, 10).await?;
    /// ```
    pub async fn remove_user(&self, group_id: RedmineId, user_id: RedmineId) -> Result<(), RedmineError> {
        let path = format!("/groups/{}/users/{}.json", group_id, user_id);
        self.http.delete(&path, &[], "groups.remove_user").await
    }
}
