// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};
use crate::types::base::{RedmineId, IdName};

/// Associação de usuário/grupo a um projeto.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Membership {
    /// Identificador único da associação.
    pub id: RedmineId,
    /// Projeto ao qual o usuário/grupo está associado.
    pub project: Option<IdName>,
    /// Usuário associado (preenchido se for associação de usuário).
    pub user: Option<IdName>,
    /// Grupo associado (preenchido se for associação de grupo).
    pub group: Option<IdName>,
    /// Papéis (roles) atribuídos ao usuário/grupo no projeto.
    pub roles: Option<Vec<IdName>>,
}

/// Payload para criar uma associação.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMembershipPayload {
    /// ID do usuário a ser associado (opcional se group_id for informado).
    pub user_id: Option<RedmineId>,
    /// ID do grupo a ser associado (opcional se user_id for informado).
    pub group_id: Option<RedmineId>,
    /// IDs dos papéis (roles) a serem atribuídos.
    pub role_ids: Vec<RedmineId>,
}

/// Payload para atualizar uma associação.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateMembershipPayload {
    /// Novos IDs dos papéis (roles) a serem atribuídos.
    pub role_ids: Vec<RedmineId>,
}
