// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::sync::Arc;

use crate::core::errors::RedmineError;
use crate::http::client::HttpClient;
use crate::types::query::Query;

/// Recurso para operações com consultas salvas.
#[derive(Debug)]
pub struct QueriesResource {
    http: Arc<HttpClient>,
}

impl QueriesResource {
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Lista todas as consultas salvas disponíveis.
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// let queries = client.queries.list()?;
    /// ```
    #[must_use]
    pub fn list(&self) -> Result<Vec<Query>, RedmineError> {
        let (items, _total) = self.http.get_paginated("/queries.json", "queries", None, &[], "queries.list")?;
        Ok(items)
    }
}
