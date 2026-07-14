// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::sync::Arc;

use crate::core::errors::RedmineError;
use crate::http::client::HttpClient;
use crate::types::tracker::Tracker;

/// Recurso para operações com trackers (tipos de issue).
#[derive(Debug)]
pub struct TrackersResource {
    http: Arc<HttpClient>,
}

impl TrackersResource {
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Lista todos os trackers disponíveis.
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// let trackers = client.trackers.list().await?;
    /// ```
    pub async fn list(&self) -> Result<Vec<Tracker>, RedmineError> {
        let (items, _total) = self.http.get_paginated("/trackers.json", "trackers", None, &[], "trackers.list").await?;
        Ok(items)
    }
}
