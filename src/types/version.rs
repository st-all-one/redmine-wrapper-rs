// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};
use crate::types::base::{RedmineId, IdName};

/// Status de uma versão.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VersionStatus {
    /// Versão aberta para edição.
    #[serde(rename = "open")]
    Open,
    /// Versão bloqueada, não pode ser alterada.
    #[serde(rename = "locked")]
    Locked,
    /// Versão encerrada/finalizada.
    #[serde(rename = "closed")]
    Closed,
}

/// Comportamento de compartilhamento de versão.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VersionSharing {
    /// Sem compartilhamento.
    #[serde(rename = "none")]
    None,
    /// Compartilhada com descendentes.
    #[serde(rename = "descendants")]
    Descendants,
    /// Compartilhada com a hierarquia do projeto.
    #[serde(rename = "hierarchy")]
    Hierarchy,
    /// Compartilhada com toda a árvore de projetos.
    #[serde(rename = "tree")]
    Tree,
    /// Compartilhada com todos os projetos do sistema.
    #[serde(rename = "system")]
    System,
}

/// Versão retornada pela API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Version {
    /// Identificador único da versão.
    pub id: RedmineId,
    /// Projeto ao qual a versão pertence.
    pub project: Option<IdName>,
    /// Nome da versão.
    pub name: Option<String>,
    /// Descrição detalhada da versão.
    pub description: Option<String>,
    /// Status atual da versão.
    pub status: Option<VersionStatus>,
    /// Comportamento de compartilhamento da versão.
    pub sharing: Option<VersionSharing>,
    /// Data de vencimento da versão (formato ISO 8601).
    pub due_date: Option<String>,
    /// Data de criação da versão (formato ISO 8601).
    pub created_on: Option<String>,
    /// Data da última atualização da versão (formato ISO 8601).
    pub updated_on: Option<String>,
    /// Título da página wiki associada à versão.
    pub wiki_page_title: Option<String>,
}

/// Payload para criação de uma versão.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateVersionPayload {
    /// Nome da versão (obrigatório).
    pub name: String,
    /// Descrição detalhada da versão.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Status inicial da versão.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<VersionStatus>,
    /// Comportamento de compartilhamento da versão.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharing: Option<VersionSharing>,
    /// Data de vencimento da versão (formato ISO 8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
}

/// Payload para atualização de uma versão.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateVersionPayload {
    /// Novo nome da versão.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Nova descrição da versão.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Novo status da versão.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<VersionStatus>,
    /// Novo comportamento de compartilhamento.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharing: Option<VersionSharing>,
    /// Nova data de vencimento (formato ISO 8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
}
