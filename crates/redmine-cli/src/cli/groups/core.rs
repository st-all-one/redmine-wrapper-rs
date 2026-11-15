//! Grupos de recursos centrais: issues, projects, users e time entries.

use redmine_wrapper::HttpMethod;

use crate::cli::spec::{ActionDef, BodySpec, ContextFlag, Extract, GroupDef, ParamsMode, PathSpec};

/// Grupo `issues`.
pub const ISSUES: GroupDef = GroupDef {
    name: "issues",
    about: "Issues: listagem, leitura, CRUD, watchers e status permitidos",
    actions: &[
        ActionDef::read_paginated(
            "list",
            "Lista issues (filtros em --params)",
            PathSpec::Static("/issues.json"),
            &[],
            "issues",
        ),
        ActionDef::read(
            "get",
            "Obtém issues por --id (repetível/vírgula)",
            PathSpec::Target("/issues/{id}.json"),
            &[ContextFlag::Ids],
            Extract::Path(&["issue"]),
        ),
        ActionDef::write(
            "create",
            "Cria uma issue (corpo em --params/stdin)",
            HttpMethod::Post,
            PathSpec::Static("/issues.json"),
            &[],
            BodySpec::Wrapped("issue"),
            Extract::Path(&["issue"]),
        ),
        ActionDef::write(
            "update",
            "Atualiza uma issue",
            HttpMethod::Put,
            PathSpec::Target("/issues/{id}.json"),
            &[ContextFlag::Id],
            BodySpec::Wrapped("issue"),
            Extract::Whole,
        ),
        ActionDef::delete(
            "delete",
            "Exclui uma issue",
            PathSpec::Target("/issues/{id}.json"),
            &[ContextFlag::Id],
        ),
        ActionDef::write(
            "add-watcher",
            "Adiciona um observador à issue",
            HttpMethod::Post,
            PathSpec::Target("/issues/{id}/watchers.json"),
            &[ContextFlag::Id, ContextFlag::User],
            BodySpec::UserId,
            Extract::Whole,
        )
        .with_params(ParamsMode::None),
        ActionDef::delete(
            "remove-watcher",
            "Remove um observador da issue",
            PathSpec::Target("/issues/{id}/watchers/{user}.json"),
            &[ContextFlag::Id, ContextFlag::User],
        ),
        ActionDef::read(
            "statuses",
            "Lista os status permitidos para transição",
            PathSpec::Target("/issues/{id}.json"),
            &[ContextFlag::Id],
            Extract::Path(&["issue", "allowed_statuses"]),
        )
        .with_fixed_query(&[("include", "allowed_statuses")]),
    ],
};

/// Grupo `projects`.
pub const PROJECTS: GroupDef = GroupDef {
    name: "projects",
    about: "Projetos: listagem, leitura, CRUD e arquivamento",
    actions: &[
        ActionDef::read_paginated(
            "list",
            "Lista projetos (filtros em --params)",
            PathSpec::Static("/projects.json"),
            &[],
            "projects",
        ),
        ActionDef::read(
            "get",
            "Obtém projetos por --id (repetível/vírgula)",
            PathSpec::Target("/projects/{id}.json"),
            &[ContextFlag::Ids],
            Extract::Path(&["project"]),
        ),
        ActionDef::write(
            "create",
            "Cria um projeto",
            HttpMethod::Post,
            PathSpec::Static("/projects.json"),
            &[],
            BodySpec::Wrapped("project"),
            Extract::Path(&["project"]),
        ),
        ActionDef::write(
            "update",
            "Atualiza um projeto",
            HttpMethod::Put,
            PathSpec::Target("/projects/{id}.json"),
            &[ContextFlag::Id],
            BodySpec::Wrapped("project"),
            Extract::Whole,
        ),
        ActionDef::delete(
            "delete",
            "Exclui um projeto",
            PathSpec::Target("/projects/{id}.json"),
            &[ContextFlag::Id],
        ),
        ActionDef::write(
            "archive",
            "Arquiva um projeto",
            HttpMethod::Put,
            PathSpec::Target("/projects/{id}/archive.json"),
            &[ContextFlag::Id],
            BodySpec::Empty,
            Extract::Whole,
        )
        .with_params(ParamsMode::None),
        ActionDef::write(
            "unarchive",
            "Desarquiva um projeto",
            HttpMethod::Put,
            PathSpec::Target("/projects/{id}/unarchive.json"),
            &[ContextFlag::Id],
            BodySpec::Empty,
            Extract::Whole,
        )
        .with_params(ParamsMode::None),
    ],
};

/// Grupo `users`.
pub const USERS: GroupDef = GroupDef {
    name: "users",
    about: "Usuários: listagem, leitura, conta atual e CRUD",
    actions: &[
        ActionDef::read_paginated(
            "list",
            "Lista usuários (filtros em --params)",
            PathSpec::Static("/users.json"),
            &[],
            "users",
        ),
        ActionDef::read(
            "get",
            "Obtém usuários por --id (repetível/vírgula)",
            PathSpec::Target("/users/{id}.json"),
            &[ContextFlag::Ids],
            Extract::Path(&["user"]),
        ),
        ActionDef::read(
            "me",
            "Retorna o usuário autenticado",
            PathSpec::Static("/my/account.json"),
            &[],
            Extract::Path(&["user"]),
        ),
        ActionDef::write(
            "create",
            "Cria um usuário",
            HttpMethod::Post,
            PathSpec::Static("/users.json"),
            &[],
            BodySpec::Wrapped("user"),
            Extract::Path(&["user"]),
        ),
        ActionDef::write(
            "update",
            "Atualiza um usuário",
            HttpMethod::Put,
            PathSpec::Target("/users/{id}.json"),
            &[ContextFlag::Id],
            BodySpec::Wrapped("user"),
            Extract::Whole,
        ),
        ActionDef::delete(
            "delete",
            "Exclui um usuário",
            PathSpec::Target("/users/{id}.json"),
            &[ContextFlag::Id],
        ),
    ],
};

/// Grupo `time-entries`.
pub const TIME_ENTRIES: GroupDef = GroupDef {
    name: "time-entries",
    about: "Apontamentos de horas: listagem, leitura e CRUD",
    actions: &[
        ActionDef::read_paginated(
            "list",
            "Lista apontamentos (filtros em --params)",
            PathSpec::Static("/time_entries.json"),
            &[],
            "time_entries",
        ),
        ActionDef::read(
            "get",
            "Obtém apontamentos por --id (repetível/vírgula)",
            PathSpec::Target("/time_entries/{id}.json"),
            &[ContextFlag::Ids],
            Extract::Path(&["time_entry"]),
        ),
        ActionDef::write(
            "create",
            "Cria um apontamento",
            HttpMethod::Post,
            PathSpec::Static("/time_entries.json"),
            &[],
            BodySpec::Wrapped("time_entry"),
            Extract::Path(&["time_entry"]),
        ),
        ActionDef::write(
            "update",
            "Atualiza um apontamento",
            HttpMethod::Put,
            PathSpec::Target("/time_entries/{id}.json"),
            &[ContextFlag::Id],
            BodySpec::Wrapped("time_entry"),
            Extract::Whole,
        ),
        ActionDef::delete(
            "delete",
            "Exclui um apontamento",
            PathSpec::Target("/time_entries/{id}.json"),
            &[ContextFlag::Id],
        ),
    ],
};
