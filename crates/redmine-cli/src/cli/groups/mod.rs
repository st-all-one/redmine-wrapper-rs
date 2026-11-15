//! Registro dos grupos de recursos.

/// Recursos de colaboração.
pub mod collab;
/// Recursos centrais.
pub mod core;
/// Recursos de metadados.
pub mod meta;
/// Recursos diversos.
pub mod misc;

use crate::cli::spec::GroupDef;

/// Grupos na ordem exibida no `--help`.
pub const GROUPS: &[GroupDef] = &[
    core::ISSUES,
    core::PROJECTS,
    core::USERS,
    core::TIME_ENTRIES,
    collab::JOURNALS,
    collab::RELATIONS,
    collab::ATTACHMENTS,
    collab::WIKI,
    collab::VERSIONS,
    meta::ENUMERATIONS,
    meta::TRACKERS,
    meta::ISSUE_STATUSES,
    meta::ISSUE_CATEGORIES,
    meta::MEMBERSHIPS,
    meta::ROLES,
    meta::GROUPS_GROUP,
    misc::CUSTOM_FIELDS,
    misc::QUERIES,
    misc::FILES,
    misc::NEWS,
    misc::MY_ACCOUNT,
];

/// Localiza um grupo pelo nome do subcomando.
#[must_use]
pub fn find(name: &str) -> Option<&'static GroupDef> {
    GROUPS.iter().find(|group| group.name == name)
}
