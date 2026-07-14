// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::sync::Arc;

use serde::Deserialize;

use crate::core::errors::RedmineError;
use crate::http::client::HttpClient;
use crate::types::attachment::Attachment;
use crate::types::base::{RedmineId, UploadToken};

/// Recurso para operações com anexos e upload de arquivos.
#[derive(Debug)]
pub struct AttachmentsResource {
    http: Arc<HttpClient>,
}

impl AttachmentsResource {
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Retorna os detalhes de um anexo pelo ID.
    ///
    /// # Parâmetros
    /// - `id` — Identificador único do anexo
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// let attachment = client.attachments.get(5).await?;
    /// ```
    pub async fn get(&self, id: RedmineId) -> Result<Attachment, RedmineError> {
        let path = format!("/attachments/{}.json", id);
        self.http.get_single(&path, "attachment", &[], "attachments.get").await
    }

    /// Exclui um anexo.
    ///
    /// # Parâmetros
    /// - `id` — Identificador único do anexo a ser excluído
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// client.attachments.delete(5).await?;
    /// ```
    pub async fn delete(&self, id: RedmineId) -> Result<(), RedmineError> {
        let path = format!("/attachments/{}.json", id);
        self.http.delete(&path, &[], "attachments.delete").await
    }

    /// Faz upload de um arquivo e retorna o token para uso posterior.
    ///
    /// O processo de upload ocorre em duas etapas:
    /// 1. Envio do conteúdo binário para `/uploads.json` — esta função executa esta etapa.
    /// 2. Uso do token retornado ao associar o anexo a uma issue (`CreateIssuePayload.uploads`)
    ///    ou a um arquivo de projeto (`CreateFilePayload`).
    ///
    /// # Parâmetros
    /// - `filename` — Nome do arquivo (usado como parâmetro de query)
    /// - `data` — Conteúdo binário do arquivo
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// let token = client.attachments.upload("foto.png", &bytes).await?;
    /// ```
    pub async fn upload(&self, filename: &str, data: &[u8]) -> Result<String, RedmineError> {
        let path = format!("/uploads.json?filename={}", filename);
        let result: UploadTokenResponse = self.http.post_binary(&path, data, "application/octet-stream", "attachments.upload").await?;
        Ok(result.upload.token)
    }
}

#[derive(Debug, Deserialize)]
struct UploadTokenResponse {
    upload: UploadToken,
}
