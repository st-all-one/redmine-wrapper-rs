# Guia de Integração

## Índice

- [Singleton e DI](#singleton-e-di)
- [Retry com Exponential Backoff](#retry-com-exponential-backoff)
- [Estratégias de Cache](#estratégias-de-cache)
- [Dicas de Performance](#dicas-de-performance)
- [Impersonação (X-Redmine-Switch-User)](#impersonação-x-redmine-switch-user)
- [Upload de Arquivos](#upload-de-arquivos)
- [Monitoramento de Erros com UUIDs](#monitoramento-de-erros-com-uuids)
- [Uso com Tokio/Async](#uso-com-tokio-async)

---

## Singleton e DI

O `RedmineClient` é thread-safe graças ao `Arc` interno e ao `Mutex` do rate
limiter. Pode ser compartilhado entre threads sem problemas.

### Singleton com `OnceLock`

```rust,ignore
use std::sync::OnceLock;
use redmine_wrapper::RedmineClient;
use redmine_wrapper::RedmineConfigBuilder;

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

### Injeção de Dependência com Arc

```rust,ignore
use std::sync::Arc;
use redmine_wrapper::RedmineClient;
use redmine_wrapper::RedmineConfigBuilder;

struct AppState {
    redmine: Arc<RedmineClient>,
}

fn criar_cliente() -> Result<Arc<RedmineClient>, Box<dyn std::error::Error>> {
    let config = RedmineConfigBuilder::default()
        .base_url("https://redmine.exemplo.com")
        .token("sua-chave")
        .build()?;
    Ok(Arc::new(RedmineClient::new(config)?))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = criar_cliente()?;
    let state = AppState { redmine: Arc::clone(&client) };
    // Pode ser passado para threads com segurança
    std::thread::spawn(move || {
        let issues = state.redmine.issues.list(None).unwrap();
        println!("{} issues", issues.len());
    });
    Ok(())
}
```

### Tabela de Decisão

| Cenário                     | Abordagem            | Quando usar                                          |
|-----------------------------|----------------------|------------------------------------------------------|
| Script descartável          | `RedmineClient` direto | Simplicidade máxima                                |
| Aplicação single-thread     | `OnceLock`           | Setup único, sem concorrência                        |
| Aplicação multi-thread      | `Arc<RedmineClient>` | Compartilhar entre threads                           |
| Web server (Actix/Axum)     | `Arc<RedmineClient>` | Injetado como estado da aplicação                    |

---

## Retry com Exponential Backoff

O wrapper não implementa retry automático. Abaixo um utilitário genérico:

```rust,ignore
use std::time::Duration;
use std::thread::sleep;
use redmine_wrapper::core::errors::{RedmineError, ErrorCategory};

fn with_retry<F, T>(
    mut operation: F,
    max_retries: u32,
    base_delay_ms: u64,
    operation_name: &str,
) -> Result<T, RedmineError>
where
    F: FnMut() -> Result<T, RedmineError>,
{
    for attempt in 1..=max_retries {
        match operation() {
            Ok(value) => return Ok(value),
            Err(ref e) if is_retryable(e) && attempt < max_retries => {
                let delay_ms = base_delay_ms * 2u64.pow(attempt - 1);
                log::warn!("[{}] tentativa {}/{} falhou, retry em {}ms", operation_name, attempt, max_retries, delay_ms);
                sleep(Duration::from_millis(delay_ms));
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

Cache reduz chamadas à API. Duas estratégias comuns:

### Cache Simples (HashMap) — Dados estáticos

Ideal para trackers, status, enumerations — raramente mudam.

```rust,ignore
use std::collections::HashMap;
use std::sync::Mutex;
use redmine_wrapper::types::tracker::Tracker;

pub struct TrackerCache {
    cache: Mutex<HashMap<&'static str, Vec<Tracker>>>,
}

impl TrackerCache {
    pub fn new() -> Self { Self { cache: Mutex::new(HashMap::new()) } }

    pub fn get_trackers(&self, client: &RedmineClient) -> Result<Vec<Tracker>, RedmineError> {
        let mut cache = self.cache.lock().unwrap();
        if let Some(trackers) = cache.get("trackers") {
            return Ok(trackers.clone());
        }
        let trackers = client.trackers.list()?;
        cache.insert("trackers", trackers.clone());
        Ok(trackers)
    }
}
```

### TimedCache — Dados com expiração

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
    pub fn new(ttl: Duration) -> Self { Self { inner: Mutex::new(HashMap::new()), ttl } }
    pub fn get(&self, key: &str) -> Option<T> {
        let cache = self.inner.lock().unwrap();
        cache.get(key).and_then(|e| if Instant::now() < e.expires_at { Some(e.data.clone()) } else { None })
    }
    pub fn set(&self, key: String, value: T) {
        self.inner.lock().unwrap().insert(key, TimedEntry { data: value, expires_at: Instant::now() + self.ttl });
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
let issues = client.issues.list(Some(&filter))?;
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

Administradores podem atuar como outro usuário enviando o header
`X-Redmine-Switch-User`.

```rust,ignore
let client = RedmineClient::new(
    RedmineConfigBuilder::default()
        .base_url("https://redmine.exemplo.com")
        .token("token_admin")
        .switch_user("joao.silva")
        .build()?,
)?;

// Todas as operações serão feitas como "joao.silva"
client.issues.list(None)?;
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
let token = client.attachments.upload("relatorio.pdf", &data)?;
```

### Passo 2: Associar token a uma issue

```rust,ignore
use redmine_wrapper::types::issue::UpdateIssuePayload;

client.issues.update(42, &UpdateIssuePayload {
    notes: Some("Relatório anexado".into()),
    uploads: Some(vec![UploadPayload {
        token: token.clone(),
        filename: Some("relatorio.pdf".into()),
        content_type: Some("application/pdf".into()),
        description: Some("Relatório mensal".into()),
    }]),
    ..Default::default()
})?;
```

---

## Monitoramento de Erros com UUIDs

Cada erro da API contém um UUID v7 (`instance`) para rastreamento.

```rust,ignore
use redmine_wrapper::core::errors::RedmineError;

if let Err(RedmineError::Api { category, detail, instance, .. }) = client.projects.list() {
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

## Uso com Tokio/Async

O wrapper é síncrono (blocking). Para uso em runtimes async, use `spawn_blocking`.

```rust,ignore
use tokio::task::spawn_blocking;
use std::sync::Arc;
use redmine_wrapper::RedmineClient;
use redmine_wrapper::RedmineConfigBuilder;

async fn buscar_issues(client: Arc<RedmineClient>) -> Result<Vec<Issue>, RedmineError> {
    spawn_blocking(move || client.issues.list(None))
        .await
        .map_err(|e| RedmineError::Config(format!("spawn_blocking panicked: {e}")))?
}
```

### Semáforo para controle de concorrência

```rust,ignore
use tokio::sync::Semaphore;

pub struct RedminePool {
    client: Arc<RedmineClient>,
    semaphore: Arc<Semaphore>,
}

impl RedminePool {
    pub fn new(client: Arc<RedmineClient>, max_concurrent: usize) -> Self {
        Self { client, semaphore: Arc::new(Semaphore::new(max_concurrent)) }
    }

    pub async fn call<F, T>(&self, op: F) -> Result<T, RedmineError>
    where
        F: FnOnce(Arc<RedmineClient>) -> Result<T, RedmineError> + Send + 'static,
        T: Send + 'static,
    {
        let _permit = self.semaphore.acquire().await.map_err(|e| RedmineError::Config(format!("semaphore: {e}")))?;
        let client = Arc::clone(&self.client);
        spawn_blocking(move || op(client))
            .await
            .map_err(|e| RedmineError::Config(format!("task panicked: {e}")))?
    }
}
```

### Exemplo: Web Server com Actix

```rust,ignore
use actix_web::{web, App, HttpServer, HttpResponse};
use std::sync::Arc;
use tokio::task::spawn_blocking;
use redmine_wrapper::RedmineClient;
use redmine_wrapper::RedmineConfigBuilder;

async fn listar_issues(client: web::Data<Arc<RedmineClient>>) -> HttpResponse {
    let c = Arc::clone(&client);
    match spawn_blocking(move || c.issues.list(None)).await {
        Ok(Ok(issues)) => HttpResponse::Ok().json(issues),
        Ok(Err(e)) => HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()})),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({"error": format!("panic: {e}")})),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = Arc::new(RedmineClient::new(
        RedmineConfigBuilder::default()
            .base_url("https://redmine.exemplo.com")
            .token("token")
            .build().unwrap(),
    ).unwrap());

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Arc::clone(&client)))
            .route("/issues", web::get().to(listar_issues))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```
