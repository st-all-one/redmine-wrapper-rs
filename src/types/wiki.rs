// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};
use crate::types::base::IdName;

/// Referência a uma página pai na hierarquia wiki (contém apenas título).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikiParent {
    /// Título da página pai.
    pub title: String,
}

/// Resumo de uma página wiki (listagem).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikiPageSummary {
    /// Título da página wiki.
    pub title: String,
    /// Página pai na hierarquia wiki.
    pub parent: Option<WikiParent>,
    /// Número da versão atual da página.
    pub version: u32,
    /// Data de criação da página.
    pub created_on: Option<String>,
    /// Data da última atualização da página.
    pub updated_on: Option<String>,
}

/// Página wiki completa.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikiPage {
    /// Título da página wiki.
    pub title: Option<String>,
    /// Página pai na hierarquia wiki.
    pub parent: Option<WikiParent>,
    /// Conteúdo textual da página em formato wiki.
    pub text: Option<String>,
    /// Número da versão atual da página.
    pub version: Option<u32>,
    /// Autor da última atualização.
    pub author: Option<IdName>,
    /// Comentário associado à versão.
    pub comments: Option<String>,
    /// Data de criação da página.
    pub created_on: Option<String>,
    /// Data da última atualização da página.
    pub updated_on: Option<String>,
    /// Lista de anexos da página.
    pub attachments: Option<Vec<super::attachment::Attachment>>,
}

/// Payload para criação ou atualização de página wiki.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWikiPagePayload {
    /// Conteúdo da página em formato wiki (obrigatório).
    pub text: String,
    /// Comentário opcional sobre a alteração.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// Título da página pai (opcional, para hierarquia).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_title: Option<String>,
}
