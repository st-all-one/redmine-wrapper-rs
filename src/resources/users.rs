// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::sync::Arc;

use crate::core::errors::RedmineError;
use crate::http::client::HttpClient;
use crate::utils::query::filter_to_query;
use crate::types::base::RedmineId;
use crate::types::user::*;

/// Recurso para operações com usuários do Redmine.
#[derive(Debug)]
pub struct UsersResource {
    http: Arc<HttpClient>,
}

impl UsersResource {
    #[must_use]
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Lista todos os usuários, com filtros opcionais.
    ///
    /// # Parâmetros
    /// - `filter` — Filtros opcionais (status, nome, grupo)
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// let users = client.users.list(None).await?;
    /// let filtered = client.users.list(Some(&UserFilter { status: Some(UserStatus::Active), ..Default::default() })).await?;
    /// ```
    pub async fn list(&self, filter: Option<&UserFilter>) -> Result<Vec<User>, RedmineError> {
        let base = filter_to_query(filter);
        let query: Vec<(&str, String)> = base.iter().map(|(k, v)| (k.as_str(), v.clone())).collect();
        self.http.get_all_paginated("/users.json", "users", &query, "users.list").await
    }

    /// Retorna um usuário pelo ID.
    ///
    /// # Parâmetros
    /// - `id` — ID numérico do usuário
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// let user = client.users.get(1).await?;
    /// ```
    pub async fn get(&self, id: RedmineId) -> Result<User, RedmineError> {
        let path = format!("/users/{}.json", id);
        self.http.get_single(&path, "user", &[], "users.get").await
    }

    /// Retorna um usuário com associações (memberships, groups).
    ///
    /// # Parâmetros
    /// - `id` — ID numérico do usuário
    /// - `includes` — Lista de associações a incluir (ex: `&["memberships", "groups"]`)
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// let user = client.users.get_with_includes(1, &["memberships", "groups"]).await?;
    /// ```
    pub async fn get_with_includes(&self, id: RedmineId, includes: &[&str]) -> Result<User, RedmineError> {
        let path = format!("/users/{}.json", id);
        let query = vec![("include", includes.join(","))];
        self.http.get_single(&path, "user", &query, "users.get_with_includes").await
    }

    /// Retorna os dados do usuário autenticado (via `/my/account.json`).
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// let current_user = client.users.get_current().await?;
    /// ```
    pub async fn get_current(&self) -> Result<User, RedmineError> {
        self.http.get_single("/my/account.json", "user", &[], "users.get_current").await
    }

    /// Retorna os dados do usuário autenticado via `/users/current.json`.
    ///
    /// Difere de `get_current()` por usar o endpoint `/users/current.json`
    /// que retorna o tipo [`User`] diretamente, sem passar por `MyAccount`.
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// let current = client.users.get_current_user().await?;
    /// ```
    pub async fn get_current_user(&self) -> Result<User, RedmineError> {
        self.http.get_single("/users/current.json", "user", &[], "users.get_current_user").await
    }

    /// Cria um novo usuário.
    ///
    /// # Parâmetros
    /// - `payload` — Dados do novo usuário (login, nome, e-mail, etc.)
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// let payload = CreateUserPayload { login: "joao".into(), firstname: "João".into(), lastname: "Silva".into(), mail: "joao@example.com".into(), ..Default::default() };
    /// let user = client.users.create(&payload).await?;
    /// ```
    pub async fn create(&self, payload: &CreateUserPayload) -> Result<User, RedmineError> {
        let body = serde_json::json!({ "user": payload });
        self.http.post_single("/users.json", "user", &body, "users.create").await
    }

    /// Atualiza um usuário existente.
    ///
    /// # Parâmetros
    /// - `id` — ID numérico do usuário
    /// - `payload` — Dados atualizados do usuário
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// let payload = UpdateUserPayload { firstname: Some("José".into()), ..Default::default() };
    /// client.users.update(1, &payload).await?;
    /// ```
    pub async fn update(&self, id: RedmineId, payload: &UpdateUserPayload) -> Result<(), RedmineError> {
        let path = format!("/users/{}.json", id);
        let body = serde_json::json!({ "user": payload });
        self.http.put::<serde_json::Value, _>(&path, &body, "users.update").await?;
        Ok(())
    }

    /// Exclui um usuário permanentemente.
    ///
    /// # Parâmetros
    /// - `id` — ID numérico do usuário
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// client.users.delete(1).await?;
    /// ```
    pub async fn delete(&self, id: RedmineId) -> Result<(), RedmineError> {
        let path = format!("/users/{}.json", id);
        self.http.delete(&path, &[], "users.delete").await
    }
}
