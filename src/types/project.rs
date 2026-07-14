// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};
use crate::types::base::{RedmineId, IdName};

/// Status de um projeto no Redmine.
///
/// A API retorna os status como números inteiros (1, 5, 9),
/// mas também aceita strings por robustez.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectStatus {
    /// Projeto ativo.
    Active = 1,
    /// Projeto encerrado.
    Closed = 5,
    /// Projeto arquivado.
    Archived = 9,
}

impl serde::Serialize for ProjectStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32(*self as u32)
    }
}

impl<'de> serde::Deserialize<'de> for ProjectStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de;

        struct Visitor;
        impl<'de> de::Visitor<'de> for Visitor {
            type Value = ProjectStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("número (1, 5, 9) ou string (\"1\", \"5\", \"9\")")
            }

            fn visit_u64<E: de::Error>(self, v: u64) -> Result<ProjectStatus, E> {
                match v {
                    1 => Ok(ProjectStatus::Active),
                    5 => Ok(ProjectStatus::Closed),
                    9 => Ok(ProjectStatus::Archived),
                    _ => Err(E::custom(format!("status de projeto desconhecido: {v}"))),
                }
            }

            fn visit_str<E: de::Error>(self, v: &str) -> Result<ProjectStatus, E> {
                match v {
                    "1" => Ok(ProjectStatus::Active),
                    "5" => Ok(ProjectStatus::Closed),
                    "9" => Ok(ProjectStatus::Archived),
                    _ => Err(E::custom(format!("status de projeto desconhecido: {v}"))),
                }
            }
        }
        deserializer.deserialize_any(Visitor)
    }
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
    /// Indica se herda membros do projeto pai.
    #[serde(rename = "inherit_members")]
    pub inherit_members: Option<bool>,
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
    /// Campos personalizados do projeto.
    #[serde(rename = "custom_fields")]
    pub custom_fields: Option<Vec<crate::types::base::CustomFieldValue>>,
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
