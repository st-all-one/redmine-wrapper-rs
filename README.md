<div align="center">

# redmine-wrapper-rs

**Wrapper Rust tipado para a API REST do Redmine — assíncrono, seguro, zero custo**

[![License: MPL 2.0](https://img.shields.io/badge/License-MPL_2.0-2b3a42?style=for-the-badge)](https://opensource.org/licenses/MPL-2.0)
[![Rust](https://img.shields.io/badge/Rust-1.85%2B-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org)
[![Made in Brazil](https://img.shields.io/badge/Made_in-Brazil-009739?style=for-the-badge)](https://github.com/topics/brazil)

</div>

**`redmine-wrapper-rs`** é um wrapper puramente back-end para a API REST do Redmine, construído em **Rust assíncrono (tokio + reqwest)** com foco em segurança de tipos, rastreabilidade via UUID v7 e cobertura completa de todos os 86 endpoints. Cada instância é isolada, imutável e `Send + Sync` — pode ser compartilhada entre tasks tokio sem risco de contaminação de estado.

---

## 📖 Documentação

- [Guia de Uso](./wiki/usage-guide.md) — Exemplos completos para todos os 22 recursos
- [Getting Started](./wiki/getting-started.md) — Instalação, configuração, primeira chamada
- [Guia de Integração](./wiki/integration-guide.md) — DI, retry, cache, axum, impersonação
- [Particularidades da API](./wiki/particularities.md) — Envelopes JSON, include pattern, upload 2-passos
- [Referência da API](./wiki/api-reference.md) — Lista completa de structs e métodos
- [Catálogo de Erros](./wiki/error/errors.md) — Erros RFC 7807 com UUID v7

---

## 🚀 Quick-start

### Adicione ao `Cargo.toml`:

```toml
[dependencies]
redmine-wrapper-rs = "0.2"
tokio = { version = "1", features = ["full"] }
```

### Configuração:

```rust
use redmine_wrapper::{RedmineClient, RedmineConfigBuilder};

let client = RedmineClient::new(
    RedmineConfigBuilder::default()
        .base_url(std::env::var("REDMINE_URL")
            .unwrap_or_else(|_| "https://redmine.example.com".into()))
        .token(std::env::var("REDMINE_TOKEN")
            .expect("REDMINE_TOKEN é obrigatório"))
        .build()?,
)?;
```

### Uso básico:

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = /* ... */;

    // Projetos
    let projects = client.projects.list().await?;
    for p in &projects {
        println!("{}: {}", p.id, p.name.as_deref().unwrap_or("?"));
    }

    // Issues abertas atribuídas a mim
    use redmine_wrapper::types::issue::IssueFilter;
    let issues = client.issues.list(Some(&IssueFilter {
        assigned_to_id: Some("me".into()),
        status_id: Some("open".into()),
        ..Default::default()
    })).await?;

    // Criar issue
    let issue = client.issues.create(&CreateIssuePayload {
        project_id: 1,
        subject: "Bug encontrado".into(),
        description: Some("Passos para reproduzir...".into()),
        priority_id: Some(4),
        ..Default::default()
    }).await?;

    Ok(())
}
```

### Tratamento de erros (RFC 7807):

```rust
use redmine_wrapper::core::errors::{RedmineError, ErrorCategory};

async fn handle(client: &RedmineClient) -> Result<(), RedmineError> {
    match client.projects.get(99999).await {
        Err(RedmineError::Api { category, status, detail, instance, .. }) => {
            eprintln!("[{}] {} (UUID: {})", status, detail, instance);
            match category {
                ErrorCategory::ResourceNotFound => { /* 404 */ }
                ErrorCategory::RateLimited => { /* 429 — aguardar */ }
                ErrorCategory::AuthenticationFailed => { /* 401 — reautenticar */ }
                _ => { /* outros */ }
            }
        }
        Err(RedmineError::Config(msg)) => {
            eprintln!("Erro de configuração: {}", msg);
        }
        Ok(project) => println!("{}", project.name.as_deref().unwrap_or("")),
        Err(e) => return Err(e),
    }
    Ok(())
}
```

---

## 🌐 Recursos Cobertos

Todos os **22 recursos** com **86 métodos** públicos.

| Resource | list | get | create | update | delete | Métodos Extras |
|---|---|---|---|---|---|---|
| **Issues** | ✅ | ✅ | ✅ | ✅ | ✅ | `get_with_includes`, `get_allowed_statuses`, `add_watcher`, `remove_watcher` |
| **Projects** | ✅ | ✅ | ✅ | ✅ | ✅ | `get_with_includes`, `archive`, `unarchive` |
| **Users** | ✅ | ✅ | ✅ | ✅ | ✅ | `get_with_includes`, `get_current` |
| **Time Entries** | ✅ | ✅ | ✅ | ✅ | ✅ | — |
| **Journals** | — | — | — | ✅ | ✅ (remove) | journals via `?include=journals` |
| **Relations** | ✅ | ✅ | ✅ | — | ✅ | `list_by_issue`, `create_on_issue` |
| **Attachments** | — | ✅ | — | — | ✅ | `upload` (2-passos) |
| **Wiki** | ✅ | ✅ | ✅ | ✅ | ✅ | `get_version`, `create_or_update` |
| **Versions** | ✅ | ✅ | ✅ | ✅ | ✅ | `list_by_project`, `create_on_project` |
| **Enumerations** | ✅ | — | — | — | — | 3 endpoints de listagem |
| **Trackers** | ✅ | — | — | — | — | — |
| **Issue Statuses** | ✅ | — | — | — | — | — |
| **Issue Categories** | ✅ | ✅ | ✅ | ✅ | ✅ | `list_by_project`, delete com reassign |
| **Memberships** | ✅ | ✅ | ✅ | ✅ | ✅ | `list_by_project` |
| **Roles** | ✅ | ✅ | — | — | — | — |
| **Groups** | ✅ | ✅ | ✅ | ✅ | ✅ | `get_with_includes`, `add_user`, `remove_user` |
| **Custom Fields** | ✅ | — | — | — | — | — |
| **Queries** | ✅ | — | — | — | — | — |
| **Files** | ✅ | — | — | — | — | `list_by_project`, `attach_to_project` |
| **Search** | ✅ | — | — | — | — | — |
| **News** | ✅ | ✅ | ✅ | ✅ | ✅ | `list_by_project` |
| **My Account** | — | ✅ | — | — | — | — |

---

## 🎯 Principais Características

### Imutabilidade e Isolamento

Cada instância é criada com `RedmineClient::new()` e sua `RedmineConfig` é **imutável** — não há `set_token()` ou `set_base_url()`. Para usar configurações diferentes, crie uma nova instância. Isso permite que múltiplas partes do sistema operem com credenciais e servidores diferentes **simultaneamente** sem risco de contaminação de estado global.

```rust
let client_admin = RedmineClient::new(RedmineConfig {
    base_url: url.clone(),
    token: Some(admin_token),
    switch_user: Some("joao".into()),
    ..Default::default()
})?;
let client_user = RedmineClient::new(RedmineConfig {
    base_url: url,
    token: Some(user_token),
    ..Default::default()
})?;
```

### Segurança de Tipos com `serde`

Todas as structs de domínio derivam `Serialize + Deserialize` com `#[serde(rename_all = "snake_case")]` e `skip_serializing_if = "Option::is_none"`. Isso garante que apenas campos preenchidos sejam serializados e que o mapeamento JSON-Rust seja exato:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateIssuePayload {
    pub project_id: RedmineId,
    pub subject: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority_id: Option<RedmineId>,
    // ...
}
```

### Sistema de Erros RFC 7807 com UUID v7

Todos os erros de API seguem o padrão **Problem Details (RFC 7807)**. Cada erro recebe um **UUID v7** único para correlação distribuída:

```json
{
    "category": "resource-not-found",
    "status": 404,
    "detail": "Issue with id=99999 not found",
    "instance": "0194b3e0-7f1a-7d80-8000-123456789abc",
    "context": {
        "operation": "issues.get",
        "http_status": 404
    }
}
```

12 categorias mapeadas diretamente para status HTTP via `ErrorCategory::from_status()`.

### Async nativo (tokio + reqwest)

Toda a biblioteca é **assíncrona** — integra-se nativamente com axum, actix e outros frameworks async sem `spawn_blocking`:

```rust
use axum::{extract::State, routing::get, Json, Router};
use std::sync::Arc;

async fn listar_issues(State(client): State<Arc<RedmineClient>>) -> Json<serde_json::Value> {
    match client.issues.list(None).await {
        Ok(issues) => Json(serde_json::json!({ "issues": issues })),
        Err(e) => Json(serde_json::json!({ "error": e.to_string() })),
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/issues", get(listar_issues))
        .with_state(client);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

### Rate Limiting (Sliding Window)

Proteção client-side contra sobrecarga do servidor. Padrão: **10 requisições/s**, configurável via `max_rps`.

```rust
let client = RedmineClient::new(
    RedmineConfigBuilder::default()
        .base_url(url)
        .token(token)
        .max_rps(5)  // máximo 5 req/s
        .build()?,
)?;
```

### Paginação Automática

Métodos de listagem (`issues.list()`, `projects.list()`, etc.) fazem **auto-paginação eager** e retornam `Vec<T>` com todos os registros. Não é necessário gerenciar `offset`/`limit` manualmente.

### Thread Safety nativo

Toda a biblioteca é `Send + Sync`. O `HttpClient` interno usa `Arc<HttpClient>` e `tokio::sync::Mutex<SlidingWindow>`, permitindo compartilhamento seguro entre tasks tokio:

```rust
let client = Arc::new(RedmineClient::new(config)?);

let c = Arc::clone(&client);
tokio::spawn(async move {
    let projects = c.projects.list().await.unwrap();
    // ...
});

let users = client.users.get_current().await?; // seguro: Arc compartilha o rate limiter
```

### Impersonação (X-Redmine-Switch-User)

Administradores podem atuar como outro usuário configurando `switch_user`:

```rust
let client = RedmineClient::new(
    RedmineConfigBuilder::default()
        .base_url(url)
        .token(admin_token)
        .switch_user("joao.silva")
        .build()?,
)?;
// Todas as operações são feitas como "joao.silva"
client.issues.list(None).await?;
```

### Upload de Arquivos em 2 Passos

```rust
// Passo 1: Upload → token
let data = std::fs::read("relatorio.pdf")?;
let token = client.attachments.upload("relatorio.pdf", &data).await?;

// Passo 2: Associar token a uma issue
client.issues.update(42, &UpdateIssuePayload {
    uploads: Some(vec![UploadPayload {
        token: token.clone(),
        filename: Some("relatorio.pdf".into()),
        content_type: Some("application/pdf".into()),
        description: Some("Relatório mensal".into()),
    }]),
    ..Default::default()
}).await?;
```

### Logs Estruturados via `tracing`

O wrapper usa `tracing` — logs estruturados com spans e instrumentação automática:

```bash
RUST_LOG=redmine_wrapper=debug cargo run
```

Cada operação HTTP é registrada com `operation`, `status` e `duration` via `#[instrument]`.

---

## 🔍 Testes

```bash
cargo test                       # 31 testes (unitários + integração com wiremock)
cargo test --test client_test    # testes do cliente HTTP
cargo test --test errors_test    # testes do sistema de erros
cargo test --test pagination_test # testes de paginação
cargo clippy                     # lints (0 warnings)
```

```
❯ cargo test
running 31 tests
test result: ok. 31 passed; 0 failed
```

---

## 🛠️ Stack

| Componente | Tecnologia |
|---|---|
| Runtime | Rust 1.85+ (edition 2024) |
| Async | `tokio` (sync, time) |
| HTTP | `reqwest` 0.12 (async, rustls-tls) |
| Serialização | `serde` + `serde_json` |
| Erros | `thiserror` — enum com 12 categorias + UUID v7 |
| Logs | `tracing` — spans, `#[instrument]`, structured |
| UUID | `uuid` v7 (correlação distribuída) |
| Rate limiting | Sliding window com `tokio::sync::Mutex` |
| Retry | Manual (você decide como e quando) |
| Licença | MPL-2.0 |

---

## ⚠️ Limitações Conhecidas

### Sem suporte a OAuth

O Redmine **não suporta OAuth**. A autenticação é exclusivamente via chave de API no header `X-Redmine-API-Key`. Não há planos de adicionar OAuth — o mecanismo não existe no servidor.

### Upload em 2 passos (não atomico)

O upload de arquivos exige duas chamadas HTTP separadas (upload do binário + associação ao recurso). Isso é uma limitação da API do Redmine, não do wrapper.

### Paginação eager (não lazy)

Diferente de wrappers em linguagens com generators (Python, JavaScript), o Rust implementa paginação **eager**: `list()` carrega tudo em `Vec<T>`. Para conjuntos muito grandes, use filtros no servidor (`IssueFilter`, `TimeEntryFilter`, etc.) para reduzir o payload.

### Limite de 100 itens por pagina

O servidor Redmine impõe um teto de 100 itens por página. O wrapper faz auto-paginação respeitando esse limite automaticamente.

---

## 📋 Pré-requisitos

- **Rust 1.85+** (edição 2024) — verifique com `rustc --version`
- **Chave de API Redmine** — gere em *Minha Conta → Chave de API* no seu Redmine

---

## 📊 Comparação com Wrapper TypeScript

| Aspecto | TypeScript (`@st-all-one/redmine-wrapper-ts`) | Rust (`redmine-wrapper-rs`) |
|---|---|---|
| **Runtime** | Deno 2.0 (async) | Tokio (async nativo) |
| **HTTP** | `fetch` nativo | `reqwest` com rustls |
| **Paginação** | Lazy (`AsyncIterableIterator`) | Eager (`Vec<T>`) |
| **Erros** | Classe `RedmineWrapperError` | Enum `RedmineError` com `match` |
| **Null safety** | `undefined` / `null` | `Option<T>` com `unwrap`/`?` |
| **Config** | Objeto `create({...})` | Struct `RedmineConfig + Default + Builder` |
| **Resources** | 22 classes lazy via getter | 22 structs como campos diretos |
| **Thread safety** | `EventLoop` single-thread | `Send + Sync` nativo |
| **Testes** | 28 testes (Deno) | 31 testes (wiremock) |
| **Maturidade** | ~86 métodos públicos | ~86 métodos públicos |

---

<div align="center">

---

**Licença:** Mozilla Public License v2.0 (MPL-2.0) — veja o arquivo [`LICENSE`](./LICENSE)

</div>
