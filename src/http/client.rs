// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::sync::Mutex;
use reqwest::blocking::{Client, Response};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde::de::DeserializeOwned;
use serde::Serialize;
use url::form_urlencoded::Serializer as UrlSerializer;
use crate::core::config::ResolvedConfig;
use crate::core::constants::DEFAULT_PAGINATION_LIMIT;
use crate::core::errors::{ErrorCategory, ErrorContext, RedmineError};
use crate::http::pagination::PaginationParams;
use crate::http::rate_limiter::SlidingWindow;

const USER_AGENT: &str = concat!("redmine-wrapper-rs/", env!("CARGO_PKG_VERSION"));

/// Cliente HTTP interno que gerencia autenticação, rate limiting e parsing.
#[derive(Debug)]
pub(crate) struct HttpClient {
    client: Client,
    config: ResolvedConfig,
    rate_limiter: Mutex<SlidingWindow>,
}

impl HttpClient {
    /// Cria um novo HttpClient a partir da config resolvida.
    pub fn new(config: ResolvedConfig) -> Result<Self, RedmineError> {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let client = Client::builder()
            .default_headers(headers)
            .timeout(config.timeout)
            .user_agent(USER_AGENT)
            .build()
            .map_err(|e| RedmineError::Config(format!("falha ao criar HTTP client: {e}")))?;

        Ok(Self {
            client,
            rate_limiter: Mutex::new(SlidingWindow::new(config.max_rps)),
            config,
        })
    }

    /// Executa uma requisição GET e desserializa a resposta JSON.
    ///
    /// # Parâmetros
    /// - `path` — caminho do endpoint (ex: `/issues.json`)
    /// - `query` — pares chave-valor para parâmetros de query string
    /// - `operation` — identificador da operação para logging e rastreamento
    pub fn get<T: DeserializeOwned>(&self, path: &str, query: &[(&str, String)], operation: &str) -> Result<T, RedmineError> {
        let url = self.build_url(path, query)?;
        self.execute(|| self.client.get(&url).headers(self.auth_headers()), operation)
    }

    /// Executa uma requisição POST com corpo JSON e desserializa a resposta.
    ///
    /// # Parâmetros
    /// - `path` — caminho do endpoint
    /// - `body` — dados a serem enviados como JSON
    /// - `operation` — identificador da operação para logging e rastreamento
    pub fn post<T: DeserializeOwned, B: Serialize>(&self, path: &str, body: &B, operation: &str) -> Result<T, RedmineError> {
        let url = self.build_url(path, &[])?;
        let json_body = serde_json::to_string(body)?;
        self.execute(|| self.client.post(&url).headers(self.auth_headers()).body(json_body.clone()), operation)
    }

    /// Executa uma requisição POST com dados binários (upload de arquivos).
    ///
    /// # Parâmetros
    /// - `path` — caminho do endpoint de upload
    /// - `data` — conteúdo binário do arquivo
    /// - `content_type` — tipo MIME do conteúdo (ex: `application/octet-stream`)
    /// - `operation` — identificador da operação para logging e rastreamento
    pub fn post_binary<T: DeserializeOwned>(&self, path: &str, data: &[u8], content_type: &str, operation: &str) -> Result<T, RedmineError> {
        let url = self.build_url(path, &[])?;
        self.execute(|| self.client.post(&url).headers(self.auth_headers()).header(CONTENT_TYPE, content_type).body(data.to_vec()), operation)
    }

    /// Executa uma requisição PUT com corpo JSON para atualizar um recurso.
    ///
    /// # Parâmetros
    /// - `path` — caminho do endpoint
    /// - `body` — dados atualizados a serem enviados como JSON
    /// - `operation` — identificador da operação para logging e rastreamento
    pub fn put<T: DeserializeOwned, B: Serialize>(&self, path: &str, body: &B, operation: &str) -> Result<T, RedmineError> {
        let url = self.build_url(path, &[])?;
        let json_body = serde_json::to_string(body)?;
        self.execute(|| self.client.put(&url).headers(self.auth_headers()).body(json_body.clone()), operation)
    }

    /// Executa uma GET e extrai um campo específico do JSON de resposta.
    ///
    /// Útil quando a API retorna envelopes como `{ "issue": { ... } }`.
    /// Extrai o valor da chave `key` e desserializa diretamente no tipo `T`.
    ///
    /// # Parâmetros
    /// - `path` — caminho do endpoint
    /// - `key` — nome do campo a ser extraído (ex: `"issue"`)
    /// - `query` — pares chave-valor para parâmetros de query string
    /// - `op` — identificador da operação para logging e rastreamento
    pub fn get_single<T: DeserializeOwned>(&self, path: &str, key: &str, query: &[(&str, String)], op: &str) -> Result<T, RedmineError> {
        let v: serde_json::Value = self.get(path, query, op)?;
        let inner = v.get(key).ok_or_else(|| RedmineError::api(ErrorCategory::ParseError, 200, format!("campo '{key}' não encontrado"), ErrorContext { operation: Some(op.into()), ..Default::default() }))?;
        serde_json::from_value(inner.clone()).map_err(RedmineError::from)
    }

    /// Executa uma POST e extrai um campo específico do JSON de resposta.
    ///
    /// Semelhante a `get_single`, mas para requisições de criação (POST).
    /// Extrai o valor da chave `key` do envelope de resposta e desserializa
    /// no tipo `T`.
    ///
    /// # Parâmetros
    /// - `path` — caminho do endpoint
    /// - `key` — nome do campo a ser extraído (ex: `"issue"`)
    /// - `body` — dados a serem enviados como JSON
    /// - `op` — identificador da operação para logging e rastreamento
    pub fn post_single<T: DeserializeOwned, B: Serialize>(&self, path: &str, key: &str, body: &B, op: &str) -> Result<T, RedmineError> {
        let v: serde_json::Value = self.post(path, body, op)?;
        let inner = v.get(key).ok_or_else(|| RedmineError::api(ErrorCategory::ParseError, 201, format!("campo '{key}' não encontrado"), ErrorContext { operation: Some(op.into()), ..Default::default() }))?;
        serde_json::from_value(inner.clone()).map_err(RedmineError::from)
    }

    /// Executa uma GET com suporte a paginação e retorna os itens da página
    /// juntamente com o total de registros disponíveis.
    ///
    /// # Parâmetros
    /// - `path` — caminho do endpoint
    /// - `item_key` — chave do array de itens no JSON de resposta (ex: `"issues"`)
    /// - `params` — parâmetros opcionais de paginação (limite e offset)
    /// - `query` — pares chave-valor adicionais para a query string (filtros)
    /// - `op` — identificador da operação para logging e rastreamento
    ///
    /// # Retorno
    /// Uma tupla com a lista de itens da página e o total de registros.
    pub fn get_paginated<T: DeserializeOwned>(&self, path: &str, item_key: &str, params: Option<&PaginationParams>, query: &[(&str, String)], op: &str) -> Result<(Vec<T>, u32), RedmineError> {
        let mut q = query.to_vec();
        if let Some(p) = params { for (k, v) in p.to_query() { q.push((k, v)); } } else { q.push(("limit", DEFAULT_PAGINATION_LIMIT.to_string())); }
        let v: serde_json::Value = self.get(path, &q, op)?;
        let total = v.get("total_count").and_then(|v| v.as_u64()).unwrap_or(0) as u32;
        let items = v.get(item_key).and_then(|v| serde_json::from_value(v.clone()).ok()).unwrap_or_default();
        Ok((items, total))
    }

    /// Executa requisições GET paginadas automaticamente até obter todos os
    /// registros. Útil quando não há limite de página e deseja-se coletar
    /// o conjunto completo de dados.
    ///
    /// Internamente faz requisições sucessivas incrementando o offset até
    /// que todos os registros tenham sido obtidos.
    ///
    /// # Parâmetros
    /// - `path` — caminho do endpoint
    /// - `item_key` — chave do array de itens no JSON de resposta (ex: `"issues"`)
    /// - `query` — pares chave-valor adicionais para a query string
    /// - `op` — identificador da operação para logging e rastreamento
    ///
    /// # Retorno
    /// Lista completa de todos os itens disponíveis.
    pub fn get_all_paginated<T: DeserializeOwned>(&self, path: &str, item_key: &str, query: &[(&str, String)], op: &str) -> Result<Vec<T>, RedmineError> {
        let limit = DEFAULT_PAGINATION_LIMIT;
        let mut offset = 0u32;
        let mut all = Vec::new();
        loop {
            let mut pq = query.to_vec();
            pq.push(("offset", offset.to_string()));
            pq.push(("limit", limit.to_string()));
            let v: serde_json::Value = self.get(path, &pq, op)?;
            let total = v.get("total_count").and_then(|v| v.as_u64()).unwrap_or(0) as u32;
            let items: Vec<T> = v.get(item_key).and_then(|v| serde_json::from_value(v.clone()).ok()).unwrap_or_default();
            let c = items.len() as u32;
            all.extend(items);
            if offset + c >= total { break; }
            offset += limit;
        }
        Ok(all)
    }

    /// Executa uma requisição DELETE para remover um recurso.
    ///
    /// # Parâmetros
    /// - `path` — caminho do endpoint do recurso a ser removido
    /// - `query` — pares chave-valor para parâmetros de query string
    /// - `operation` — identificador da operação para logging e rastreamento
    pub fn delete(&self, path: &str, query: &[(&str, String)], operation: &str) -> Result<(), RedmineError> {
        let url = self.build_url(path, query)?;
        self.execute_void(|| self.client.delete(&url).headers(self.auth_headers()), operation)
    }

    fn auth_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        if let Some(ref token) = self.config.token {
            headers.insert("X-Redmine-API-Key", HeaderValue::from_str(token).unwrap());
        }
        if let Some(ref switch_user) = self.config.switch_user {
            headers.insert(
                "X-Redmine-Switch-User",
                HeaderValue::from_str(switch_user).unwrap(),
            );
        }
        headers
    }

    fn build_url(&self, path: &str, query: &[(&str, String)]) -> Result<String, RedmineError> {
        let mut url = self.config.api_url(path);
        if !query.is_empty() {
            url.push('?');
            let mut serializer = UrlSerializer::new(&mut url);
            for (k, v) in query {
                serializer.append_pair(k, v);
            }
            serializer.finish();
        }
        Ok(url)
    }

    fn execute<T: DeserializeOwned>(
        &self,
        request_fn: impl Fn() -> reqwest::blocking::RequestBuilder,
        operation: &str,
    ) -> Result<T, RedmineError> {
        self.acquire_rate_limit();

        let response = request_fn().send().map_err(|e| {
            if e.is_timeout() {
                RedmineError::Timeout {
                    duration: self.config.timeout,
                    context: Box::new(ErrorContext {
                        operation: Some(operation.into()),
                        ..Default::default()
                    }),
                }
            } else {
                RedmineError::Http(e)
            }
        })?;

        Self::handle_response(response, operation)
    }

    fn execute_void(
        &self,
        request_fn: impl Fn() -> reqwest::blocking::RequestBuilder,
        operation: &str,
    ) -> Result<(), RedmineError> {
        self.acquire_rate_limit();

        let response = request_fn().send().map_err(|e| {
            if e.is_timeout() {
                RedmineError::Timeout {
                    duration: self.config.timeout,
                    context: Box::new(ErrorContext {
                        operation: Some(operation.into()),
                        ..Default::default()
                    }),
                }
            } else {
                RedmineError::Http(e)
            }
        })?;

        let status = response.status();
        if status.is_success() {
            return Ok(());
        }

        let body = response.text().unwrap_or_default();
        Err(Self::map_error(status.as_u16(), &body, operation))
    }

    fn handle_response<T: DeserializeOwned>(
        response: Response,
        operation: &str,
    ) -> Result<T, RedmineError> {
        let status = response.status();

        if status == 204 {
            return Err(RedmineError::api(
                ErrorCategory::ParseError,
                204,
                "resposta 204 sem corpo",
                ErrorContext {
                    operation: Some(operation.into()),
                    http_status: Some(204),
                    ..Default::default()
                },
            ));
        }

        let body = response.text().unwrap_or_default();

        if !status.is_success() {
            return Err(Self::map_error(status.as_u16(), &body, operation));
        }

        serde_json::from_str(&body).map_err(|e| {
            log::error!(
                target: "redmine_wrapper::http",
                "{}: falha ao desserializar resposta: {}",
                operation,
                e,
            );
            let ctx = ErrorContext {
                operation: Some(operation.into()),
                http_status: Some(status.as_u16()),
                response_body: Some(body),
                ..Default::default()
            };
            RedmineError::api(ErrorCategory::ParseError, status.as_u16(), e.to_string(), ctx)
        })
    }

    fn map_error(status: u16, body: &str, operation: &str) -> RedmineError {
        let category = ErrorCategory::from_status(status).unwrap_or(ErrorCategory::InternalError);

        let api_errors: Option<Vec<String>> = serde_json::from_str::<serde_json::Value>(body)
            .ok()
            .and_then(|v| {
                v.get("errors")
                    .and_then(|e| serde_json::from_value(e.clone()).ok())
            });

        let detail = api_errors
            .as_ref()
            .and_then(|e| e.first().cloned())
            .unwrap_or_else(|| match category {
                ErrorCategory::RateLimited => "limite de taxa excedido".into(),
                ErrorCategory::ImpersonationFailed => "impersonação inválida".into(),
                _ => format!("erro HTTP {status}"),
            });

        let context = ErrorContext {
            operation: Some(operation.into()),
            http_status: Some(status),
            api_errors,
            response_body: Some(body.into()),
            ..Default::default()
        };

        if category == ErrorCategory::RateLimited {
            let retry_after = serde_json::from_str::<serde_json::Value>(body)
                .ok()
                .and_then(|v| {
                    v.get("retry_after")
                        .and_then(|v| v.as_u64())
                });
            return RedmineError::RateLimited {
                retry_after,
                context: Box::new(context),
            };
        }

        RedmineError::api(category, status, detail, context)
    }

    fn acquire_rate_limit(&self) {
        if let Ok(mut limiter) = self.rate_limiter.lock() {
            limiter.acquire();
        }
    }
}


