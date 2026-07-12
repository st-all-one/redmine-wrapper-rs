// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::types::base::{RedmineId, IdName, CustomFieldValue, CustomFieldPayload, UploadPayload};

/// Status permitido para transição de uma issue.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllowedStatus {
    /// ID do status permitido.
    pub id: RedmineId,
    /// Nome do status permitido.
    pub name: String,
}

/// Filtros disponíveis para listagem de issues.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IssueFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Filtra por ID específico de issue.
    pub issue_id: Option<RedmineId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Filtra por ID do projeto.
    pub project_id: Option<RedmineId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Filtra por ID do subprojeto (identificador).
    pub subproject_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Filtra por ID do tracker.
    pub tracker_id: Option<RedmineId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Filtra por status (ex: "open", "closed", "*").
    pub status_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Filtra por ID do usuário designado ("me" para usuário atual).
    pub assigned_to_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Filtra por ID da issue pai.
    pub parent_id: Option<RedmineId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Filtra por ID da prioridade.
    pub priority_id: Option<RedmineId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Filtra por ID da categoria.
    pub category_id: Option<RedmineId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Filtra por ID da versão alvo (fixed_version).
    pub fixed_version_id: Option<RedmineId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Filtra por ID do autor.
    pub author_id: Option<RedmineId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Filtra por data de criação.
    pub created_on: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Filtra por data de atualização.
    pub updated_on: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Campo para ordenação (ex: "created_on:desc").
    pub sort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ID da consulta salva (query) para aplicar filtros predefinidos.
    pub query_id: Option<RedmineId>,

    /// Campos personalizados para filtragem (ex: `{ "cf_1": "valor", "cf_5": "10" }`).
    /// As chaves devem estar no formato `cf_{id}`.
    #[serde(skip_serializing_if = "Option::is_none", flatten)]
    pub custom_fields: Option<HashMap<String, String>>,
}

/// Issue completa retornada pela API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Issue {
    /// Identificador único da issue.
    pub id: RedmineId,
    /// Projeto ao qual a issue pertence.
    pub project: Option<IdName>,
    /// Tracker da issue (ex: Bug, Feature).
    pub tracker: Option<IdName>,
    /// Status atual da issue.
    pub status: Option<IdName>,
    /// Prioridade da issue.
    pub priority: Option<IdName>,
    /// Autor da issue.
    pub author: Option<IdName>,
    /// Usuário designado para a issue.
    pub assigned_to: Option<IdName>,
    /// Categoria da issue.
    pub category: Option<IdName>,
    /// Versão alvo (fixed version) da issue.
    pub fixed_version: Option<IdName>,
    /// Issue pai (se for subtarefa).
    pub parent: Option<IdName>,
    /// Assunto da issue.
    pub subject: Option<String>,
    /// Descrição detalhada da issue.
    pub description: Option<String>,
    /// Data de início.
    pub start_date: Option<String>,
    /// Data de vencimento.
    pub due_date: Option<String>,
    /// Percentual de conclusão (0–100).
    pub done_ratio: Option<u32>,
    /// Horas estimadas.
    pub estimated_hours: Option<f64>,
    /// Total de horas estimadas (incluindo subtarefas).
    pub total_estimated_hours: Option<f64>,
    /// Horas gastas.
    pub spent_hours: Option<f64>,
    /// Total de horas gastas (incluindo subtarefas).
    pub total_spent_hours: Option<f64>,
    /// Indica se a issue é privada.
    pub is_private: Option<bool>,
    /// Data de fechamento da issue.
    pub closed_on: Option<String>,
    /// Data de criação.
    pub created_on: Option<String>,
    /// Data da última atualização.
    pub updated_on: Option<String>,
    /// Campos personalizados da issue.
    pub custom_fields: Option<Vec<CustomFieldValue>>,
    /// Histórico de alterações (journals).
    pub journals: Option<Vec<super::journal::Journal>>,
    /// Anexos da issue.
    pub attachments: Option<Vec<super::attachment::Attachment>>,
    /// Relacionamentos com outras issues.
    pub relations: Option<Vec<super::relation::Relation>>,
    /// Observadores da issue.
    pub watchers: Option<Vec<IdName>>,
    /// Subtarefas (issues filhas).
    pub children: Option<Vec<Issue>>,
    /// Status permitidos para transição.
    pub allowed_statuses: Option<Vec<AllowedStatus>>,
}

/// Payload para criação de uma issue.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateIssuePayload {
    /// ID do projeto onde a issue será criada (obrigatório).
    pub project_id: RedmineId,
    /// Assunto da issue (obrigatório).
    pub subject: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ID do tracker.
    pub tracker_id: Option<RedmineId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ID do status inicial.
    pub status_id: Option<RedmineId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ID da prioridade.
    pub priority_id: Option<RedmineId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Descrição detalhada da issue.
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ID da categoria.
    pub category_id: Option<RedmineId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ID do usuário designado.
    pub assigned_to_id: Option<RedmineId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ID da issue pai (para subtarefas).
    pub parent_issue_id: Option<RedmineId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ID da versão alvo.
    pub fixed_version_id: Option<RedmineId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Horas estimadas para conclusão.
    pub estimated_hours: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Percentual de conclusão (0–100).
    pub done_ratio: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Indica se a issue é privada.
    pub is_private: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Campos personalizados.
    pub custom_fields: Option<Vec<CustomFieldPayload>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Anexos enviados via upload.
    pub uploads: Option<Vec<UploadPayload>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// IDs dos usuários observadores.
    pub watcher_user_ids: Option<Vec<RedmineId>>,
}

/// Payload para atualização de uma issue.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateIssuePayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Novo assunto da issue.
    pub subject: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Novo ID do tracker.
    pub tracker_id: Option<RedmineId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Novo ID do status.
    pub status_id: Option<RedmineId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Novo ID da prioridade.
    pub priority_id: Option<RedmineId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Nova descrição da issue.
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Novo ID da categoria.
    pub category_id: Option<RedmineId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Novo ID do usuário designado.
    pub assigned_to_id: Option<RedmineId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Novo ID da issue pai.
    pub parent_issue_id: Option<RedmineId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Novo ID da versão alvo.
    pub fixed_version_id: Option<RedmineId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Novas horas estimadas.
    pub estimated_hours: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Novo percentual de conclusão (0–100).
    pub done_ratio: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Indica se a issue deve ser privada.
    pub is_private: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Comentário sobre a alteração (nota).
    pub notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Indica se a nota é privada.
    pub private_notes: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Campos personalizados atualizados.
    pub custom_fields: Option<Vec<CustomFieldPayload>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Novos anexos.
    pub uploads: Option<Vec<UploadPayload>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// IDs dos observadores.
    pub watcher_user_ids: Option<Vec<RedmineId>>,
}

/// Wrapper para requisições de criação de issue.
#[derive(Debug, Clone, Serialize)]
pub(crate) struct CreateIssueWrapper {
    /// Payload de criação encapsulado.
    pub issue: CreateIssuePayload,
}

/// Wrapper para requisições de atualização de issue.
#[derive(Debug, Clone, Serialize)]
pub(crate) struct UpdateIssueWrapper {
    /// Payload de atualização encapsulado.
    pub issue: UpdateIssuePayload,
}
