//! Superfície da CLI: grupos, ações, flags globais e parsing tipado.

/// Tabelas declarativas dos grupos de recursos.
pub mod groups;
/// Tipos do contrato de comandos.
pub mod spec;

use std::path::PathBuf;

use clap::{Arg, ArgAction, ArgMatches, Command, value_parser};

use crate::cli::spec::{ActionDef, ContextFlag, GroupDef, ParamsMode};

/// Texto do rodapé que fixa o contrato de entrada/saída.
const AFTER_HELP: &str = "\
POSICIONAL: apenas conteúdo (hoje, `search <CONSULTA>`). Metadados são flags:
  --id, --project, --title, --user, --file, --method, --path.
PARAMS: --params '<JSON>' define filtros (leitura) ou corpo (escrita); `-` lê stdin.
  Em escritas, sem --params e com stdin não-TTY, o JSON vem do stdin.
LOTE: --batch <ARQUIVO|-> lê JSONL; cada linha é um objeto com
  id/project/title/user/path/query/params (chaves desconhecidas são erro).
SAÍDA: stdout = dados; --json = envelope {success,command,data,error?};
  lote = NDJSON, uma linha por item; EPIPE termina em 0.
EXIT: 0 ok, 2 uso, 3 não encontrado, 4 conflito, 5 rede/IO, 6 auth, 70 interno.";

/// Flags globais de conexão e saída.
#[derive(Debug, Clone)]
pub struct Globals {
    /// URL base do Redmine.
    pub url: Option<String>,
    /// Token via flag ou `REDMINE_TOKEN`.
    pub token: Option<String>,
    /// Arquivo com o token (tem precedência sobre `--token`).
    pub token_file: Option<PathBuf>,
    /// Timeout por requisição, em segundos.
    pub timeout: u64,
    /// Limite de requisições por segundo.
    pub max_rps: u32,
    /// Usuário para impersonação.
    pub switch_user: Option<String>,
    /// Emite o envelope JSON de máquina.
    pub json: bool,
}

/// Contexto (metadados) resolvido de uma ação.
#[derive(Debug, Clone, Default)]
pub struct Context {
    /// IDs alvo (`--id`).
    pub ids: Vec<String>,
    /// Projeto (`--project`), aceita ID ou identificador.
    pub project: Option<String>,
    /// Título de página wiki (`--title`).
    pub title: Option<String>,
    /// Usuário (`--user`).
    pub user: Option<String>,
    /// Arquivo de upload (`--file`).
    pub file: Option<PathBuf>,
    /// Nome do arquivo no upload (`--filename`).
    pub filename: Option<String>,
    /// Descrição do arquivo (`--description`).
    pub description: Option<String>,
    /// Versão da página wiki (`--version`).
    pub version: Option<u32>,
    /// Categoria de destino ao remover (`--reassign-to`).
    pub reassign_to: Option<String>,
    /// Método HTTP (`--method`, apenas `api`).
    pub method: Option<String>,
    /// Caminho (`--path`, apenas `api`).
    pub path: Option<String>,
    /// Consulta textual posicional (`search`).
    pub query: Option<String>,
}

/// Ação de um grupo de recurso.
#[derive(Debug)]
pub struct ActionInvocation {
    /// Nome do grupo.
    pub group: &'static str,
    /// Definição da ação.
    pub def: &'static ActionDef,
    /// Metadados resolvidos.
    pub context: Context,
    /// JSON bruto de `--params`.
    pub params: Option<String>,
    /// Fonte de `--batch`.
    pub batch: Option<String>,
    /// Não envia; imprime o plano.
    pub dry_run: bool,
}

/// Comando `search`.
#[derive(Debug)]
pub struct SearchInvocation {
    /// Contexto (consulta em `query`).
    pub context: Context,
    /// JSON bruto de `--params`.
    pub params: Option<String>,
    /// Fonte de `--batch`.
    pub batch: Option<String>,
    /// Não envia; imprime o plano.
    pub dry_run: bool,
}

/// Comando `api`.
#[derive(Debug)]
pub struct ApiInvocation {
    /// Contexto (`method` e `path`).
    pub context: Context,
    /// JSON bruto de `--params`.
    pub params: Option<String>,
    /// Fonte de `--batch`.
    pub batch: Option<String>,
    /// Não envia; imprime o plano.
    pub dry_run: bool,
}

/// Alvo resolvido do parsing.
#[derive(Debug)]
pub enum Target {
    /// Ação de um grupo.
    Action(ActionInvocation),
    /// Busca textual.
    Search(SearchInvocation),
    /// Requisição arbitrária.
    Api(ApiInvocation),
}

impl Target {
    /// Nome canônico usado no envelope.
    #[must_use]
    pub fn command_name(&self) -> String {
        match self {
            Self::Action(invocation) => format!("{}.{}", invocation.group, invocation.def.name),
            Self::Search(_) => "search".to_string(),
            Self::Api(_) => "api".to_string(),
        }
    }
}

/// Resultado do parsing.
#[derive(Debug)]
pub struct Parsed {
    /// Flags globais.
    pub globals: Globals,
    /// Alvo.
    pub target: Target,
}

/// Monta o `clap::Command` completo.
#[must_use]
pub fn command() -> Command {
    let mut root = Command::new("redmine")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Consulta e age na API REST do Redmine")
        .after_help(AFTER_HELP)
        .arg_required_else_help(true)
        .subcommand_required(true);
    root = root.args(global_args());
    for group in groups::GROUPS {
        root = root.subcommand(group_command(group));
    }
    root.subcommand(search_command()).subcommand(api_command())
}

/// Faz o parsing dos argumentos do processo.
///
/// # Errors
/// Devolve o erro do `clap` para que `main` decida o canal e o exit code.
pub fn parse() -> Result<Parsed, clap::Error> {
    let matches = command().try_get_matches()?;
    let globals = Globals {
        url: matches.get_one::<String>("url").cloned(),
        token: matches.get_one::<String>("token").cloned(),
        token_file: matches.get_one::<PathBuf>("token-file").cloned(),
        timeout: matches.get_one::<u64>("timeout").copied().unwrap_or(30),
        max_rps: matches.get_one::<u32>("max-rps").copied().unwrap_or(10),
        switch_user: matches.get_one::<String>("switch-user").cloned(),
        json: matches.get_flag("json"),
    };

    let (name, sub) = matches.subcommand().expect("subcomando obrigatório");
    let target = match name {
        "search" => Target::Search(SearchInvocation {
            context: search_context(sub),
            params: optional_string(sub, "params"),
            batch: optional_string(sub, "batch"),
            dry_run: flag(sub, "dry-run"),
        }),
        "api" => Target::Api(ApiInvocation {
            context: Context::from_matches(sub),
            params: optional_string(sub, "params"),
            batch: optional_string(sub, "batch"),
            dry_run: flag(sub, "dry-run"),
        }),
        _ => {
            let group = groups::find(name).expect("grupo registrado");
            let (action_name, action) = sub.subcommand().expect("ação obrigatória");
            let def = group
                .actions
                .iter()
                .find(|action| action.name == action_name)
                .expect("ação registrada");
            Target::Action(ActionInvocation {
                group: group.name,
                def,
                context: Context::from_matches(action),
                params: optional_string(action, "params"),
                batch: optional_string(action, "batch"),
                dry_run: flag(action, "dry-run"),
            })
        }
    };

    Ok(Parsed { globals, target })
}

impl Context {
    fn from_matches(matches: &ArgMatches) -> Self {
        Self {
            ids: strings(matches, "id"),
            project: optional_string(matches, "project"),
            title: optional_string(matches, "title"),
            user: optional_string(matches, "user"),
            file: optional_path(matches, "file"),
            filename: optional_string(matches, "filename"),
            description: optional_string(matches, "description"),
            version: optional_u32(matches, "version"),
            reassign_to: optional_string(matches, "reassign-to"),
            method: optional_string(matches, "method"),
            path: optional_string(matches, "path"),
            query: None,
        }
    }
}

fn optional_string(matches: &ArgMatches, id: &str) -> Option<String> {
    matches.try_get_one::<String>(id).ok().flatten().cloned()
}

fn strings(matches: &ArgMatches, id: &str) -> Vec<String> {
    matches
        .try_get_many::<String>(id)
        .ok()
        .flatten()
        .map_or_else(Vec::new, |values| values.cloned().collect())
}

fn optional_path(matches: &ArgMatches, id: &str) -> Option<PathBuf> {
    matches.try_get_one::<PathBuf>(id).ok().flatten().cloned()
}

fn optional_u32(matches: &ArgMatches, id: &str) -> Option<u32> {
    matches.try_get_one::<u32>(id).ok().flatten().copied()
}

fn flag(matches: &ArgMatches, id: &str) -> bool {
    matches
        .try_get_one::<bool>(id)
        .ok()
        .flatten()
        .copied()
        .unwrap_or(false)
}

fn search_context(matches: &ArgMatches) -> Context {
    let parts = strings(matches, "query");
    let query = if parts.len() == 1 && parts.first().is_some_and(|part| part == "-") {
        Some("-".to_string())
    } else if parts.is_empty() {
        None
    } else {
        Some(parts.join(" "))
    };
    Context {
        query,
        ..Context::from_matches(matches)
    }
}

fn global_args() -> Vec<Arg> {
    vec![
        Arg::new("url")
            .long("url")
            .env("REDMINE_URL")
            .value_name("URL")
            .global(true)
            .help("URL base do Redmine (env: REDMINE_URL)"),
        Arg::new("token")
            .long("token")
            .env("REDMINE_TOKEN")
            .value_name("TOKEN")
            .global(true)
            .help("Chave de API (env: REDMINE_TOKEN; prefira --token-file)"),
        Arg::new("token-file")
            .long("token-file")
            .value_name("ARQUIVO")
            .value_parser(value_parser!(PathBuf))
            .global(true)
            .help("Lê a chave de API de um arquivo (precede --token)"),
        Arg::new("timeout")
            .long("timeout")
            .env("REDMINE_TIMEOUT")
            .value_name("SEGUNDOS")
            .value_parser(value_parser!(u64))
            .default_value("30")
            .global(true)
            .help("Timeout por requisição em segundos"),
        Arg::new("max-rps")
            .long("max-rps")
            .env("REDMINE_MAX_RPS")
            .value_name("N")
            .value_parser(value_parser!(u32))
            .default_value("10")
            .global(true)
            .help("Máximo de requisições por segundo"),
        Arg::new("switch-user")
            .long("switch-user")
            .value_name("LOGIN")
            .global(true)
            .help("Impersona o usuário (header X-Redmine-Switch-User)"),
        Arg::new("json")
            .long("json")
            .action(ArgAction::SetTrue)
            .global(true)
            .help("Emite o envelope JSON compacto"),
    ]
}

fn group_command(group: &GroupDef) -> Command {
    let mut command = Command::new(group.name)
        .about(group.about)
        .arg_required_else_help(true)
        .subcommand_required(true);
    for action in group.actions {
        command = command.subcommand(action_command(action));
    }
    command
}

fn action_command(def: &ActionDef) -> Command {
    let mut command = Command::new(def.name).about(def.about);
    for flag in def.context {
        command = command.arg(context_arg(*flag));
    }
    if !matches!(def.params, ParamsMode::None) {
        command = command.arg(params_arg());
    }
    if def.batch {
        command = command.arg(batch_arg());
    }
    command.arg(dry_run_arg())
}

fn search_command() -> Command {
    Command::new("search")
        .about("Busca textual no Redmine")
        .arg(
            Arg::new("query")
                .value_name("CONSULTA")
                .num_args(0..)
                .help("Consulta textual; `-` lê stdin; sem posicional e com pipe também lê stdin"),
        )
        .arg(params_arg())
        .arg(batch_arg())
        .arg(dry_run_arg())
}

fn api_command() -> Command {
    Command::new("api")
        .about("Executa uma requisição JSON arbitrária (via de escape)")
        .arg(context_arg(ContextFlag::Method))
        .arg(context_arg(ContextFlag::Path))
        .arg(params_arg())
        .arg(batch_arg())
        .arg(dry_run_arg())
}

fn context_arg(flag: ContextFlag) -> Arg {
    // Flags de metadados não são `required` no clap porque o lote pode
    // fornecê-las por linha; a validação ocorre ao resolver a requisição.
    match flag {
        ContextFlag::Id => Arg::new("id")
            .long("id")
            .value_name("ID")
            .help("ID do recurso"),
        ContextFlag::Ids => Arg::new("id")
            .long("id")
            .value_name("ID")
            .action(ArgAction::Append)
            .value_delimiter(',')
            .help("IDs (repetível ou separado por vírgula)"),
        ContextFlag::Project => Arg::new("project")
            .long("project")
            .value_name("PROJETO")
            .help("ID ou identificador do projeto"),
        ContextFlag::OptionalProject => Arg::new("project")
            .long("project")
            .value_name("PROJETO")
            .help("ID ou identificador do projeto (opcional)"),
        ContextFlag::Title => Arg::new("title")
            .long("title")
            .value_name("TITULO")
            .help("Título da página wiki"),
        ContextFlag::User => Arg::new("user")
            .long("user")
            .value_name("ID")
            .help("ID do usuário"),
        ContextFlag::File => Arg::new("file")
            .long("file")
            .value_name("ARQUIVO")
            .value_parser(value_parser!(PathBuf))
            .help("Arquivo local a enviar"),
        ContextFlag::Filename => Arg::new("filename")
            .long("filename")
            .value_name("NOME")
            .help("Nome do arquivo no Redmine (default: nome local)"),
        ContextFlag::Description => Arg::new("description")
            .long("description")
            .value_name("TEXTO")
            .help("Descrição do arquivo"),
        ContextFlag::Version => Arg::new("version")
            .long("version")
            .value_name("N")
            .value_parser(value_parser!(u32))
            .help("Número da versão da página wiki"),
        ContextFlag::ReassignTo => Arg::new("reassign-to")
            .long("reassign-to")
            .value_name("ID")
            .help("Categoria que recebe as issues ao remover"),
        ContextFlag::Method => Arg::new("method")
            .long("method")
            .value_name("METODO")
            .required(true)
            .value_parser(["GET", "POST", "PUT", "PATCH", "DELETE"])
            .ignore_case(true)
            .help("Método HTTP"),
        ContextFlag::Path => Arg::new("path")
            .long("path")
            .value_name("CAMINHO")
            .help("Caminho iniciado por `/` (sem query string)"),
    }
}

fn params_arg() -> Arg {
    Arg::new("params")
        .long("params")
        .value_name("JSON")
        .help("Objeto JSON de filtros (leitura) ou corpo (escrita); `-` lê stdin")
}

fn batch_arg() -> Arg {
    Arg::new("batch")
        .long("batch")
        .value_name("ARQUIVO")
        .help("Lote JSONL (`-` lê stdin); exclusivo com --params")
}

fn dry_run_arg() -> Arg {
    Arg::new("dry-run")
        .long("dry-run")
        .action(ArgAction::SetTrue)
        .help("Não envia: imprime a requisição resolvida")
}
