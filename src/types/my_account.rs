// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};
use crate::types::base::RedmineId;

/// Dados da conta do usuário autenticado.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MyAccount {
    /// Identificador único do usuário.
    pub id: RedmineId,
    /// Nome de login do usuário.
    pub login: Option<String>,
    /// Primeiro nome do usuário.
    pub firstname: Option<String>,
    /// Sobrenome do usuário.
    pub lastname: Option<String>,
    /// Endereço de e-mail do usuário.
    pub mail: Option<String>,
    /// Indica se o usuário possui privilégios de administrador.
    pub admin: Option<bool>,
    /// Data de criação da conta.
    pub created_on: Option<String>,
    /// Data da última atualização da conta.
    pub updated_on: Option<String>,
    /// Data do último login do usuário.
    pub last_login_on: Option<String>,
    /// Chave de API do usuário para autenticação.
    pub api_key: Option<String>,
    /// Associações (memberships) do usuário a projetos.
    pub memberships: Option<Vec<super::membership::Membership>>,
}
