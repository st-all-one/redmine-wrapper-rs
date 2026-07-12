// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};
use crate::types::base::RedmineId;

/// Status de issue retornado pela API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueStatus {
    /// Identificador único do status.
    pub id: RedmineId,
    /// Nome do status (ex: "Novo", "Em Andamento", "Fechado").
    pub name: Option<String>,
    /// Indica se este status representa um estado fechado.
    pub is_closed: Option<bool>,
    /// Indica se este é o status padrão para novas issues.
    pub is_default: Option<bool>,
    /// Posição ordinal para ordenação dos status.
    pub position: Option<u32>,
}
