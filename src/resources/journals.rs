// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::sync::Arc;

use crate::core::errors::RedmineError;
use crate::http::client::HttpClient;
use crate::types::base::RedmineId;
use crate::types::journal::*;

/// Recurso para operações com journals (histórico de issues).
#[derive(Debug)]
pub struct JournalsResource {
    http: Arc<HttpClient>,
}

impl JournalsResource {
    #[must_use]
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Atualiza as anotações de um journal.
    ///
    /// # Parâmetros
    /// - `id` — ID numérico do journal
    /// - `payload` — Dados atualizados do journal (novas anotações)
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// let payload = UpdateJournalPayload { notes: "Nova anotação".into() };
    /// client.journals.update(1, &payload).await?;
    /// ```
    pub async fn update(&self, id: RedmineId, payload: &UpdateJournalPayload) -> Result<(), RedmineError> {
        let path = format!("/journals/{}.json", id);
        let body = serde_json::json!({ "journal": payload });
        self.http.put::<serde_json::Value, _>(&path, &body, "journals.update").await?;
        Ok(())
    }

    /// Remove as anotações de um journal (define a anotação como string vazia).
    ///
    /// # Parâmetros
    /// - `id` — ID numérico do journal
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// client.journals.remove(1).await?;
    /// ```
    pub async fn remove(&self, id: RedmineId) -> Result<(), RedmineError> {
        let path = format!("/journals/{}.json", id);
        let body = serde_json::json!({ "journal": { "notes": "" } });
        self.http.put::<serde_json::Value, _>(&path, &body, "journals.remove").await?;
        Ok(())
    }
}
