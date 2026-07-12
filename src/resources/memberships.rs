// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::sync::Arc;

use crate::core::errors::RedmineError;
use crate::http::client::HttpClient;
use crate::types::base::RedmineId;
use crate::types::membership::*;

/// Recurso para operações com associações de usuários/grupos a projetos.
#[derive(Debug)]
pub struct MembershipsResource {
    http: Arc<HttpClient>,
}

impl MembershipsResource {
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Lista associações de um projeto.
    ///
    /// # Parâmetros
    /// - `project_id` — Identificador do projeto
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// let memberships = client.memberships.list_by_project(1)?;
    /// ```
    #[must_use]
    pub fn list_by_project(&self, project_id: RedmineId) -> Result<Vec<Membership>, RedmineError> {
        let path = format!("/projects/{}/memberships.json", project_id);
        let (items, _total) = self.http.get_paginated(&path, "memberships", None, &[], "memberships.list_by_project")?;
        Ok(items)
    }

    /// Retorna uma associação pelo ID.
    ///
    /// # Parâmetros
    /// - `id` — Identificador único da associação
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// let membership = client.memberships.get(15)?;
    /// ```
    #[must_use]
    pub fn get(&self, id: RedmineId) -> Result<Membership, RedmineError> {
        let path = format!("/memberships/{}.json", id);
        self.http.get_single(&path, "membership", &[], "memberships.get")
    }

    /// Cria uma associação em um projeto.
    ///
    /// # Parâmetros
    /// - `project_id` — Identificador do projeto
    /// - `payload` — Dados da associação a ser criada (usuário/grupo e papéis)
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// let payload = CreateMembershipPayload { user_id: Some(10), role_ids: vec![3] };
    /// let membership = client.memberships.create(1, &payload)?;
    /// ```
    #[must_use]
    pub fn create(&self, project_id: RedmineId, payload: &CreateMembershipPayload) -> Result<Membership, RedmineError> {
        let path = format!("/projects/{}/memberships.json", project_id);
        let body = serde_json::json!({ "membership": payload });
        self.http.post_single(&path, "membership", &body, "memberships.create")
    }

    /// Atualiza uma associação.
    ///
    /// # Parâmetros
    /// - `id` — Identificador único da associação
    /// - `payload` — Dados atualizados da associação
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// let payload = UpdateMembershipPayload { role_ids: Some(vec![4, 5]) };
    /// client.memberships.update(15, &payload)?;
    /// ```
    #[must_use]
    pub fn update(&self, id: RedmineId, payload: &UpdateMembershipPayload) -> Result<(), RedmineError> {
        let path = format!("/memberships/{}.json", id);
        let body = serde_json::json!({ "membership": payload });
        self.http.put::<serde_json::Value, _>(&path, &body, "memberships.update")?;
        Ok(())
    }

    /// Exclui uma associação.
    ///
    /// # Parâmetros
    /// - `id` — Identificador único da associação a ser excluída
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// client.memberships.delete(15)?;
    /// ```
    #[must_use]
    pub fn delete(&self, id: RedmineId) -> Result<(), RedmineError> {
        let path = format!("/memberships/{}.json", id);
        self.http.delete(&path, &[], "memberships.delete")
    }
}
