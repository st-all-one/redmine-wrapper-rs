//! Grupos de metadados: enumerações, trackers, status, categorias, membros, papéis e grupos.

use redmine_wrapper::HttpMethod;

use crate::cli::spec::{ActionDef, BodySpec, ContextFlag, Extract, GroupDef, ParamsMode, PathSpec};

/// Grupo `enumerations`.
pub const ENUMERATIONS: GroupDef = GroupDef {
    name: "enumerations",
    about: "Enumerações do Redmine",
    actions: &[
        ActionDef::read_paginated(
            "priorities",
            "Lista prioridades de issue",
            PathSpec::Static("/enumerations/issue_priorities.json"),
            &[],
            "issue_priorities",
        )
        .without_batch(),
        ActionDef::read_paginated(
            "activities",
            "Lista atividades de apontamento de horas",
            PathSpec::Static("/enumerations/time_entry_activities.json"),
            &[],
            "time_entry_activities",
        )
        .without_batch(),
        ActionDef::read_paginated(
            "document-categories",
            "Lista categorias de documento",
            PathSpec::Static("/enumerations/document_categories.json"),
            &[],
            "document_categories",
        )
        .without_batch(),
    ],
};

/// Grupo `trackers`.
pub const TRACKERS: GroupDef = GroupDef {
    name: "trackers",
    about: "Trackers disponíveis",
    actions: &[ActionDef::read_paginated(
        "list",
        "Lista trackers",
        PathSpec::Static("/trackers.json"),
        &[],
        "trackers",
    )
    .without_batch()],
};

/// Grupo `issue-statuses`.
pub const ISSUE_STATUSES: GroupDef = GroupDef {
    name: "issue-statuses",
    about: "Status de issue disponíveis",
    actions: &[ActionDef::read_paginated(
        "list",
        "Lista status de issue",
        PathSpec::Static("/issue_statuses.json"),
        &[],
        "issue_statuses",
    )
    .without_batch()],
};

/// Grupo `issue-categories`.
pub const ISSUE_CATEGORIES: GroupDef = GroupDef {
    name: "issue-categories",
    about: "Categorias de issue por projeto",
    actions: &[
        ActionDef::read_paginated(
            "list",
            "Lista categorias de um projeto (--project)",
            PathSpec::Target("/projects/{project}/issue_categories.json"),
            &[ContextFlag::Project],
            "issue_categories",
        ),
        ActionDef::read(
            "get",
            "Obtém categorias por --id (repetível/vírgula)",
            PathSpec::Target("/issue_categories/{id}.json"),
            &[ContextFlag::Ids],
            Extract::Path(&["issue_category"]),
        ),
        ActionDef::write(
            "create",
            "Cria uma categoria em um projeto (--project)",
            HttpMethod::Post,
            PathSpec::Target("/projects/{project}/issue_categories.json"),
            &[ContextFlag::Project],
            BodySpec::Wrapped("issue_category"),
            Extract::Path(&["issue_category"]),
        ),
        ActionDef::write(
            "update",
            "Atualiza uma categoria",
            HttpMethod::Put,
            PathSpec::Target("/issue_categories/{id}.json"),
            &[ContextFlag::Id],
            BodySpec::Wrapped("issue_category"),
            Extract::Whole,
        ),
        ActionDef::delete(
            "delete",
            "Exclui uma categoria (--reassign-to opcional)",
            PathSpec::Target("/issue_categories/{id}.json"),
            &[ContextFlag::Id, ContextFlag::ReassignTo],
        ),
    ],
};

/// Grupo `memberships`.
pub const MEMBERSHIPS: GroupDef = GroupDef {
    name: "memberships",
    about: "Associações de usuários/grupos a projetos",
    actions: &[
        ActionDef::read_paginated(
            "list",
            "Lista associações de um projeto (--project)",
            PathSpec::Target("/projects/{project}/memberships.json"),
            &[ContextFlag::Project],
            "memberships",
        ),
        ActionDef::read(
            "get",
            "Obtém associações por --id (repetível/vírgula)",
            PathSpec::Target("/memberships/{id}.json"),
            &[ContextFlag::Ids],
            Extract::Path(&["membership"]),
        ),
        ActionDef::write(
            "create",
            "Cria uma associação em um projeto (--project)",
            HttpMethod::Post,
            PathSpec::Target("/projects/{project}/memberships.json"),
            &[ContextFlag::Project],
            BodySpec::Wrapped("membership"),
            Extract::Path(&["membership"]),
        ),
        ActionDef::write(
            "update",
            "Atualiza uma associação",
            HttpMethod::Put,
            PathSpec::Target("/memberships/{id}.json"),
            &[ContextFlag::Id],
            BodySpec::Wrapped("membership"),
            Extract::Whole,
        ),
        ActionDef::delete(
            "delete",
            "Exclui uma associação",
            PathSpec::Target("/memberships/{id}.json"),
            &[ContextFlag::Id],
        ),
    ],
};

/// Grupo `roles`.
pub const ROLES: GroupDef = GroupDef {
    name: "roles",
    about: "Papéis disponíveis",
    actions: &[
        ActionDef::read_paginated(
            "list",
            "Lista papéis",
            PathSpec::Static("/roles.json"),
            &[],
            "roles",
        )
        .without_batch(),
        ActionDef::read(
            "get",
            "Obtém papéis por --id (repetível/vírgula)",
            PathSpec::Target("/roles/{id}.json"),
            &[ContextFlag::Ids],
            Extract::Path(&["role"]),
        ),
    ],
};

/// Grupo `groups`.
pub const GROUPS_GROUP: GroupDef = GroupDef {
    name: "groups",
    about: "Grupos de usuários",
    actions: &[
        ActionDef::read_paginated(
            "list",
            "Lista grupos",
            PathSpec::Static("/groups.json"),
            &[],
            "groups",
        ),
        ActionDef::read(
            "get",
            "Obtém grupos por --id (repetível/vírgula)",
            PathSpec::Target("/groups/{id}.json"),
            &[ContextFlag::Ids],
            Extract::Path(&["group"]),
        ),
        ActionDef::write(
            "create",
            "Cria um grupo",
            HttpMethod::Post,
            PathSpec::Static("/groups.json"),
            &[],
            BodySpec::Wrapped("group"),
            Extract::Path(&["group"]),
        ),
        ActionDef::write(
            "update",
            "Atualiza um grupo",
            HttpMethod::Put,
            PathSpec::Target("/groups/{id}.json"),
            &[ContextFlag::Id],
            BodySpec::Wrapped("group"),
            Extract::Whole,
        ),
        ActionDef::delete(
            "delete",
            "Exclui um grupo",
            PathSpec::Target("/groups/{id}.json"),
            &[ContextFlag::Id],
        ),
        ActionDef::write(
            "add-user",
            "Adiciona um usuário ao grupo",
            HttpMethod::Post,
            PathSpec::Target("/groups/{id}/users.json"),
            &[ContextFlag::Id, ContextFlag::User],
            BodySpec::UserId,
            Extract::Whole,
        )
        .with_params(ParamsMode::None),
        ActionDef::delete(
            "remove-user",
            "Remove um usuário do grupo",
            PathSpec::Target("/groups/{id}/users/{user}.json"),
            &[ContextFlag::Id, ContextFlag::User],
        ),
    ],
};
