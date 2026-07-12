// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};
use crate::types::base::RedmineId;

/// Tracker (tipo de issue) retornado pela API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tracker {
    /// Identificador único do tracker.
    pub id: RedmineId,
    /// Nome do tracker (ex: "Bug", "Feature", "Suporte").
    pub name: Option<String>,
    /// Status padrão associado a este tracker.
    pub default_status: Option<super::base::IdName>,
    /// Descrição do tracker.
    pub description: Option<String>,
}
