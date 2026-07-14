// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::sync::Arc;

use crate::core::errors::RedmineError;
use crate::http::client::HttpClient;
use crate::types::issue_status::IssueStatus;

/// Recurso para operações com status de issue.
#[derive(Debug)]
pub struct IssueStatusesResource {
    http: Arc<HttpClient>,
}

impl IssueStatusesResource {
    #[must_use]
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Lista todos os status de issue disponíveis.
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// let statuses = client.issue_statuses.list().await?;
    /// ```
    pub async fn list(&self) -> Result<Vec<IssueStatus>, RedmineError> {
        let (items, _total) = self.http.get_paginated("/issue_statuses.json", "issue_statuses", None, &[], "issue_statuses.list").await?;
        Ok(items)
    }
}
