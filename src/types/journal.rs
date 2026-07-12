// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};
use crate::types::base::{RedmineId, IdName};

/// Detalhe de alteração em um campo da issue dentro de um journal.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalDetail {
    /// Nome da propriedade alterada (ex: "status_id", "subject").
    pub property: Option<String>,
    /// Nome legível do campo alterado.
    pub name: Option<String>,
    /// Valor anterior do campo.
    pub old_value: Option<serde_json::Value>,
    /// Novo valor do campo.
    pub new_value: Option<serde_json::Value>,
}

/// Journal (histórico/anotação) de uma issue.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Journal {
    /// Identificador único do journal.
    pub id: RedmineId,
    /// Usuário que criou o journal.
    pub user: Option<IdName>,
    /// Anotações textuais do journal.
    pub notes: Option<String>,
    /// Data de criação do journal.
    pub created_on: Option<String>,
    /// Data da última atualização do journal.
    pub updated_on: Option<String>,
    /// Usuário que realizou a última atualização.
    pub updated_by: Option<IdName>,
    /// Indica se as anotações são privadas.
    pub private_notes: Option<bool>,
    /// Lista de detalhes das alterações realizadas.
    pub details: Option<Vec<JournalDetail>>,
}

/// Payload para atualização de um journal.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateJournalPayload {
    /// Novo texto das anotações (obrigatório).
    pub notes: String,
    /// Indica se as anotações devem ser privadas.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_notes: Option<bool>,
}
