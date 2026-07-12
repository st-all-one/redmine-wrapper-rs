// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};

/// Resultado de uma busca textual.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    /// Identificador único do resultado.
    pub id: u64,
    /// Título do resultado encontrado.
    pub title: Option<String>,
    /// Tipo do resultado (issue, project, news, etc.).
    #[serde(rename = "type")]
    pub result_type: Option<String>,
    /// URL para acesso ao resultado.
    pub url: Option<String>,
    /// Descrição do resultado.
    pub description: Option<String>,
    /// Data/hora do resultado.
    pub datetime: Option<String>,
}

/// Filtros para busca textual.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchFilter {
    /// Termo de busca (obrigatório).
    #[serde(rename = "q")]
    pub query: String,
    /// Deslocamento para paginação dos resultados.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    /// Número máximo de resultados por página.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Escopo da busca (ex: "all", "my_project").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    /// Se verdadeiro, busca por todas as palavras (e não qualquer uma).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_words: Option<bool>,
    /// Se verdadeiro, busca apenas nos títulos.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub titles_only: Option<bool>,
    /// Incluir issues nos resultados.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issues: Option<bool>,
    /// Incluir notícias nos resultados.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub news: Option<bool>,
    /// Incluir documentos nos resultados.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents: Option<bool>,
    /// Incluir changesets (commits) nos resultados.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changesets: Option<bool>,
    /// Incluir páginas wiki nos resultados.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wiki_pages: Option<bool>,
    /// Incluir mensagens de fóruns nos resultados.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<bool>,
    /// Incluir projetos nos resultados.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projects: Option<bool>,
    /// Se verdadeiro, busca apenas em issues abertas.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_issues: Option<bool>,
    /// Filtro para anexos (nome do arquivo ou descrição).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<String>,
}
