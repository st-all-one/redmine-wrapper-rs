// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::sync::Arc;

use crate::core::config::RedmineConfig;
use crate::core::errors::RedmineError;
use crate::http::client::HttpClient;
use crate::resources;

/// Cliente principal para a API REST do Redmine.
///
/// Cria-se uma instância via `RedmineClient::new(config)` e acessa-se
/// os recursos por meio dos campos (ex: `client.issues.list(...)`).
///
/// # Exemplo
///
/// ```rust,ignore
/// use redmine_wrapper::{RedmineClient, RedmineConfigBuilder};
///
/// let client = RedmineClient::new(
///     RedmineConfigBuilder::default()
///         .base_url("https://redmine.example.com")
///         .token("seu-api-key")
///         .build()?,
/// )?;
///
/// let issues = client.issues.list(None).await?;
/// ```
pub struct RedmineClient {
    /// Configuração resolvida do cliente.
    pub config: RedmineConfig,
    /// Recurso para operações com issues.
    pub issues: resources::IssuesResource,
    /// Recurso para operações com projetos.
    pub projects: resources::ProjectsResource,
    /// Recurso para operações com usuários.
    pub users: resources::UsersResource,
    /// Recurso para operações com apontamentos de horas.
    pub time_entries: resources::TimeEntriesResource,
    /// Recurso para operações com journals (histórico).
    pub journals: resources::JournalsResource,
    /// Recurso para operações com relações entre issues.
    pub relations: resources::RelationsResource,
    /// Recurso para operações com anexos e upload.
    pub attachments: resources::AttachmentsResource,
    /// Recurso para operações com páginas wiki.
    pub wiki: resources::WikiResource,
    /// Recurso para operações com versões.
    pub versions: resources::VersionsResource,
    /// Recurso para operações com enumerações.
    pub enumerations: resources::EnumerationsResource,
    /// Recurso para operações com trackers.
    pub trackers: resources::TrackersResource,
    /// Recurso para operações com status de issue.
    pub issue_statuses: resources::IssueStatusesResource,
    /// Recurso para operações com categorias de issue.
    pub issue_categories: resources::IssueCategoriesResource,
    /// Recurso para operações com associações.
    pub memberships: resources::MembershipsResource,
    /// Recurso para operações com papéis.
    pub roles: resources::RolesResource,
    /// Recurso para operações com grupos.
    pub groups: resources::GroupsResource,
    /// Recurso para operações com campos personalizados.
    pub custom_fields: resources::CustomFieldsResource,
    /// Recurso para operações com consultas salvas.
    pub queries: resources::QueriesResource,
    /// Recurso para operações com arquivos de projeto.
    pub files: resources::FilesResource,
    /// Recurso para operações de busca textual.
    pub search: resources::SearchResource,
    /// Recurso para operações com notícias.
    pub news: resources::NewsResource,
    /// Recurso para operações com a conta do usuário.
    pub my_account: resources::MyAccountResource,
}

impl RedmineClient {
    /// Cria um novo cliente Redmine a partir da configuração fornecida.
    ///
    /// Este é o principal ponto de entrada da biblioteca. A configuração
    /// inclui a URL base do Redmine, a chave de API (token) e parâmetros
    /// opcionais como timeout e limite de requisições por segundo.
    ///
    /// # Exemplo
    ///
    /// ```rust,ignore
    /// use redmine_wrapper::{RedmineClient, RedmineConfigBuilder};
    ///
    /// let client = RedmineClient::new(
    ///     RedmineConfigBuilder::default()
    ///         .base_url("https://redmine.example.com")
    ///         .token("sua-chave-api")
    ///         .build()?,
    /// )?;
    /// ```
    pub fn new(config: RedmineConfig) -> Result<Self, RedmineError> {
        if config.base_url.is_empty() {
            return Err(RedmineError::Config("base_url não pode estar vazia".into()));
        }
        let http = Arc::new(HttpClient::new(config.clone())?);
        Ok(Self {
            config,
            issues: resources::IssuesResource::new(Arc::clone(&http)),
            projects: resources::ProjectsResource::new(Arc::clone(&http)),
            users: resources::UsersResource::new(Arc::clone(&http)),
            time_entries: resources::TimeEntriesResource::new(Arc::clone(&http)),
            journals: resources::JournalsResource::new(Arc::clone(&http)),
            relations: resources::RelationsResource::new(Arc::clone(&http)),
            attachments: resources::AttachmentsResource::new(Arc::clone(&http)),
            wiki: resources::WikiResource::new(Arc::clone(&http)),
            versions: resources::VersionsResource::new(Arc::clone(&http)),
            enumerations: resources::EnumerationsResource::new(Arc::clone(&http)),
            trackers: resources::TrackersResource::new(Arc::clone(&http)),
            issue_statuses: resources::IssueStatusesResource::new(Arc::clone(&http)),
            issue_categories: resources::IssueCategoriesResource::new(Arc::clone(&http)),
            memberships: resources::MembershipsResource::new(Arc::clone(&http)),
            roles: resources::RolesResource::new(Arc::clone(&http)),
            groups: resources::GroupsResource::new(Arc::clone(&http)),
            custom_fields: resources::CustomFieldsResource::new(Arc::clone(&http)),
            queries: resources::QueriesResource::new(Arc::clone(&http)),
            files: resources::FilesResource::new(Arc::clone(&http)),
            search: resources::SearchResource::new(Arc::clone(&http)),
            news: resources::NewsResource::new(Arc::clone(&http)),
            my_account: resources::MyAccountResource::new(Arc::clone(&http)),
        })
    }

    /// Retorna uma referência à configuração resolvida do cliente.
    pub fn config(&self) -> &RedmineConfig {
        &self.config
    }
}

impl std::fmt::Debug for RedmineClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RedmineClient")
            .field("config", &self.config)
            .field("issues", &self.issues)
            .finish_non_exhaustive()
    }
}
