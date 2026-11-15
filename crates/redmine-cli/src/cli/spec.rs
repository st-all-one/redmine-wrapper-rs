//! Contrato declarativo dos comandos: cada ação vira uma linha na tabela do grupo.
//!
//! A regra de posicional segue o knudge (D140): o posicional é **conteúdo**
//! (hoje só a consulta de `search`); identificadores e escopos são flags
//! (`--id`, `--project`, `--title`, `--user`). Corpo e filtros vêm de
//! `--params`/stdin; lotes vêm de `--batch`.

use redmine_wrapper::HttpMethod;

/// Metadado que a ação aceita como flag nomeada.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContextFlag {
    /// `--id <ID>` único (mutações).
    Id,
    /// `--id <ID>...` repetível e separado por vírgula (leituras).
    Ids,
    /// `--project <ID|IDENTIFICADOR>` obrigatório.
    Project,
    /// `--project <ID|IDENTIFICADOR>` opcional (escopo global ou de projeto).
    OptionalProject,
    /// `--title <TITULO>` (wiki).
    Title,
    /// `--user <ID>` (watchers e membros de grupo).
    User,
    /// `--file <ARQUIVO>` (upload).
    File,
    /// `--filename <NOME>` (upload).
    Filename,
    /// `--description <TEXTO>` (arquivo de projeto).
    Description,
    /// `--version <N>` (versão de página wiki).
    Version,
    /// `--reassign-to <ID>` (remoção de categoria).
    ReassignTo,
    /// `--method <GET|POST|PUT|PATCH|DELETE>` (api).
    Method,
    /// `--path </caminho.json>` (api).
    Path,
}

/// Forma de resolução do caminho do endpoint.
#[derive(Debug, Clone, Copy)]
pub enum PathSpec {
    /// Caminho fixo.
    Static(&'static str),
    /// Caminho com placeholders (`{id}`, `{project}`, `{title}`, `{user}`, `{version}`).
    Target(&'static str),
    /// Lista global ou escopada por `--project`.
    OptionalScope {
        /// Caminho sem projeto.
        global: &'static str,
        /// Caminho com `{project}`.
        scoped: &'static str,
    },
}

/// Como `--params` é consumido.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParamsMode {
    /// A operação não aceita `--params`.
    None,
    /// Objeto vira query string (leituras e remoções).
    Query,
    /// Objeto vira corpo (criações e atualizações), embrulhado por `BodySpec`.
    Body,
    /// `api`: query em GET/DELETE, corpo nos demais métodos.
    Api,
}

/// Corpo da requisição.
#[derive(Debug, Clone, Copy)]
pub enum BodySpec {
    /// Sem corpo.
    None,
    /// Corpo vazio `{}` (archive/unarchive).
    Empty,
    /// `--params` embrulhado na chave raiz do endpoint (ex.: `{"issue": {...}}`).
    Wrapped(&'static str),
    /// Corpo `{"user_id": <--user>}`.
    UserId,
    /// Corpo JSON fixo (ex.: limpar notas de journal).
    Fixed(&'static str),
    /// Corpo JSON bruto (api).
    Raw,
}

/// Extração do valor útil da resposta.
#[derive(Debug, Clone, Copy)]
pub enum Extract {
    /// Resposta inteira.
    Whole,
    /// Caminho de chaves (ex.: `["issue"]`, `["issue", "allowed_statuses"]`).
    Path(&'static [&'static str]),
    /// Lista paginada; o valor é a chave do array.
    Paginated(&'static str),
}

/// Execução especial que não é um simples verbo JSON.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Special {
    /// Verbo CRUD comum.
    None,
    /// Upload binário para `/uploads.json` (retorna token).
    AttachmentUpload,
    /// Upload binário + associação a `/projects/{project}/files.json`.
    FileUpload,
    /// Busca textual com consulta posicional.
    Search,
    /// Requisição JSON arbitrária.
    Api,
}

/// Definição declarativa de uma ação de grupo.
#[derive(Debug, Clone, Copy)]
pub struct ActionDef {
    /// Nome do subcomando.
    pub name: &'static str,
    /// Descrição curta exibida no `--help`.
    pub about: &'static str,
    /// Método HTTP (ignorado em `api`, que o recebe por flag).
    pub method: HttpMethod,
    /// Caminho do endpoint.
    pub path: PathSpec,
    /// Flags de contexto aceitas.
    pub context: &'static [ContextFlag],
    /// Semântica de `--params`.
    pub params: ParamsMode,
    /// Corpo da requisição.
    pub body: BodySpec,
    /// Extração da resposta.
    pub extract: Extract,
    /// Query fixa acrescentada (ex.: `include=allowed_statuses`).
    pub fixed_query: &'static [(&'static str, &'static str)],
    /// Se aceita `--batch`.
    pub batch: bool,
    /// Execução especial.
    pub special: Special,
}

impl ActionDef {
    /// Leitura de um valor único.
    #[must_use]
    pub const fn read(
        name: &'static str,
        about: &'static str,
        path: PathSpec,
        context: &'static [ContextFlag],
        extract: Extract,
    ) -> Self {
        Self {
            name,
            about,
            method: HttpMethod::Get,
            path,
            context,
            params: ParamsMode::Query,
            body: BodySpec::None,
            extract,
            fixed_query: &[],
            batch: true,
            special: Special::None,
        }
    }

    /// Leitura paginada (lista).
    #[must_use]
    pub const fn read_paginated(
        name: &'static str,
        about: &'static str,
        path: PathSpec,
        context: &'static [ContextFlag],
        list_key: &'static str,
    ) -> Self {
        Self::read(name, about, path, context, Extract::Paginated(list_key))
    }

    /// Criação ou atualização com corpo.
    #[must_use]
    pub const fn write(
        name: &'static str,
        about: &'static str,
        method: HttpMethod,
        path: PathSpec,
        context: &'static [ContextFlag],
        body: BodySpec,
        extract: Extract,
    ) -> Self {
        Self {
            name,
            about,
            method,
            path,
            context,
            params: ParamsMode::Body,
            body,
            extract,
            fixed_query: &[],
            batch: true,
            special: Special::None,
        }
    }

    /// Remoção.
    #[must_use]
    pub const fn delete(
        name: &'static str,
        about: &'static str,
        path: PathSpec,
        context: &'static [ContextFlag],
    ) -> Self {
        Self {
            name,
            about,
            method: HttpMethod::Delete,
            path,
            context,
            params: ParamsMode::Query,
            body: BodySpec::None,
            extract: Extract::Whole,
            fixed_query: &[],
            batch: true,
            special: Special::None,
        }
    }

    /// Query fixa somada aos `--params`.
    #[must_use]
    pub const fn with_fixed_query(
        mut self,
        fixed: &'static [(&'static str, &'static str)],
    ) -> Self {
        self.fixed_query = fixed;
        self
    }

    /// Substitui a semântica de `--params`.
    #[must_use]
    pub const fn with_params(mut self, params: ParamsMode) -> Self {
        self.params = params;
        self
    }

    /// Desabilita `--batch` (operações binárias).
    #[must_use]
    pub const fn without_batch(mut self) -> Self {
        self.batch = false;
        self
    }
}

/// Grupo de ações (`issues`, `projects`, ...).
#[derive(Debug, Clone, Copy)]
pub struct GroupDef {
    /// Nome do subcomando de grupo.
    pub name: &'static str,
    /// Descrição curta.
    pub about: &'static str,
    /// Ações do grupo.
    pub actions: &'static [ActionDef],
}

/// Ação sintética de `search` (top-level, posicional = consulta).
pub const SEARCH: ActionDef = ActionDef {
    name: "search",
    about: "Busca textual no Redmine",
    method: HttpMethod::Get,
    path: PathSpec::Static("/search.json"),
    context: &[],
    params: ParamsMode::Query,
    body: BodySpec::None,
    extract: Extract::Paginated("results"),
    fixed_query: &[],
    batch: true,
    special: Special::Search,
};

/// Ação sintética de `api` (top-level, via de escape).
pub const API: ActionDef = ActionDef {
    name: "api",
    about: "Executa uma requisição JSON arbitrária",
    method: HttpMethod::Get,
    path: PathSpec::Static(""),
    context: &[ContextFlag::Method, ContextFlag::Path],
    params: ParamsMode::Api,
    body: BodySpec::Raw,
    extract: Extract::Whole,
    fixed_query: &[],
    batch: true,
    special: Special::Api,
};
