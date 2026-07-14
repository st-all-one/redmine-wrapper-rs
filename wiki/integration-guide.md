# Guia de Integração

## Índice

- [Singleton e DI](#singleton-e-di)
- [Retry com Exponential Backoff](#retry-com-exponential-backoff)
- [Estratégias de Cache](#estratégias-de-cache)
- [Dicas de Performance](#dicas-de-performance)
- [Impersonação (X-Redmine-Switch-User)](#impersonação-x-redmine-switch-user)
- [Upload de Arquivos](#upload-de-arquivos)
- [Monitoramento de Erros com UUIDs](#monitoramento-de-erros-com-uuids)
- [Integração com Axum](#integração-com-axum)
- [Integração com Outros Runtimes](#integração-com-outros-runtimes)

---

## Singleton e DI

O `RedmineClient` é thread-safe (Send + Sync). Pode ser compartilhado
entre tasks sem problemas.

### Singleton com `OnceLock`

```rust,ignore
use std::sync::OnceLock;
use redmine_wrapper::{RedmineClient, RedmineConfigBuilder};

static CLIENT: OnceLock<RedmineClient> = OnceLock::new();

fn get_client() -> &'static RedmineClient {
    CLIENT.get_or_init(|| {
        RedmineClient::new(
            RedmineConfigBuilder::default()
                .base_url("https://redmine.exemplo.com")
                .token("sua-chave")
                .build()
                .expect("config inválida"),
        )
        .expect("falha ao criar cliente")
    })
}
```

### Injeção de Dependência (DI)

```rust,ignore
use std::sync::Arc;
use redmine_wrapper::{RedmineClient, RedmineConfigBuilder};

struct AppState {
    redmine: RedmineClient,  // ou Arc<RedmineClient>
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RedmineClient::new(
        RedmineConfigBuilder::default()
            .base_url("https://redmine.exemplo.com")
            .token("sua-chave")
            .build()?,
    )?;

    let state = AppState { redmine: client };
    // Pode ser passado para tasks com segurança
    tokio::spawn(async move {
        let issues = state.redmine.issues.list(None).await.unwrap();
        println!("{} issues", issues.len());
    });
    Ok(())
}
```

### Tabela de Decisão

| Cenário                     | Abordagem            | Quando usar                                          |
|-----------------------------|----------------------|------------------------------------------------------|
| Script descartável          | `RedmineClient` direto | Simplicidade máxima                                |
| Aplicação single-task       | `OnceLock`           | Setup único, sem concorrência                        |
| Aplicação multi-task        | `Arc<RedmineClient>` | Compartilhar entre tasks tokio                       |
| Web server (Axum/Actix)     | `RedmineClient` ou `Arc` | Injetado como estado via `State` ou `Data`         |

---

## Retry com Exponential Backoff

O wrapper não implementa retry automático. Abaixo um utilitário genérico
com suporte async:

```rust,ignore
use tokio::time::{sleep, Duration};
use redmine_wrapper::core::errors::{RedmineError, ErrorCategory};

async fn with_retry<F, Fut, T>(
    mut operation: F,
    max_retries: u32,
    base_delay_ms: u64,
    operation_name: &str,
) -> Result<T, RedmineError>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T, RedmineError>>,
{
    for attempt in 1..=max_retries {
        match operation().await {
            Ok(value) => return Ok(value),
            Err(ref e) if is_retryable(e) && attempt < max_retries => {
                let delay_ms = base_delay_ms * 2u64.pow(attempt - 1);
                tracing::warn!("[{}] tentativa {}/{} falhou, retry em {}ms", operation_name, attempt, max_retries, delay_ms);
                sleep(Duration::from_millis(delay_ms)).await;
            }
            Err(e) => return Err(e),
        }
    }
    unreachable!()
}

fn is_retryable(err: &RedmineError) -> bool {
    matches!(err.category(), Some(ErrorCategory::RateLimited | ErrorCategory::NetworkError | ErrorCategory::Timeout))
}
```

### Tabela de Decisão

| HTTP | Categoria           | Retry? | Motivo                         |
|------|---------------------|--------|--------------------------------|
| 429  | `RateLimited`       | Sim    | Redmine por IP/token           |
| 503  | `NetworkError`      | Sim    | Sobrecarga temporária          |
| 504  | `Timeout`           | Sim    | Timeout de rede                |
| 401  | `AuthenticationFailed` | Não | Token inválido                 |
| 403  | `AuthorizationDenied`  | Não | Permissão negada               |
| 404  | `ResourceNotFound`  | Não    | Recurso inexistente            |
| 422  | `ValidationError`   | Não    | Payload precisa ser corrigido  |

---

## Estratégias de Cache

### Cache de Dados Estáticos

Ideal para trackers, status, enumerations — raramente mudam.

```rust,ignore
use std::sync::Mutex;
use std::collections::HashMap;
use redmine_wrapper::types::tracker::Tracker;

pub struct TrackerCache {
    cache: Mutex<HashMap<&'static str, Vec<Tracker>>>,
}

impl TrackerCache {
    pub fn new() -> Self {
        Self { cache: Mutex::new(HashMap::new()) }
    }

    pub async fn get_trackers(&self, client: &RedmineClient) -> Result<Vec<Tracker>, RedmineError> {
        let mut cache = self.cache.lock().expect("lock");
        if let Some(trackers) = cache.get("trackers") {
            return Ok(trackers.clone());
        }
        let trackers = client.trackers.list().await?;
        cache.insert("trackers", trackers.clone());
        Ok(trackers)
    }
}
```

### Cache com Expiração

Para projetos, usuários — mudam com frequência moderada.

```rust,ignore
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

struct TimedEntry<T> { data: T, expires_at: Instant }

pub struct TimedCache<T> {
    inner: Mutex<HashMap<String, TimedEntry<T>>>,
    ttl: Duration,
}

impl<T: Clone> TimedCache<T> {
    pub fn new(ttl: Duration) -> Self {
        Self { inner: Mutex::new(HashMap::new()), ttl }
    }
    pub fn get(&self, key: &str) -> Option<T> {
        let cache = self.inner.lock().expect("lock");
        cache.get(key).and_then(|e| {
            if Instant::now() < e.expires_at { Some(e.data.clone()) } else { None }
        })
    }
    pub fn set(&self, key: String, value: T) {
        self.inner.lock().expect("lock").insert(key, TimedEntry {
            data: value,
            expires_at: Instant::now() + self.ttl,
        });
    }
}
```

---

## Dicas de Performance

### Filtrar no servidor

Sempre use filtros — reduz payload e latência.

```rust,ignore
use redmine_wrapper::types::issue::IssueFilter;

// BOM: deixa o Redmine filtrar
let filter = IssueFilter {
    assigned_to_id: Some("me".into()),
    status_id: Some("open".into()),
    ..Default::default()
};
let issues = client.issues.list(Some(&filter)).await?;
```

### Rate Limit

```rust,ignore
let client = RedmineClient::new(
    RedmineConfigBuilder::default()
        .base_url("https://redmine.exemplo.com")
        .token("token")
        .max_rps(5)
        .build()?,
)?;
```

### Timeout

```rust,ignore
let client = RedmineClient::new(
    RedmineConfigBuilder::default()
        .base_url("https://redmine.exemplo.com")
        .token("token")
        .timeout_secs(60)
        .build()?,
)?;
```

---

## Impersonação (X-Redmine-Switch-User)

Administradores podem atuar como outro usuário.

```rust,ignore
let client = RedmineClient::new(
    RedmineConfigBuilder::default()
        .base_url("https://redmine.exemplo.com")
        .token("token_admin")
        .switch_user("joao.silva")
        .build()?,
)?;

// Todas as operações serão feitas como "joao.silva"
client.issues.list(None).await?;
```

| Requisito          | Detalhe                                           |
|--------------------|---------------------------------------------------|
| Permissão          | Usuário autenticado deve ser **administrador**    |
| Formato            | Login do usuário (`String`)                       |
| Header             | `X-Redmine-Switch-User: login`                    |
| Erro               | 412 `ImpersonationFailed` se usuário inexistente  |

---

## Upload de Arquivos

Processo em 2 passos.

### Passo 1: Upload do binário → token

```rust,ignore
let data = std::fs::read("relatorio.pdf")?;
let token = client.attachments.upload("relatorio.pdf", &data).await?;
```

### Passo 2: Associar token a uma issue

```rust,ignore
use redmine_wrapper::types::issue::UpdateIssuePayload;
use redmine_wrapper::types::base::UploadPayload;

client.issues.update(42, &UpdateIssuePayload {
    notes: Some("Relatório anexado".into()),
    uploads: Some(vec![UploadPayload {
        token: token.clone(),
        filename: Some("relatorio.pdf".into()),
        content_type: Some("application/pdf".into()),
        description: Some("Relatório mensal".into()),
    }]),
    ..Default::default()
}).await?;
```

---

## Monitoramento de Erros com UUIDs

Cada erro da API contém um UUID v7 (`instance`) para rastreamento.

```rust,ignore
use redmine_wrapper::core::errors::RedmineError;

if let Err(RedmineError::Api { category, detail, instance, .. }) = client.projects.list().await {
    eprintln!("[{}] {} (correlation: {})", category, detail, instance);
}
```

### Integração com Sentry

```rust,ignore
fn reportar(err: &RedmineError) {
    if let RedmineError::Api { category, status, detail, instance, .. } = err {
        sentry::with_scope(|scope| {
            scope.set_tag("correlation_id", &instance.to_string());
            scope.set_tag("http_status", &status.to_string());
            scope.set_tag("error_category", &category.to_string());
        });
        sentry::capture_message(&detail, sentry::Level::Error);
    }
}
```

---

## Integração com Axum

O `RedmineClient` é nativamente async e `Send + Sync`, podendo ser usado
diretamente como estado do Axum via `State`:

```rust,ignore
use axum::{extract::State, routing::get, Json, Router};
use redmine_wrapper::{RedmineClient, RedmineConfigBuilder};
use std::sync::Arc;

async fn listar_issues(State(client): State<Arc<RedmineClient>>) -> Json<serde_json::Value> {
    match client.issues.list(None).await {
        Ok(issues) => Json(serde_json::json!({ "issues": issues })),
        Err(e) => Json(serde_json::json!({ "error": e.to_string() })),
    }
}

#[tokio::main]
async fn main() {
    let client = Arc::new(RedmineClient::new(
        RedmineConfigBuilder::default()
            .base_url(std::env::var("REDMINE_URL").unwrap())
            .token(std::env::var("REDMINE_TOKEN").ok())
            .build()
            .unwrap(),
    ).unwrap());

    let app = Router::new()
        .route("/issues", get(listar_issues))
        .with_state(client);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

### Exemplo com Tokio + Tarefas Concorrentes

```rust,ignore
use std::sync::Arc;
use redmine_wrapper::{RedmineClient, RedmineConfigBuilder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Arc::new(RedmineClient::new(
        RedmineConfigBuilder::default()
            .base_url("https://redmine.exemplo.com")
            .token("sua-chave")
            .build()?,
    )?);

    // Dispara múltiplas requisições concorrentes
    let (issues, projects) = tokio::join!(
        client.issues.list(None),
        client.projects.list(),
    );

    println!("Issues: {:?}", issues?.len());
    println!("Projetos: {:?}", projects?.len());

    Ok(())
}
```

---

## Integração com Outros Runtimes

O `RedmineClient` usa `reqwest` async e `tokio` internamente (rate limiter).
Para usar com outros runtimes async (async-std, smol), é necessário
executar num runtime tokio auxiliar ou criar o cliente num contexto tokio.

Recomenda-se usar **tokio como runtime principal** quando for consumir
esta biblioteca.
