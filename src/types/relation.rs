// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};
use crate::types::base::RedmineId;

/// Tipos de relação entre issues.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RelationType {
    /// Relacionada a (relates).
    #[serde(rename = "relates")]
    Relates,
    /// Duplica (duplicates).
    #[serde(rename = "duplicates")]
    Duplicates,
    /// Duplicada por (duplicated).
    #[serde(rename = "duplicated")]
    Duplicated,
    /// Bloqueia (blocks).
    #[serde(rename = "blocks")]
    Blocks,
    /// Bloqueada por (blocked).
    #[serde(rename = "blocked")]
    Blocked,
    /// Antecede (precedes).
    #[serde(rename = "precedes")]
    Precedes,
    /// Sucede (follows).
    #[serde(rename = "follows")]
    Follows,
    /// Copiada para (copied_to).
    #[serde(rename = "copied_to")]
    CopiedTo,
    /// Copiada de (copied_from).
    #[serde(rename = "copied_from")]
    CopiedFrom,
}

/// Relação entre duas issues.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relation {
    /// Identificador único da relação.
    pub id: RedmineId,
    /// ID da issue de origem.
    pub issue_id: Option<RedmineId>,
    /// ID da issue de destino.
    pub issue_to_id: Option<RedmineId>,
    /// Tipo da relação.
    pub relation_type: Option<RelationType>,
    /// Atraso em dias para relações do tipo "precedes" ou "follows".
    pub delay: Option<u32>,
}

/// Payload para criação de uma relação.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRelationPayload {
    /// ID da issue de destino (obrigatório).
    pub issue_to_id: RedmineId,
    /// Tipo da relação (obrigatório).
    pub relation_type: RelationType,
    /// Atraso em dias para relações do tipo "precedes" ou "follows".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay: Option<u32>,
}
