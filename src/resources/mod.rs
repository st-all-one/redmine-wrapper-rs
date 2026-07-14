// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/// Operacoes com anexos: get, delete, upload.
pub mod attachments;
/// Operacoes com campos personalizados: list.
pub mod custom_fields;
/// Operacoes com enumeracoes: prioridades, atividades, categorias.
pub mod enumerations;
/// Operacoes com arquivos de projeto: list_by_project, attach_to_project.
pub mod files;
/// Operacoes com grupos: CRUD + add_user, remove_user.
pub mod groups;
/// Operacoes com categorias de issue: CRUD.
pub mod issue_categories;
/// Operacoes com status de issue: list.
pub mod issue_statuses;
/// Operacoes com issues: CRUD + watchers + includes.
pub mod issues;
/// Operacoes com journals: update, remove.
pub mod journals;
/// Operacoes com memberships: CRUD.
pub mod memberships;
/// Operacoes com a conta do usuario: get.
pub mod my_account;
/// Operacoes com noticias: CRUD.
pub mod news;
/// Operacoes com projetos: CRUD + archive, unarchive.
pub mod projects;
/// Operacoes com consultas salvas: list.
pub mod queries;
/// Operacoes com relacoes entre issues: CRUD.
pub mod relations;
/// Operacoes com papeis: list, get.
pub mod roles;
/// Operacoes de busca textual: search.
pub mod search;
/// Operacoes com apontamentos de horas: CRUD.
pub mod time_entries;
/// Operacoes com trackers: list.
pub mod trackers;
/// Operacoes com usuarios: CRUD + includes + get_current.
pub mod users;
/// Operacoes com versoes: CRUD.
pub mod versions;
/// Operacoes com wiki: CRUD + get_version.
pub mod wiki;

pub use attachments::AttachmentsResource;
pub use custom_fields::CustomFieldsResource;
pub use enumerations::EnumerationsResource;
pub use files::FilesResource;
pub use groups::GroupsResource;
pub use issue_categories::IssueCategoriesResource;
pub use issue_statuses::IssueStatusesResource;
pub use issues::IssuesResource;
pub use journals::JournalsResource;
pub use memberships::MembershipsResource;
pub use my_account::MyAccountResource;
pub use news::NewsResource;
pub use projects::ProjectsResource;
pub use queries::QueriesResource;
pub use relations::RelationsResource;
pub use roles::RolesResource;
pub use search::SearchResource;
pub use time_entries::TimeEntriesResource;
pub use trackers::TrackersResource;
pub use users::UsersResource;
pub use versions::VersionsResource;
pub use wiki::WikiResource;
