// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::sync::Arc;

use crate::http::client::HttpClient;
use crate::resources::*;

/// Agrupa todos os recursos disponíveis no cliente Redmine.
///
/// O acesso aos recursos é feito via `Deref` em `RedmineClient`.
#[derive(Debug)]
pub struct ResourceGroup {
    /// Recurso para operações com issues.
    pub issues: IssuesResource,
    /// Recurso para operações com projetos.
    pub projects: ProjectsResource,
    /// Recurso para operações com usuários.
    pub users: UsersResource,
    /// Recurso para operações com apontamentos de horas.
    pub time_entries: TimeEntriesResource,
    /// Recurso para operações com journals (histórico).
    pub journals: JournalsResource,
    /// Recurso para operações com relações entre issues.
    pub relations: RelationsResource,
    /// Recurso para operações com anexos e upload.
    pub attachments: AttachmentsResource,
    /// Recurso para operações com páginas wiki.
    pub wiki: WikiResource,
    /// Recurso para operações com versões.
    pub versions: VersionsResource,
    /// Recurso para operações com enumerações.
    pub enumerations: EnumerationsResource,
    /// Recurso para operações com trackers.
    pub trackers: TrackersResource,
    /// Recurso para operações com status de issue.
    pub issue_statuses: IssueStatusesResource,
    /// Recurso para operações com categorias de issue.
    pub issue_categories: IssueCategoriesResource,
    /// Recurso para operações com associações.
    pub memberships: MembershipsResource,
    /// Recurso para operações com papéis.
    pub roles: RolesResource,
    /// Recurso para operações com grupos.
    pub groups: GroupsResource,
    /// Recurso para operações com campos personalizados.
    pub custom_fields: CustomFieldsResource,
    /// Recurso para operações com consultas salvas.
    pub queries: QueriesResource,
    /// Recurso para operações com arquivos de projeto.
    pub files: FilesResource,
    /// Recurso para operações de busca textual.
    pub search: SearchResource,
    /// Recurso para operações com notícias.
    pub news: NewsResource,
    /// Recurso para operações com a conta do usuário.
    pub my_account: MyAccountResource,
}

impl ResourceGroup {
    /// Cria um novo grupo de recursos compartilhando o mesmo HTTP client.
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self {
            issues: IssuesResource::new(Arc::clone(&http)),
            projects: ProjectsResource::new(Arc::clone(&http)),
            users: UsersResource::new(Arc::clone(&http)),
            time_entries: TimeEntriesResource::new(Arc::clone(&http)),
            journals: JournalsResource::new(Arc::clone(&http)),
            relations: RelationsResource::new(Arc::clone(&http)),
            attachments: AttachmentsResource::new(Arc::clone(&http)),
            wiki: WikiResource::new(Arc::clone(&http)),
            versions: VersionsResource::new(Arc::clone(&http)),
            enumerations: EnumerationsResource::new(Arc::clone(&http)),
            trackers: TrackersResource::new(Arc::clone(&http)),
            issue_statuses: IssueStatusesResource::new(Arc::clone(&http)),
            issue_categories: IssueCategoriesResource::new(Arc::clone(&http)),
            memberships: MembershipsResource::new(Arc::clone(&http)),
            roles: RolesResource::new(Arc::clone(&http)),
            groups: GroupsResource::new(Arc::clone(&http)),
            custom_fields: CustomFieldsResource::new(Arc::clone(&http)),
            queries: QueriesResource::new(Arc::clone(&http)),
            files: FilesResource::new(Arc::clone(&http)),
            search: SearchResource::new(Arc::clone(&http)),
            news: NewsResource::new(Arc::clone(&http)),
            my_account: MyAccountResource::new(Arc::clone(&http)),
        }
    }
}
