// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};
use crate::types::base::{RedmineId, IdName};

/// Comentário em uma notícia.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsComment {
    /// Identificador único do comentário.
    pub id: RedmineId,
    /// Autor do comentário.
    pub author: Option<IdName>,
    /// Conteúdo do comentário.
    pub comments: Option<String>,
    /// Data de criação do comentário.
    pub created_on: Option<String>,
}

/// Notícia retornada pela API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct News {
    /// Identificador único da notícia.
    pub id: RedmineId,
    /// Projeto ao qual a notícia pertence.
    pub project: Option<IdName>,
    /// Autor da notícia.
    pub author: Option<IdName>,
    /// Título da notícia.
    pub title: Option<String>,
    /// Resumo da notícia.
    pub summary: Option<String>,
    /// Descrição completa da notícia.
    pub description: Option<String>,
    /// Data de criação da notícia.
    pub created_on: Option<String>,
    /// Data da última atualização da notícia.
    pub updated_on: Option<String>,
    /// Lista de comentários da notícia.
    pub comments: Option<Vec<NewsComment>>,
    /// Anexos da notícia.
    pub attachments: Option<Vec<super::attachment::Attachment>>,
}

/// Payload para criar uma notícia.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateNewsPayload {
    /// Título da notícia (obrigatório).
    pub title: String,
    /// Resumo da notícia.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// Descrição completa da notícia.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Payload para atualizar uma notícia.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateNewsPayload {
    /// Novo título da notícia.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Novo resumo da notícia.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// Nova descrição da notícia.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
