// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};

/// Tipo alias para identificadores numéricos do Redmine.
pub type RedmineId = u64;

/// Par genérico de ID e nome usado em referências (project, tracker, status, etc.).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdName {
    /// ID do recurso.
    pub id: RedmineId,
    /// Nome do recurso.
    pub name: String,
}

/// Valor de campo personalizado.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomFieldValue {
    /// ID do campo personalizado.
    pub id: RedmineId,
    /// Nome do campo personalizado.
    pub name: Option<String>,
    /// Valor do campo personalizado.
    pub value: Option<serde_json::Value>,
}

/// Payload para definir um campo personalizado em criação/atualização.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomFieldPayload {
    /// ID do campo personalizado.
    pub id: RedmineId,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Valor a ser definido no campo personalizado.
    pub value: Option<serde_json::Value>,
}

/// Token de upload retornado pelo Redmine após enviar um arquivo.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadToken {
    /// Token do upload retornado pelo Redmine.
    pub token: String,
}

/// Referência a um upload em payloads de criação/atualização.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadPayload {
    /// Token do upload.
    pub token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Nome do arquivo enviado.
    pub filename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Tipo de conteúdo (MIME) do arquivo.
    pub content_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Descrição opcional do arquivo.
    pub description: Option<String>,
}

/// Resposta de erro da API.
#[derive(Debug, Clone, Deserialize)]
pub struct ErrorResponse {
    /// Lista de mensagens de erro retornadas pela API.
    pub errors: Vec<String>,
}
