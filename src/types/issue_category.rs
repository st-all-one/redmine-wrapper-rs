// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};
use crate::types::base::RedmineId;

/// Categoria de issue retornada pela API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueCategory {
    /// Identificador único da categoria.
    pub id: RedmineId,
    /// Nome da categoria.
    pub name: Option<String>,
    /// Projeto ao qual a categoria pertence.
    pub project: Option<super::base::IdName>,
    /// Usuário designado como responsável pela categoria.
    pub assigned_to: Option<super::base::IdName>,
}

/// Payload para criação de uma categoria.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateIssueCategoryPayload {
    /// Nome da nova categoria (obrigatório).
    pub name: String,
    /// ID do usuário responsável pela categoria.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_to_id: Option<RedmineId>,
}

/// Payload para atualização de uma categoria.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateIssueCategoryPayload {
    /// Novo nome da categoria.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Novo ID do usuário responsável.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_to_id: Option<RedmineId>,
}
