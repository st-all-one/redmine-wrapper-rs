// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};
use crate::types::base::{RedmineId, IdName};

/// Grupo retornado pela API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    /// Identificador único do grupo.
    pub id: RedmineId,
    /// Nome do grupo.
    pub name: Option<String>,
    /// Usuários que pertencem ao grupo.
    pub users: Option<Vec<IdName>>,
    /// Associações (memberships) do grupo a projetos.
    pub memberships: Option<Vec<super::membership::Membership>>,
}

/// Payload para criação de um grupo.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateGroupPayload {
    /// Nome do novo grupo (obrigatório).
    pub name: String,
    /// IDs dos usuários a serem adicionados ao grupo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<RedmineId>>,
}

/// Payload para atualização de um grupo.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateGroupPayload {
    /// Novo nome do grupo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Nova lista de IDs de usuários do grupo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<RedmineId>>,
}
