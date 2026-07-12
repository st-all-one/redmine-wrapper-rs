// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::sync::Arc;

use crate::core::errors::RedmineError;
use crate::http::client::HttpClient;
use crate::types::base::RedmineId;
use crate::types::role::Role;

/// Recurso para operações com papéis (roles) do Redmine.
#[derive(Debug)]
pub struct RolesResource {
    http: Arc<HttpClient>,
}

impl RolesResource {
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Lista todos os papéis disponíveis.
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// let roles = client.roles.list()?;
    /// ```
    #[must_use]
    pub fn list(&self) -> Result<Vec<Role>, RedmineError> {
        let (items, _total) = self.http.get_paginated("/roles.json", "roles", None, &[], "roles.list")?;
        Ok(items)
    }

    /// Retorna um papel pelo ID, incluindo permissões.
    ///
    /// # Parâmetros
    /// - `id` — Identificador único do papel
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// let role = client.roles.get(4)?;
    /// ```
    #[must_use]
    pub fn get(&self, id: RedmineId) -> Result<Role, RedmineError> {
        let path = format!("/roles/{}.json", id);
        self.http.get_single(&path, "role", &[], "roles.get")
    }
}
