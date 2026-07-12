// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};
use crate::types::base::{RedmineId, IdName};

/// Filtros para listagem de apontamentos de horas.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TimeEntryFilter {
    /// Filtra por ID do usuário.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<RedmineId>,
    /// Filtra por ID do projeto.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<RedmineId>,
    /// Filtra por ID da issue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_id: Option<RedmineId>,
    /// Filtra pela data em que o tempo foi gasto (formato AAAA-MM-DD).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spent_on: Option<String>,
    /// Data inicial para filtro por período (formato AAAA-MM-DD).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    /// Data final para filtro por período (formato AAAA-MM-DD).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
}

/// Apontamento de horas retornado pela API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeEntry {
    /// Identificador único do apontamento.
    pub id: RedmineId,
    /// Projeto ao qual o apontamento pertence.
    pub project: Option<IdName>,
    /// Issue associada ao apontamento.
    pub issue: Option<IdName>,
    /// Usuário que registrou o apontamento.
    pub user: Option<IdName>,
    /// Atividade do apontamento.
    pub activity: Option<IdName>,
    /// Quantidade de horas gastas.
    pub hours: Option<f64>,
    /// Comentário sobre o apontamento.
    pub comments: Option<String>,
    /// Data em que o tempo foi gasto (formato AAAA-MM-DD).
    pub spent_on: Option<String>,
    /// Data de criação do registro.
    pub created_on: Option<String>,
    /// Data da última atualização do registro.
    pub updated_on: Option<String>,
    /// Valores dos campos personalizados associados.
    pub custom_fields: Option<Vec<super::base::CustomFieldValue>>,
}

/// Payload para criação de um apontamento de horas.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTimeEntryPayload {
    /// ID da issue associada (obrigatório se project_id não for informado).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_id: Option<RedmineId>,
    /// ID do projeto (obrigatório se issue_id não for informado).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<RedmineId>,
    /// Quantidade de horas gastas (obrigatório).
    pub hours: f64,
    /// ID da atividade (obrigatório).
    pub activity_id: RedmineId,
    /// Data em que o tempo foi gasto (formato AAAA-MM-DD).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spent_on: Option<String>,
    /// Comentário sobre o apontamento.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// ID do usuário para o qual o tempo será registrado.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<RedmineId>,
    /// Valores dos campos personalizados.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<super::base::CustomFieldPayload>>,
}

/// Payload para atualização de um apontamento de horas.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateTimeEntryPayload {
    /// Nova quantidade de horas gastas.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hours: Option<f64>,
    /// Novo ID da atividade.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<RedmineId>,
    /// Nova data em que o tempo foi gasto (formato AAAA-MM-DD).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spent_on: Option<String>,
    /// Novo comentário sobre o apontamento.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// Novo ID do usuário associado.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<RedmineId>,
    /// Novos valores dos campos personalizados.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<super::base::CustomFieldPayload>>,
}
