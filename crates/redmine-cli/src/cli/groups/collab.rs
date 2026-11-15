//! Grupos de colaboração: journals, relations, attachments, wiki e versions.

use redmine_wrapper::HttpMethod;

use crate::cli::spec::{
    ActionDef, BodySpec, ContextFlag, Extract, GroupDef, ParamsMode, PathSpec, Special,
};

/// Grupo `journals`.
pub const JOURNALS: GroupDef = GroupDef {
    name: "journals",
    about: "Histórico: atualizar ou limpar as notas de um journal",
    actions: &[
        ActionDef::write(
            "update",
            "Atualiza as notas de um journal",
            HttpMethod::Put,
            PathSpec::Target("/journals/{id}.json"),
            &[ContextFlag::Id],
            BodySpec::Wrapped("journal"),
            Extract::Whole,
        ),
        ActionDef::write(
            "remove",
            "Limpa as notas de um journal",
            HttpMethod::Put,
            PathSpec::Target("/journals/{id}.json"),
            &[ContextFlag::Id],
            BodySpec::Fixed(r#"{"journal":{"notes":""}}"#),
            Extract::Whole,
        )
        .with_params(ParamsMode::None),
    ],
};

/// Grupo `relations`.
pub const RELATIONS: GroupDef = GroupDef {
    name: "relations",
    about: "Relações entre issues",
    actions: &[
        ActionDef::read_paginated(
            "list",
            "Lista as relações de uma issue (--id da issue)",
            PathSpec::Target("/issues/{id}/relations.json"),
            &[ContextFlag::Id],
            "relations",
        ),
        ActionDef::read(
            "get",
            "Obtém relações por --id (repetível/vírgula)",
            PathSpec::Target("/relations/{id}.json"),
            &[ContextFlag::Ids],
            Extract::Path(&["relation"]),
        ),
        ActionDef::write(
            "create",
            "Cria uma relação em uma issue (--id da issue)",
            HttpMethod::Post,
            PathSpec::Target("/issues/{id}/relations.json"),
            &[ContextFlag::Id],
            BodySpec::Wrapped("relation"),
            Extract::Path(&["relation"]),
        ),
        ActionDef::delete(
            "delete",
            "Exclui uma relação",
            PathSpec::Target("/relations/{id}.json"),
            &[ContextFlag::Id],
        ),
    ],
};

/// Grupo `attachments`.
pub const ATTACHMENTS: GroupDef = GroupDef {
    name: "attachments",
    about: "Anexos: leitura, metadados, remoção e upload",
    actions: &[
        ActionDef::read(
            "get",
            "Obtém anexos por --id (repetível/vírgula)",
            PathSpec::Target("/attachments/{id}.json"),
            &[ContextFlag::Ids],
            Extract::Path(&["attachment"]),
        ),
        ActionDef::write(
            "update",
            "Atualiza os metadados de um anexo",
            HttpMethod::Patch,
            PathSpec::Target("/attachments/{id}.json"),
            &[ContextFlag::Id],
            BodySpec::Wrapped("attachment"),
            Extract::Path(&["attachment"]),
        ),
        ActionDef::delete(
            "delete",
            "Exclui um anexo",
            PathSpec::Target("/attachments/{id}.json"),
            &[ContextFlag::Id],
        ),
        ActionDef {
            name: "upload",
            about: "Envia um arquivo e devolve o token de upload",
            method: HttpMethod::Post,
            path: PathSpec::Static(""),
            context: &[ContextFlag::File, ContextFlag::Filename],
            params: ParamsMode::None,
            body: BodySpec::None,
            extract: Extract::Whole,
            fixed_query: &[],
            batch: false,
            special: Special::AttachmentUpload,
        },
    ],
};

/// Grupo `wiki`.
pub const WIKI: GroupDef = GroupDef {
    name: "wiki",
    about: "Páginas wiki: índice, leitura, versão, upsert e remoção",
    actions: &[
        ActionDef::read_paginated(
            "list",
            "Lista o índice wiki de um projeto (--project)",
            PathSpec::Target("/projects/{project}/wiki/index.json"),
            &[ContextFlag::Project],
            "wiki_pages",
        ),
        ActionDef::read(
            "get",
            "Obtém uma página wiki (--project e --title)",
            PathSpec::Target("/projects/{project}/wiki/{title}.json"),
            &[ContextFlag::Project, ContextFlag::Title],
            Extract::Path(&["wiki_page"]),
        ),
        ActionDef::read(
            "get-version",
            "Obtém uma versão específica da página (--version)",
            PathSpec::Target("/projects/{project}/wiki/{title}/{version}.json"),
            &[
                ContextFlag::Project,
                ContextFlag::Title,
                ContextFlag::Version,
            ],
            Extract::Path(&["wiki_page"]),
        ),
        ActionDef::write(
            "upsert",
            "Cria ou atualiza uma página wiki",
            HttpMethod::Put,
            PathSpec::Target("/projects/{project}/wiki/{title}.json"),
            &[ContextFlag::Project, ContextFlag::Title],
            BodySpec::Wrapped("wiki_page"),
            Extract::Whole,
        ),
        ActionDef::delete(
            "delete",
            "Exclui uma página wiki",
            PathSpec::Target("/projects/{project}/wiki/{title}.json"),
            &[ContextFlag::Project, ContextFlag::Title],
        ),
    ],
};

/// Grupo `versions`.
pub const VERSIONS: GroupDef = GroupDef {
    name: "versions",
    about: "Versões de projeto",
    actions: &[
        ActionDef::read_paginated(
            "list",
            "Lista as versões de um projeto (--project)",
            PathSpec::Target("/projects/{project}/versions.json"),
            &[ContextFlag::Project],
            "versions",
        ),
        ActionDef::read(
            "get",
            "Obtém versões por --id (repetível/vírgula)",
            PathSpec::Target("/versions/{id}.json"),
            &[ContextFlag::Ids],
            Extract::Path(&["version"]),
        ),
        ActionDef::write(
            "create",
            "Cria uma versão em um projeto (--project)",
            HttpMethod::Post,
            PathSpec::Target("/projects/{project}/versions.json"),
            &[ContextFlag::Project],
            BodySpec::Wrapped("version"),
            Extract::Path(&["version"]),
        ),
        ActionDef::write(
            "update",
            "Atualiza uma versão",
            HttpMethod::Put,
            PathSpec::Target("/versions/{id}.json"),
            &[ContextFlag::Id],
            BodySpec::Wrapped("version"),
            Extract::Whole,
        ),
        ActionDef::delete(
            "delete",
            "Exclui uma versão",
            PathSpec::Target("/versions/{id}.json"),
            &[ContextFlag::Id],
        ),
    ],
};
