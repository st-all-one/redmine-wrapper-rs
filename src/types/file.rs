// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};
use crate::types::base::{RedmineId, IdName};

/// Arquivo anexado a um projeto.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct File {
    /// Identificador único do arquivo.
    pub id: RedmineId,
    /// Nome original do arquivo.
    pub filename: Option<String>,
    /// Tamanho do arquivo em bytes.
    pub filesize: Option<u64>,
    /// Tipo MIME do conteúdo do arquivo.
    pub content_type: Option<String>,
    /// Descrição do arquivo.
    pub description: Option<String>,
    /// URL para acesso ao conteúdo do arquivo.
    pub content_url: Option<String>,
    /// Autor que fez o upload do arquivo.
    pub author: Option<IdName>,
    /// Data de criação do arquivo.
    pub created_on: Option<String>,
    /// Hash de integridade (digest) do arquivo.
    pub digest: Option<String>,
    /// URL para download do arquivo.
    pub download_url: Option<String>,
}

/// Payload para anexar um arquivo a um projeto.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFilePayload {
    /// Token obtido do upload prévio do arquivo (obrigatório).
    pub token: String,
    /// Nome do arquivo (obrigatório).
    pub filename: String,
    /// Tipo MIME do arquivo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// Descrição do arquivo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// ID da versão à qual o arquivo será associado.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<RedmineId>,
}
