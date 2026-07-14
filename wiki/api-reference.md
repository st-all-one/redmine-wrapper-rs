# Referência da API

## Visão Geral

```
redmine_wrapper (lib)
├── lib.rs              # Re-exports públicos
├── client/             # RedmineClient (22 resources como campos diretos)
├── core/               # Config, erros, constantes
├── http/               # HTTP client async, paginação, rate limiter
├── resources/          # 22 recursos (1 por domínio)
├── types/              # 22 módulos de tipos serde
└── utils/              # Utilitários (filtros, query helpers)
```

A biblioteca expõe 22 resources como campos públicos diretos no
`RedmineClient`. Cada resource contém métodos `async` que mapeiam
1:1 com endpoints da API do Redmine.

---

## Core Types

### `RedmineConfig`

```rust,ignore
pub struct RedmineConfig {
    pub base_url: String,                    // URL base (ex: https://redmine.exemplo.com)
    pub token: Option<String>,               // Chave de API
    pub auth_method: AuthMethod,             // Método de autenticação
    pub switch_user: Option<String>,         // Impersonação (admin)
    pub timeout: Duration,                   // Timeout HTTP (padrão: 30s)
    pub max_rps: u32,                        // Rate limiting (padrão: 10 req/s)
}
```

Construída via builder (`RedmineConfigBuilder`) ou diretamente:

```rust,ignore
use redmine_wrapper::core::config::RedmineConfig;

let config = RedmineConfig {
    base_url: "https://redmine.exemplo.com".into(),
    token: Some("sua-chave".into()),
    ..Default::default()
};
```

### `RedmineConfigBuilder`

```rust,ignore
pub struct RedmineConfigBuilder { /* campos privados */ }

impl RedmineConfigBuilder {
    pub fn base_url(self, url: impl Into<String>) -> Self;
    pub fn token(self, token: impl Into<String>) -> Self;
    pub fn auth_method(self, method: AuthMethod) -> Self;
    pub fn switch_user(self, user: impl Into<String>) -> Self;
    pub fn timeout_secs(self, secs: u64) -> Self;
    pub fn max_rps(self, rps: u32) -> Self;
    pub fn build(self) -> Result<RedmineConfig, RedmineError>;
}
```

```rust,ignore
use redmine_wrapper::RedmineConfigBuilder;

let config = RedmineConfigBuilder::default()
    .base_url("https://redmine.exemplo.com")
    .token("sua-chave")
    .build()?;
```

### `AuthMethod`

```rust,ignore
pub enum AuthMethod {
    Header,     // X-Redmine-API-Key (único suportado)
}
```

---

## Error Types

### `RedmineError`

```rust,ignore
pub enum RedmineError {
    Api {
        category: ErrorCategory,
        status: u16,
        detail: String,
        instance: String,           // UUID v7 para correlação
        context: Box<ErrorContext>,
    },
    Http(reqwest::Error),           // Erro de transporte
    RateLimited {
        retry_after: Option<u64>,
        context: Box<ErrorContext>,
    },
    Timeout {
        duration: Duration,
        context: Box<ErrorContext>,
    },
    Url(String),                    // URL inválida
    Serialization(serde_json::Error), // Erro JSON
    Config(String),                 // Erro de configuração
}
```

### `ErrorCategory` (12 variantes)

| Variante | HTTP | Descrição |
|----------|------|-----------|
| `AuthenticationFailed` | 401 | Chave ausente/inválida |
| `AuthorizationDenied` | 403 | Acesso negado |
| `ResourceNotFound` | 404 | Recurso não encontrado |
| `ValidationError` | 422 | Dados inválidos |
| `Conflict` | 409 | Conflito (ex: versão de wiki) |
| `RateLimited` | 429 | Taxa excedida |
| `ImpersonationFailed` | 412 | `X-Redmine-Switch-User` inválido |
| `UploadTooLarge` | 413 | Arquivo excede tamanho máximo |
| `Timeout` | 504 | Tempo limite excedido |
| `NetworkError` | 503 | Rede indisponível |
| `ParseError` | 500 | JSON inválido na resposta |
| `InternalError` | 500 | Erro não categorizado |

### `ErrorContext`

```rust,ignore
pub struct ErrorContext {
    pub operation: Option<String>,           // Ex: "issues.list"
    pub http_status: Option<u16>,            // Código HTTP
    pub api_errors: Option<Vec<String>>,     // Mensagens da API
    pub response_body: Option<String>,       // Corpo bruto da resposta
    pub extra: HashMap<String, String>,      // Metadados adicionais
}
```

---

## HTTP Client (internal)

### `HttpClient` (privado)

| Método | Descrição |
|--------|-----------|
| `get<T>(path, query, op)` | GET genérico, desserializa resposta |
| `post<T, B>(path, body, op)` | POST com corpo JSON |
| `put<T, B>(path, body, op)` | PUT com corpo JSON |
| `delete(path, query, op)` | DELETE, retorna `()` |
| `post_binary<T>(path, data, content_type, op)` | POST binário (upload) |
| `get_single<T>(path, key, query, op)` | GET + extração de campo do envelope |
| `post_single<T, B>(path, key, body, op)` | POST + extração de campo do envelope |
| `get_paginated<T>(path, item_key, params, op)` | GET paginado, retorna `(Vec<T>, u32)` |
| `get_all_paginated<T>(path, item_key, query, op)` | Auto-paginado (todas as páginas) |

Todos os métodos são `async` e usam `operation` como identificador
para tracing/rastreio (ex: `"issues.list"`, `"projects.get"`).

---

## Resources

### IssuesResource

| Método | Endpoint | Status |
|--------|----------|--------|
| `list(filter)` | `GET /issues.json` | Stable |
| `list_with_includes(filter, includes)` | `GET /issues.json?include=` | Stable |
| `get(id)` | `GET /issues/{id}.json` | Stable |
| `get_with_includes(id, includes)` | `GET /issues/{id}.json?include=` | Stable |
| `get_allowed_statuses(id)` | `GET /issues/{id}.json?include=allowed_statuses` | Stable |
| `create(request)` | `POST /issues.json` | Stable |
| `update(id, request)` | `PUT /issues/{id}.json` | Stable |
| `delete(id)` | `DELETE /issues/{id}.json` | Stable |
| `add_watcher(issue_id, user_id)` | `POST /issues/{id}/watchers.json` | Stable |
| `remove_watcher(issue_id, user_id)` | `DELETE /issues/{id}/watchers/{uid}.json` | Stable |

### ProjectsResource

| Método | Endpoint | Status |
|--------|----------|--------|
| `list()` | `GET /projects.json` | Stable |
| `get(id)` | `GET /projects/{id}.json` | Stable |
| `get_with_includes(id, includes)` | `GET /projects/{id}.json?include=` | Stable |
| `create(request)` | `POST /projects.json` | Stable |
| `update(id, request)` | `PUT /projects/{id}.json` | Stable |
| `delete(id)` | `DELETE /projects/{id}.json` | Stable |
| `archive(id)` | `POST /projects/{id}/archive.json` | Stable |
| `unarchive(id)` | `POST /projects/{id}/unarchive.json` | Stable |

### UsersResource

| Método | Endpoint | Status |
|--------|----------|--------|
| `list(filter)` | `GET /users.json` | Stable |
| `get(id)` | `GET /users/{id}.json` | Stable |
| `get_with_includes(id, includes)` | `GET /users/{id}.json?include=` | Stable |
| `get_current()` | `GET /my/account.json` (via `User`) | Stable |
| `get_current_user()` | `GET /users/current.json` | Alpha |
| `create(request)` | `POST /users.json` | Stable |
| `update(id, request)` | `PUT /users/{id}.json` | Stable |
| `delete(id)` | `DELETE /users/{id}.json` | Stable |

### TimeEntriesResource

| Método | Endpoint | Status |
|--------|----------|--------|
| `list(filter)` | `GET /time_entries.json` | Stable |
| `get(id)` | `GET /time_entries/{id}.json` | Stable |
| `create(request)` | `POST /time_entries.json` | Stable |
| `update(id, request)` | `PUT /time_entries/{id}.json` | Stable |
| `delete(id)` | `DELETE /time_entries/{id}.json` | Stable |

### AttachmentsResource

| Método | Endpoint | Status |
|--------|----------|--------|
| `get(id)` | `GET /attachments/{id}.json` | Beta |
| `delete(id)` | `DELETE /attachments/{id}.json` | Beta |
| `upload(filename, data)` | `POST /uploads.json` | Beta |
| `update(id, payload)` | `PATCH /attachments/{id}.json` | Alpha |

### JournalsResource

| Método | Endpoint | Status |
|--------|----------|--------|
| `update(id, request)` | `PUT /journals/{id}.json` | Alpha |
| `remove(id)` | `PUT /journals/{id}.json` | Alpha |

> Journals são obtidos exclusivamente via `GET /issues/{id}.json?include=journals`.
> Não há endpoint GET standalone para journals.

### RelationsResource

| Método | Endpoint | Status |
|--------|----------|--------|
| `get(id)` | `GET /relations/{id}.json` | Alpha |
| `list_by_issue(issue_id)` | `GET /issues/{id}/relations.json` | Alpha |
| `create_on_issue(issue_id, request)` | `POST /issues/{id}/relations.json` | Alpha |
| `delete(id)` | `DELETE /relations/{id}.json` | Alpha |

### WikiResource

| Método | Endpoint | Status |
|--------|----------|--------|
| `list(project_id)` | `GET /projects/{id}/wiki/index.json` | Alpha |
| `get(project_id, title, includes)` | `GET /projects/{id}/wiki/{title}.json` | Alpha |
| `get_version(project_id, title, version)` | `GET /projects/{id}/wiki/{title}/{v}.json` | Alpha |
| `create_or_update(project_id, title, request)` | `PUT /projects/{id}/wiki/{title}.json` | Alpha |
| `delete(project_id, title)` | `DELETE /projects/{id}/wiki/{title}.json` | Alpha |

### VersionsResource

| Método | Endpoint | Status |
|--------|----------|--------|
| `get(id)` | `GET /versions/{id}.json` | Alpha |
| `list_by_project(project_id)` | `GET /projects/{id}/versions.json` | Alpha |
| `create_on_project(project_id, request)` | `POST /projects/{id}/versions.json` | Alpha |
| `update(id, request)` | `PUT /versions/{id}.json` | Alpha |
| `delete(id)` | `DELETE /versions/{id}.json` | Alpha |

### EnumerationsResource

| Método | Endpoint | Status |
|--------|----------|--------|
| `list_issue_priorities()` | `GET /enumerations/issue_priorities.json` | Alpha |
| `list_time_entry_activities()` | `GET /enumerations/time_entry_activities.json` | Alpha |
| `list_document_categories()` | `GET /enumerations/document_categories.json` | Alpha |

### TrackersResource

| Método | Endpoint | Status |
|--------|----------|--------|
| `list()` | `GET /trackers.json` | Alpha |

### IssueStatusesResource

| Método | Endpoint | Status |
|--------|----------|--------|
| `list()` | `GET /issue_statuses.json` | Alpha |

### IssueCategoriesResource

| Método | Endpoint | Status |
|--------|----------|--------|
| `list_by_project(project_id)` | `GET /projects/{id}/issue_categories.json` | Alpha |
| `get(id)` | `GET /issue_categories/{id}.json` | Alpha |
| `create(project_id, request)` | `POST /projects/{id}/issue_categories.json` | Alpha |
| `update(id, request)` | `PUT /issue_categories/{id}.json` | Alpha |
| `delete(id, reassign_to_id)` | `DELETE /issue_categories/{id}.json` | Alpha |

### MembershipsResource

| Método | Endpoint | Status |
|--------|----------|--------|
| `list_by_project(project_id)` | `GET /projects/{id}/memberships.json` | Alpha |
| `get(id)` | `GET /memberships/{id}.json` | Alpha |
| `create(project_id, request)` | `POST /projects/{id}/memberships.json` | Alpha |
| `update(id, request)` | `PUT /memberships/{id}.json` | Alpha |
| `delete(id)` | `DELETE /memberships/{id}.json` | Alpha |

### RolesResource

| Método | Endpoint | Status |
|--------|----------|--------|
| `list()` | `GET /roles.json` | Alpha |
| `get(id)` | `GET /roles/{id}.json` | Alpha |

### GroupsResource

| Método | Endpoint | Status |
|--------|----------|--------|
| `list()` | `GET /groups.json` | Alpha |
| `get(id)` | `GET /groups/{id}.json` | Alpha |
| `get_with_includes(id, includes)` | `GET /groups/{id}.json?include=` | Alpha |
| `create(request)` | `POST /groups.json` | Alpha |
| `update(id, request)` | `PUT /groups/{id}.json` | Alpha |
| `delete(id)` | `DELETE /groups/{id}.json` | Alpha |
| `add_user(group_id, user_id)` | `POST /groups/{id}/users.json` | Alpha |
| `remove_user(group_id, user_id)` | `DELETE /groups/{id}/users/{uid}.json` | Alpha |

### CustomFieldsResource

| Método | Endpoint | Status |
|--------|----------|--------|
| `list()` | `GET /custom_fields.json` | Alpha |

### QueriesResource

| Método | Endpoint | Status |
|--------|----------|--------|
| `list()` | `GET /queries.json` | Alpha |

### FilesResource

| Método | Endpoint | Status |
|--------|----------|--------|
| `list_by_project(project_id)` | `GET /projects/{id}/files.json` | Alpha |
| `attach_to_project(project_id, request)` | `POST /projects/{id}/files.json` | Alpha |

### SearchResource

| Método | Endpoint | Status |
|--------|----------|--------|
| `search(filter)` | `GET /search.json` | Alpha |

### NewsResource

| Método | Endpoint | Status |
|--------|----------|--------|
| `list()` | `GET /news.json` | Alpha |
| `list_by_project(project_id)` | `GET /projects/{id}/news.json` | Alpha |
| `get(id)` | `GET /news/{id}.json` | Alpha |
| `get_with_includes(id, includes)` | `GET /news/{id}.json?include=` | Alpha |
| `create(project_id, request)` | `POST /news.json` | Alpha |
| `update(id, request)` | `PUT /news/{id}.json` | Alpha |
| `delete(id)` | `DELETE /news/{id}.json` | Alpha |

### MyAccountResource

| Método | Endpoint | Status |
|--------|----------|--------|
| `get()` | `GET /my/account.json` | Alpha |

---

## Type Reference

A pasta `types/` contém 22 módulos, cada um mapeando os tipos (structs)
de um domínio da API. Todos os tipos implementam `Debug`, `Clone`,
`Serialize` e `Deserialize`.

| Módulo | Tipos principais | Descrição |
|--------|-----------------|-----------|
| `base` | `RedmineId`, `IdName`, `CustomFieldValue`, `UploadToken`, `UploadPayload` | Tipos fundamentais reutilizados |
| `issue` | `Issue`, `IssueFilter`, `CreateIssuePayload`, `UpdateIssuePayload`, `AllowedStatus` | Issues e filtros |
| `project` | `Project`, `CreateProjectPayload`, `UpdateProjectPayload` | Projetos |
| `user` | `User`, `UserFilter`, `CreateUserPayload`, `UpdateUserPayload` | Usuários |
| `time_entry` | `TimeEntry`, `TimeEntryFilter`, `CreateTimeEntryPayload`, `UpdateTimeEntryPayload` | Apontamentos de horas |
| `journal` | `Journal`, `JournalDetail`, `UpdateJournalPayload` | Histórico de alterações |
| `relation` | `Relation`, `CreateRelationPayload` | Relações entre issues |
| `attachment` | `Attachment` | Anexos |
| `wiki` | `WikiPage`, `WikiPageSummary`, `CreateWikiPagePayload` | Páginas wiki |
| `version` | `Version`, `CreateVersionPayload`, `UpdateVersionPayload` | Versões |
| `enumeration` | `IssuePriority`, `TimeEntryActivity`, `DocumentCategory` | Enumerações (listas fixas) |
| `tracker` | `Tracker` | Trackers (tipos de issue) |
| `issue_status` | `IssueStatus` | Status de issue |
| `issue_category` | `IssueCategory`, `CreateIssueCategoryPayload` | Categorias de issue |
| `membership` | `Membership`, `CreateMembershipPayload`, `UpdateMembershipPayload` | Associações projeto-usuário |
| `role` | `Role` | Papéis e permissões |
| `group` | `Group`, `CreateGroupPayload`, `UpdateGroupPayload` | Grupos de usuários |
| `custom_field` | `CustomField` | Campos personalizados |
| `query` | `Query` | Consultas salvas |
| `file` | `File`, `CreateFilePayload` | Arquivos de projeto |
| `search` | `SearchResult`, `SearchFilter` | Resultados de busca textual |
| `news` | `News`, `CreateNewsPayload`, `UpdateNewsPayload` | Notícias |
| `my_account` | `MyAccount` | Conta do usuário atual |

---

## Pagination

### `PaginationParams`

```rust,ignore
pub struct PaginationParams {
    pub offset: Option<u32>,   // Deslocamento (padrão: 0)
    pub limit: Option<u32>,    // Limite por página (padrão: 25, máx: 100)
}
```

### Comportamento do servidor

- O servidor **ignora** `limit` > 100 e usa 100 como teto.
- `offset` = 0 retorna os primeiros registros.
- `total_count` reflete o total **antes** dos filtros de paginação.
- Se `offset` >= `total_count`, o array de itens vem vazio.
- O wrapper faz auto-paginação internamente — métodos de listagem como
  `client.issues.list()` retornam `Vec<T>` com **todos** os registros.
