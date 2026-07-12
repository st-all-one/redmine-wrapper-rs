// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};
use crate::types::base::RedmineId;

/// Prioridade de issue.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssuePriority {
    /// Identificador único da prioridade.
    pub id: RedmineId,
    /// Nome da prioridade (ex: "Baixa", "Normal", "Alta").
    pub name: Option<String>,
    /// Indica se esta é a prioridade padrão do sistema.
    pub is_default: Option<bool>,
    /// Indica se a prioridade está ativa para uso.
    pub active: Option<bool>,
    /// Posição ordinal para ordenação da prioridade.
    pub position: Option<u32>,
}

/// Atividade de apontamento de horas.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeEntryActivity {
    /// Identificador único da atividade.
    pub id: RedmineId,
    /// Nome da atividade (ex: "Desenvolvimento", "Reunião").
    pub name: Option<String>,
    /// Indica se esta é a atividade padrão do sistema.
    pub is_default: Option<bool>,
    /// Indica se a atividade está ativa para uso.
    pub active: Option<bool>,
    /// Posição ordinal para ordenação da atividade.
    pub position: Option<u32>,
}

/// Categoria de documento.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentCategory {
    /// Identificador único da categoria.
    pub id: RedmineId,
    /// Nome da categoria (ex: "Documentação Técnica", "Manual").
    pub name: Option<String>,
    /// Indica se esta é a categoria padrão do sistema.
    pub is_default: Option<bool>,
    /// Indica se a categoria está ativa para uso.
    pub active: Option<bool>,
    /// Posição ordinal para ordenação da categoria.
    pub position: Option<u32>,
}
