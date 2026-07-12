// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};
use crate::types::base::{RedmineId, IdName};

/// Status de um projeto no Redmine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProjectStatus {
    /// Projeto ativo.
    #[serde(rename = "1")]
    Active,
    /// Projeto encerrado.
    #[serde(rename = "5")]
    Closed,
    /// Projeto arquivado.
    #[serde(rename = "9")]
    Archived,
}

/// Projeto retornado pela API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    /// Identificador único do projeto.
    pub id: RedmineId,
    /// Nome do projeto.
    pub name: Option<String>,
    /// Identificador textual único do projeto.
    pub identifier: Option<String>,
    /// Descrição do projeto.
    pub description: Option<String>,
    /// URL da página inicial do projeto.
    pub homepage: Option<String>,
    /// Status atual do projeto.
    pub status: Option<ProjectStatus>,
    /// Indica se o projeto é público.
    pub is_public: Option<bool>,
    /// Projeto pai (subprojeto).
    pub parent: Option<IdName>,
    /// Data de criação do projeto.
    pub created_on: Option<String>,
    /// Data da última atualização do projeto.
    pub updated_on: Option<String>,
    /// Lista de trackers disponíveis no projeto.
    pub trackers: Option<Vec<IdName>>,
    /// Lista de categorias de issues do projeto.
    pub issue_categories: Option<Vec<IdName>>,
    /// Módulos habilitados no projeto.
    pub enabled_modules: Option<Vec<IdName>>,
    /// Atividades de apontamento de horas do projeto.
    pub time_entry_activities: Option<Vec<IdName>>,
    /// Campos personalizados das issues do projeto.
    pub issue_custom_fields: Option<Vec<IdName>>,
}

/// Payload para criação de um projeto.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProjectPayload {
    /// Nome do projeto (obrigatório).
    pub name: String,
    /// Identificador textual único do projeto (obrigatório).
    pub identifier: String,
    /// Descrição do projeto.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// URL da página inicial do projeto.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
    /// Indica se o projeto é público.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,
    /// ID do projeto pai (subprojeto).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<RedmineId>,
    /// IDs dos trackers habilitados no projeto.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracker_ids: Option<Vec<RedmineId>>,
    /// Nomes dos módulos habilitados no projeto.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_module_names: Option<Vec<String>>,
}

/// Payload para atualização de um projeto.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateProjectPayload {
    /// Novo nome do projeto.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Nova descrição do projeto.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Nova URL da página inicial do projeto.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
    /// Indica se o projeto é público.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,
    /// ID do projeto pai (subprojeto).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<RedmineId>,
    /// IDs dos trackers habilitados no projeto.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracker_ids: Option<Vec<RedmineId>>,
    /// Nomes dos módulos habilitados no projeto.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_module_names: Option<Vec<String>>,
}

/// Wrapper para requisições de criação de projeto.
#[derive(Debug, Clone, Serialize)]
pub(crate) struct CreateProjectWrapper {
    /// Dados do projeto a ser criado.
    pub project: CreateProjectPayload,
}

/// Wrapper para requisições de atualização de projeto.
#[derive(Debug, Clone, Serialize)]
pub(crate) struct UpdateProjectWrapper {
    pub project: UpdateProjectPayload,
}
