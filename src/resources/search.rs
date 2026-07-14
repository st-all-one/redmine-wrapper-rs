// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::sync::Arc;

use crate::core::errors::RedmineError;
use crate::http::client::HttpClient;
use crate::http::pagination::PaginationParams;
use crate::types::search::*;

/// Recurso para operações de busca textual.
#[derive(Debug)]
pub struct SearchResource {
    http: Arc<HttpClient>,
}

impl SearchResource {
    #[must_use]
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Executa uma busca textual nos recursos do Redmine.
    ///
    /// # Parâmetros
    /// - `filter` — Estrutura com os parâmetros da busca
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// let filter = SearchFilter { query: "bug".into(), issues: Some(true), ..Default::default() };
    /// let results = client.search.search(&filter).await?;
    /// ```
    pub async fn search(&self, filter: &SearchFilter) -> Result<Vec<SearchResult>, RedmineError> {
        use crate::utils::query::filter_to_query;
        let base = filter_to_query(Some(filter));
        let query: Vec<(&str, String)> = base.iter().map(|(k, v)| (k.as_str(), v.clone())).collect();

        let params = PaginationParams {
            offset: filter.offset,
            limit: filter.limit,
        };
        let (items, _total) = self.http.get_paginated("/search.json", "results", Some(&params), &query, "search.search").await?;
        Ok(items)
    }
}
