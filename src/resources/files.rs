// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::sync::Arc;

use crate::core::errors::RedmineError;
use crate::http::client::HttpClient;
use crate::types::base::RedmineId;
use crate::types::file::*;

/// Recurso para operações com arquivos de projetos.
#[derive(Debug)]
pub struct FilesResource {
    http: Arc<HttpClient>,
}

impl FilesResource {
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Lista arquivos de um projeto.
    ///
    /// # Parâmetros
    /// - `project_id` — Identificador do projeto
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// let files = client.files.list_by_project(1)?;
    /// ```
    #[must_use]
    pub fn list_by_project(&self, project_id: RedmineId) -> Result<Vec<File>, RedmineError> {
        let path = format!("/projects/{}/files.json", project_id);
        let (items, _total) = self.http.get_paginated(&path, "files", None, &[], "files.list_by_project")?;
        Ok(items)
    }

    /// Anexa um arquivo a um projeto usando token de upload.
    ///
    /// O token deve ser obtido previamente através de `AttachmentsResource::upload`.
    ///
    /// # Parâmetros
    /// - `project_id` — Identificador do projeto
    /// - `payload` — Dados do arquivo (token, nome do arquivo, descrição)
    ///
    /// # Exemplo
    /// ```rust,ignore
    /// let token = client.attachments.upload("relatorio.pdf", &bytes)?;
    /// let payload = CreateFilePayload { token, filename: "relatorio.pdf".into(), ..Default::default() };
    /// let file = client.files.attach_to_project(1, &payload)?;
    /// ```
    #[must_use]
    pub fn attach_to_project(&self, project_id: RedmineId, payload: &CreateFilePayload) -> Result<File, RedmineError> {
        let path = format!("/projects/{}/files.json", project_id);
        let body = serde_json::json!({ "file": payload });
        self.http.post_single(&path, "file", &body, "files.attach_to_project")
    }
}
