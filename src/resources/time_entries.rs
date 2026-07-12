// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::sync::Arc;

use crate::core::errors::RedmineError;
use crate::http::client::HttpClient;
use crate::utils::query::filter_to_query;
use crate::types::base::RedmineId;
use crate::types::time_entry::*;

/// Recurso para operações com apontamentos de horas.
#[derive(Debug)]
pub struct TimeEntriesResource {
    http: Arc<HttpClient>,
}

impl TimeEntriesResource {
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Lista apontamentos de horas com filtros opcionais.
    ///
    /// # Parâmetros
    /// - `filter` — Filtros opcionais (usuário, projeto, issue, data)
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// let entries = client.time_entries.list(None)?;
    /// ```
    #[must_use]
    pub fn list(&self, filter: Option<&TimeEntryFilter>) -> Result<Vec<TimeEntry>, RedmineError> {
        let base = filter_to_query(filter);
        let query: Vec<(&str, String)> = base.iter().map(|(k, v)| (k.as_str(), v.clone())).collect();
        self.http.get_all_paginated("/time_entries.json", "time_entries", &query, "time_entries.list")
    }

    /// Retorna um apontamento de horas pelo ID.
    ///
    /// # Parâmetros
    /// - `id` — ID numérico do apontamento de horas
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// let entry = client.time_entries.get(1)?;
    /// ```
    #[must_use]
    pub fn get(&self, id: RedmineId) -> Result<TimeEntry, RedmineError> {
        let path = format!("/time_entries/{}.json", id);
        self.http.get_single(&path, "time_entry", &[], "time_entries.get")
    }

    /// Cria um novo apontamento de horas.
    ///
    /// # Parâmetros
    /// - `payload` — Dados do novo apontamento (issue_id, horas, data, atividade, etc.)
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// let payload = CreateTimeEntryPayload { issue_id: Some(42), hours: 2.5, ..Default::default() };
    /// let entry = client.time_entries.create(&payload)?;
    /// ```
    #[must_use]
    pub fn create(&self, payload: &CreateTimeEntryPayload) -> Result<TimeEntry, RedmineError> {
        let body = serde_json::json!({ "time_entry": payload });
        self.http.post_single("/time_entries.json", "time_entry", &body, "time_entries.create")
    }

    /// Atualiza um apontamento de horas existente.
    ///
    /// # Parâmetros
    /// - `id` — ID numérico do apontamento de horas
    /// - `payload` — Dados atualizados do apontamento
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// let payload = UpdateTimeEntryPayload { hours: Some(3.0), ..Default::default() };
    /// client.time_entries.update(1, &payload)?;
    /// ```
    #[must_use]
    pub fn update(&self, id: RedmineId, payload: &UpdateTimeEntryPayload) -> Result<(), RedmineError> {
        let path = format!("/time_entries/{}.json", id);
        let body = serde_json::json!({ "time_entry": payload });
        self.http.put::<serde_json::Value, _>(&path, &body, "time_entries.update")?;
        Ok(())
    }

    /// Exclui um apontamento de horas permanentemente.
    ///
    /// # Parâmetros
    /// - `id` — ID numérico do apontamento de horas
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// client.time_entries.delete(1)?;
    /// ```
    #[must_use]
    pub fn delete(&self, id: RedmineId) -> Result<(), RedmineError> {
        let path = format!("/time_entries/{}.json", id);
        self.http.delete(&path, &[], "time_entries.delete")
    }
}
