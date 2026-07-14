// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::sync::Arc;

use crate::core::errors::RedmineError;
use crate::http::client::HttpClient;
use crate::types::enumeration::*;

/// Recurso para operações com enumerações do Redmine.
#[derive(Debug)]
pub struct EnumerationsResource {
    http: Arc<HttpClient>,
}

impl EnumerationsResource {
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Lista prioridades de issue disponíveis.
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// let priorities = client.enumerations.list_issue_priorities().await?;
    /// ```
    pub async fn list_issue_priorities(&self) -> Result<Vec<IssuePriority>, RedmineError> {
        let (items, _total) = self.http.get_paginated("/enumerations/issue_priorities.json", "issue_priorities", None, &[], "enumerations.list_issue_priorities").await?;
        Ok(items)
    }

    /// Lista atividades de apontamento de horas disponíveis.
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// let activities = client.enumerations.list_time_entry_activities().await?;
    /// ```
    pub async fn list_time_entry_activities(&self) -> Result<Vec<TimeEntryActivity>, RedmineError> {
        let (items, _total) = self.http.get_paginated("/enumerations/time_entry_activities.json", "time_entry_activities", None, &[], "enumerations.list_time_entry_activities").await?;
        Ok(items)
    }

    /// Lista categorias de documento disponíveis.
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// let categories = client.enumerations.list_document_categories().await?;
    /// ```
    pub async fn list_document_categories(&self) -> Result<Vec<DocumentCategory>, RedmineError> {
        let (items, _total) = self.http.get_paginated("/enumerations/document_categories.json", "document_categories", None, &[], "enumerations.list_document_categories").await?;
        Ok(items)
    }
}
