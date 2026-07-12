// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};
use crate::types::base::{RedmineId, IdName};

/// Anexo retornado pela API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
    /// Identificador único do anexo.
    pub id: RedmineId,
    /// Nome do arquivo anexado.
    pub filename: Option<String>,
    /// Tamanho do arquivo em bytes.
    pub filesize: Option<u64>,
    /// Tipo de conteúdo (MIME type) do arquivo.
    pub content_type: Option<String>,
    /// Descrição do anexo.
    pub description: Option<String>,
    /// URL do conteúdo do anexo.
    pub content_url: Option<String>,
    /// Autor do anexo.
    pub author: Option<IdName>,
    /// Data de criação do anexo.
    pub created_on: Option<String>,
    /// Hash de integridade (digest) do arquivo.
    pub digest: Option<String>,
    /// URL para download do anexo.
    pub download_url: Option<String>,
}
