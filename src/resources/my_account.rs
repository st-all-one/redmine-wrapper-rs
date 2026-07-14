// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::sync::Arc;

use crate::core::errors::RedmineError;
use crate::http::client::HttpClient;
use crate::types::my_account::MyAccount;

/// Recurso para operações com a conta do usuário autenticado.
#[derive(Debug)]
pub struct MyAccountResource {
    http: Arc<HttpClient>,
}

impl MyAccountResource {
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Retorna os dados da conta do usuário autenticado.
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// let account = client.my_account.get().await?;
    /// ```
    pub async fn get(&self) -> Result<MyAccount, RedmineError> {
        self.http.get_single("/my/account.json", "user", &[], "my_account.get").await
    }
}
