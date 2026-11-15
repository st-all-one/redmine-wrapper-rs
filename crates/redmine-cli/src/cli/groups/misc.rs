//! Grupos diversos: campos personalizados, consultas, arquivos, notícias e conta.

use redmine_wrapper::HttpMethod;

use crate::cli::spec::{
    ActionDef, BodySpec, ContextFlag, Extract, GroupDef, ParamsMode, PathSpec, Special,
};

/// Grupo `custom-fields`.
pub const CUSTOM_FIELDS: GroupDef = GroupDef {
    name: "custom-fields",
    about: "Campos personalizados",
    actions: &[ActionDef::read_paginated(
        "list",
        "Lista campos personalizados",
        PathSpec::Static("/custom_fields.json"),
        &[],
        "custom_fields",
    )
    .without_batch()],
};

/// Grupo `queries`.
pub const QUERIES: GroupDef = GroupDef {
    name: "queries",
    about: "Consultas salvas",
    actions: &[ActionDef::read_paginated(
        "list",
        "Lista consultas salvas",
        PathSpec::Static("/queries.json"),
        &[],
        "queries",
    )
    .without_batch()],
};

/// Grupo `files`.
pub const FILES: GroupDef = GroupDef {
    name: "files",
    about: "Arquivos de projeto",
    actions: &[
        ActionDef::read_paginated(
            "list",
            "Lista arquivos de um projeto (--project)",
            PathSpec::Target("/projects/{project}/files.json"),
            &[ContextFlag::Project],
            "files",
        ),
        ActionDef {
            name: "upload",
            about: "Envia e anexa um arquivo a um projeto (--project e --file)",
            method: HttpMethod::Post,
            path: PathSpec::Static(""),
            context: &[
                ContextFlag::Project,
                ContextFlag::File,
                ContextFlag::Filename,
                ContextFlag::Description,
            ],
            params: ParamsMode::None,
            body: BodySpec::None,
            extract: Extract::Whole,
            fixed_query: &[],
            batch: false,
            special: Special::FileUpload,
        },
    ],
};

/// Grupo `news`.
pub const NEWS: GroupDef = GroupDef {
    name: "news",
    about: "Notícias: globais ou por projeto",
    actions: &[
        ActionDef::read_paginated(
            "list",
            "Lista notícias (--project opcional)",
            PathSpec::OptionalScope {
                global: "/news.json",
                scoped: "/projects/{project}/news.json",
            },
            &[ContextFlag::OptionalProject],
            "news",
        ),
        ActionDef::read(
            "get",
            "Obtém notícias por --id (repetível/vírgula)",
            PathSpec::Target("/news/{id}.json"),
            &[ContextFlag::Ids],
            Extract::Path(&["news"]),
        ),
        ActionDef::write(
            "create",
            "Cria uma notícia em um projeto (--project)",
            HttpMethod::Post,
            PathSpec::Target("/projects/{project}/news.json"),
            &[ContextFlag::Project],
            BodySpec::Wrapped("news"),
            Extract::Path(&["news"]),
        ),
        ActionDef::write(
            "update",
            "Atualiza uma notícia",
            HttpMethod::Put,
            PathSpec::Target("/news/{id}.json"),
            &[ContextFlag::Id],
            BodySpec::Wrapped("news"),
            Extract::Whole,
        ),
        ActionDef::delete(
            "delete",
            "Exclui uma notícia",
            PathSpec::Target("/news/{id}.json"),
            &[ContextFlag::Id],
        ),
    ],
};

/// Grupo `my-account`.
pub const MY_ACCOUNT: GroupDef = GroupDef {
    name: "my-account",
    about: "Conta autenticada",
    actions: &[ActionDef::read(
        "get",
        "Retorna a conta autenticada",
        PathSpec::Static("/my/account.json"),
        &[],
        Extract::Path(&["user"]),
    )
    .without_batch()],
};
