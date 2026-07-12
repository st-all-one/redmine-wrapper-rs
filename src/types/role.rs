// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};
use crate::types::base::RedmineId;

/// Papel (role) retornado pela API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    /// Identificador único do papel.
    pub id: RedmineId,
    /// Nome do papel (ex: "Gerente", "Desenvolvedor", "Relator").
    pub name: Option<String>,
    /// Posição ordinal para ordenação dos papéis.
    pub position: Option<u32>,
    /// Lista de permissões associadas a este papel.
    pub permissions: Option<Vec<String>>,
    /// Indica se o papel foi herdado de um grupo pai.
    pub inherited: Option<bool>,
}
