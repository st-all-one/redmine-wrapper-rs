// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::sync::Arc;

use crate::core::errors::RedmineError;
use crate::http::client::HttpClient;
use crate::types::custom_field::CustomField;

/// Recurso para operações com campos personalizados.
#[derive(Debug)]
pub struct CustomFieldsResource {
    http: Arc<HttpClient>,
}

impl CustomFieldsResource {
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Lista todos os campos personalizados configurados.
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// let fields = client.custom_fields.list()?;
    /// ```
    #[must_use]
    pub fn list(&self) -> Result<Vec<CustomField>, RedmineError> {
        let (items, _total) = self.http.get_paginated("/custom_fields.json", "custom_fields", None, &[], "custom_fields.list")?;
        Ok(items)
    }
}
