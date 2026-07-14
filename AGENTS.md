# redmine-wrapper-rs — Guia do Agente

## TL;DR

```bash
cargo build
cargo test
cargo clippy
cargo check --example demo
REDMINE_URL=https://redmine.example.com REDMINE_TOKEN=xxx cargo run --example demo
```

## Stack

- **Runtime async**: `tokio` (sync, time)
- **HTTP**: `reqwest` (async, rustls-tls)
- **Serialização**: `serde` + `serde_json`
- **Erros**: `thiserror` (enum-based, RFC 7807-aligned)
- **Logs**: `tracing` (structured, spans, `#[instrument]`)
- **Correlation IDs**: `uuid` v7
- **Rate limiting**: Sliding window manual (`tokio::sync::Mutex`)

## Arquitetura

```
redmine_wrapper (lib)
├── lib.rs              # Barrel: re-exports públicos
├── client/
│   └── mod.rs          # RedmineClient (22 resources como campos diretos)
├── core/
│   ├── mod.rs
│   ├── config.rs       # RedmineConfig (unificado), RedmineConfigBuilder
│   ├── errors.rs       # RedmineError (enum), ErrorCategory (12), ErrorContext
│   └── constants.rs    # DEFAULT_TIMEOUT, DEFAULT_MAX_RPS, etc.
├── http/
│   ├── mod.rs
│   ├── client.rs       # HttpClient async (reqwest wrapper, auth, rate-limit)
│   ├── pagination.rs   # PaginationParams, PaginatedResponse
│   └── rate_limiter.rs # SlidingWindow (tokio::sync::Mutex<VecDeque>)
├── types/              # 22 módulos de tipos serde
│   ├── mod.rs          # Barrel
│   ├── base.rs         # RedmineId, IdName, CustomFieldValue, etc.
│   ├── issue.rs, project.rs, user.rs, time_entry.rs, journal.rs
│   ├── relation.rs, attachment.rs, wiki.rs, version.rs
│   ├── enumeration.rs, tracker.rs, issue_status.rs
│   ├── issue_category.rs, membership.rs, role.rs, group.rs
│   ├── custom_field.rs, query.rs, file.rs, search.rs
│   ├── news.rs, my_account.rs
├── resources/          # 22 recursos (1 por domínio)
│   ├── mod.rs          # Barrel
│   ├── issues.rs, projects.rs, users.rs, time_entries.rs
│   ├── journals.rs, relations.rs, attachments.rs, wiki.rs
│   ├── versions.rs, enumerations.rs, trackers.rs
│   ├── issue_statuses.rs, issue_categories.rs, memberships.rs
│   ├── roles.rs, groups.rs, custom_fields.rs, queries.rs
│   ├── files.rs, search.rs, news.rs, my_account.rs
└── utils/
    ├── mod.rs
    └── query.rs        # filter_to_query helper
```

## Convenções de código

- MPL-2.0 header em todos os arquivos
- `snake_case` para funções e variáveis
- `CamelCase` para tipos e enums
- Todos os tipos `Debug, Clone, Serialize, Deserialize`
- `#[serde(rename_all = "snake_case")]` em tipos da API
- Campos opcionais: `Option<T>` com `#[serde(skip_serializing_if = "Option::is_none")]`
- Métodos públicos retornam `Result<T, RedmineError>` (async)
- Identificador de operação (ex: `"issues.list"`) para tracing/rastreio
- Documentação pública em português
- `unwrap()` proibido em produção (use `expect()` com mensagem)

## Dependências (prod)

| Crate | Motivo |
|-------|--------|
| `tokio` | Runtime async (sync, time) |
| `reqwest` | HTTP client async com TLS |
| `serde` + `serde_json` | Serialização JSON |
| `thiserror` | Macro de erro derive |
| `tracing` | Logging estruturado com spans |
| `uuid` (v7) | Correlation IDs |

## Testes

```bash
cargo test                      # todos os testes
cargo test --test errors_test   # testes de erro
cargo test --test client_test   # testes de integração (wiremock)
cargo test --test pagination_test # testes de paginação
cargo clippy                    # lints
```

## Exemplos

```bash
REDMINE_URL=https://redmine.example.com REDMINE_TOKEN=xxx cargo run --example demo
REDMINE_URL=https://redmine.example.com REDMINE_TOKEN=xxx cargo run --example check
```

## Endpoints (91 total, 22 resources)

| Resource | Endpoints | Status |
|----------|-----------|--------|
| IssuesResource | list, list_with_includes, get, get_with_includes, get_allowed_statuses, create, update, delete, add_watcher, remove_watcher | Stable |
| ProjectsResource | list, get, get_with_includes, create, update, delete, archive, unarchive | Stable |
| UsersResource | list, get, get_with_includes, get_current, get_current_user, create, update, delete | Stable |
| TimeEntriesResource | list, get, create, update, delete | Stable |
| AttachmentsResource | get, delete, upload, update | Beta |
| JournalsResource | update, remove | Alpha |
| RelationsResource | get, list_by_issue, create_on_issue, delete | Alpha |
| WikiResource | list, get, get_version, create_or_update, delete | Alpha |
| VersionsResource | get, list_by_project, create_on_project, update, delete | Alpha |
| EnumerationsResource | list_issue_priorities, list_time_entry_activities, list_document_categories | Alpha |
| TrackersResource | list | Alpha |
| IssueStatusesResource | list | Alpha |
| IssueCategoriesResource | list_by_project, get, create, update, delete | Alpha |
| MembershipsResource | list_by_project, get, create, update, delete | Alpha |
| RolesResource | list, get | Alpha |
| GroupsResource | list, get, get_with_includes, create, update, delete, add_user, remove_user | Alpha |
| CustomFieldsResource | list | Alpha |
| QueriesResource | list | Alpha |
| FilesResource | list_by_project, attach_to_project | Alpha |
| SearchResource | search | Alpha |
| NewsResource | list, list_by_project, get, get_with_includes, create, update, delete | Alpha |
| MyAccountResource | get | Alpha |
