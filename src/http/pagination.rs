// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use serde::Deserialize;

/// Resposta paginada da API do Redmine.
#[derive(Debug, Clone, Deserialize)]
pub struct PaginatedResponse<T> {
    /// Lista de itens retornados na página atual.
    pub items: Vec<T>,
    /// Número total de itens disponíveis (considerando todas as páginas).
    pub total_count: u32,
    /// Número máximo de itens por página.
    pub limit: u32,
    /// Deslocamento (offset) do primeiro item retornado.
    pub offset: u32,
}

/// Parâmetros de paginação para filtros.
#[derive(Debug, Clone, Copy, Default)]
pub struct PaginationParams {
    /// Deslocamento (offset) para a consulta.
    pub offset: Option<u32>,
    /// Número máximo de itens a retornar.
    pub limit: Option<u32>,
}

impl PaginationParams {
    /// Cria um novo `PaginationParams` com os valores de deslocamento e limite.
    pub fn new(offset: u32, limit: u32) -> Self {
        Self {
            offset: Some(offset),
            limit: Some(limit),
        }
    }

    /// Converte os parâmetros de paginação em uma lista de pares chave-valor
    /// para serem usados como query string em requisições HTTP.
    pub fn to_query(&self) -> Vec<(&str, String)> {
        let mut params = Vec::new();
        if let Some(offset) = self.offset {
            params.push(("offset", offset.to_string()));
        }
        if let Some(limit) = self.limit {
            params.push(("limit", limit.to_string()));
        }
        params
    }
}
