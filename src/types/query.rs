// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};
use crate::types::base::RedmineId;

/// Consulta salva (query) do Redmine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Query {
    /// Identificador único da consulta.
    pub id: RedmineId,
    /// Nome da consulta salva.
    pub name: Option<String>,
    /// Indica se a consulta é pública (visível a todos os usuários).
    pub is_public: Option<bool>,
    /// ID do projeto ao qual a consulta está associada (se for específica de um projeto).
    pub project_id: Option<RedmineId>,
}
