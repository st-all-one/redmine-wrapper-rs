// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};
use crate::types::base::{RedmineId, IdName};

/// Status de um usuário no Redmine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserStatus {
    /// Usuário ativo.
    #[serde(rename = "1")]
    Active,
    /// Usuário registrado (aguardando ativação).
    #[serde(rename = "2")]
    Registered,
    /// Usuário bloqueado.
    #[serde(rename = "3")]
    Locked,
}

/// Filtros para listagem de usuários.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserFilter {
    /// Filtra por status do usuário.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<UserStatus>,
    /// Filtra por nome (login, nome ou sobrenome).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Filtra por ID do grupo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<RedmineId>,
}

/// Usuário retornado pela API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
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
    /// Indica se o usuário é administrador.
    pub admin: Option<bool>,
    /// Status atual do usuário.
    pub status: Option<UserStatus>,
    /// Chave de API do usuário.
    pub api_key: Option<String>,
    /// Data de criação do usuário.
    pub created_on: Option<String>,
    /// Data da última atualização do usuário.
    pub updated_on: Option<String>,
    /// Data do último login do usuário.
    pub last_login_on: Option<String>,
    /// Associações do usuário a projetos.
    pub memberships: Option<Vec<MembershipSummary>>,
    /// Grupos aos quais o usuário pertence.
    pub groups: Option<Vec<IdName>>,
}

/// Resumo de associação de usuário a um projeto.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MembershipSummary {
    /// Identificador único da associação.
    pub id: RedmineId,
    /// Projeto ao qual o usuário está associado.
    pub project: IdName,
    /// Papéis (funções) do usuário no projeto.
    pub roles: Vec<IdName>,
}

/// Payload para criação de um usuário.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserPayload {
    /// Nome de login do usuário (obrigatório).
    pub login: String,
    /// Primeiro nome do usuário (obrigatório).
    pub firstname: String,
    /// Sobrenome do usuário (obrigatório).
    pub lastname: String,
    /// Endereço de e-mail do usuário (obrigatório).
    pub mail: String,
    /// Senha do usuário. Omitir para usar autenticação externa.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// ID da fonte de autenticação externa (LDAP, etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_source_id: Option<RedmineId>,
    /// Obriga o usuário a alterar a senha no próximo login.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub must_change_passwd: Option<bool>,
    /// Define se o usuário é administrador.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin: Option<bool>,
    /// Status do usuário (ativo, registrado, bloqueado).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<UserStatus>,
    /// Valores para campos personalizados do usuário.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<super::base::CustomFieldPayload>>,
}

/// Payload para atualização de um usuário.
///
/// Todos os campos são opcionais — apenas os campos fornecidos serão alterados.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateUserPayload {
    /// Novo nome de login.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login: Option<String>,
    /// Novo primeiro nome.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firstname: Option<String>,
    /// Novo sobrenome.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lastname: Option<String>,
    /// Novo endereço de e-mail.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mail: Option<String>,
    /// Nova senha.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// Novo ID de fonte de autenticação externa.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_source_id: Option<RedmineId>,
    /// Obrigar alteração de senha no próximo login.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub must_change_passwd: Option<bool>,
    /// Definir/remover privilégios de administrador.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin: Option<bool>,
    /// Novo status do usuário.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<UserStatus>,
    /// Novos valores para campos personalizados.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<super::base::CustomFieldPayload>>,
}
