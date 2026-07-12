# Guia de Integração

## Índice

- [Factory Pattern e DI](#factory-pattern-e-di)
- [Retry com Exponential Backoff](#retry-com-exponential-backoff)
- [Estratégias de Cache](#estratégias-de-cache)
- [Dicas de Performance](#dicas-de-performance)
- [Impersonação (X-Redmine-Switch-User)](#impersonação-x-redmine-switch-user)
- [Upload de Arquivos](#upload-de-arquivos)
- [Monitoramento de Erros com UUIDs](#monitoramento-de-erros-com-uuids)
- [Uso com Tokio/Async](#uso-com-tokio-async)

---

## Factory Pattern e DI

O `RedmineClient` é construído através de um **Factory Pattern** que garante inicialização segura em
ambiente single-thread e multi-thread. A biblioteca utiliza `OnceLock` para lazy initialization
estática e `Arc` para compartilhamento entre threads.

### Singleton com OnceLock

`OnceLock` (estabilizado no Rust 1.70) permite que o cliente seja inicializado uma única vez de
forma segura, mesmo em cenários concorrentes.

```rust,ignore
use std::sync::OnceLock;
use redmine_wrapper_rs::RedmineClient;
use redmine_wrapper_rs::config::{AuthMethod, RedmineConfig};

static CLIENT: OnceLock<RedmineClient> = OnceLock::new();

fn get_client() -> &'static RedmineClient {
    CLIENT.get_or_init(|| {
        RedmineClient::new(
            RedmineConfig::builder()
                .base_url("https://redmine.exemplo.com")
                .token(std::env::var("REDMINE_API_TOKEN").expect("REDMINE_API_TOKEN não definido"))
                .auth_method(AuthMethod::ApiKey)
                .build()
                .expect("configuração inválida"),
        )
        .expect("falha ao criar cliente Redmine")
    })
}

fn main() {
    let projetos = get_client().projects().list(0, 25).unwrap();
    println!("{} projectos encontrados", projetos.len());
}
```

### Injeção de Dependência com Arc

Para aplicações que exigem múltiplas instâncias ou injeção de dependência, utilize `Arc`:

```rust,ignore
use std::sync::Arc;
use redmine_wrapper_rs::RedmineClient;
use redmine_wrapper_rs::config::{AuthMethod, RedmineConfig};

struct AppState {
    redmine: Arc<RedmineClient>,
}

fn criar_cliente() -> Result<Arc<RedmineClient>, Box<dyn std::error::Error>> {
    let config = RedmineConfig::builder()
        .base_url("https://redmine.exemplo.com")
        .token("seu_token_api_aqui")
        .auth_method(AuthMethod::ApiKey)
        .build()?;

    Ok(Arc::new(RedmineClient::new(config)?))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = criar_cliente()?;
    let state = AppState { redmine: client };

    // Clonar Arc é barato — apenas incrementa o contador de referências
    let state_outra_thread = AppState {
        redmine: Arc::clone(&state.redmine),
    };

    // Pode ser passado para threads com segurança
    std::thread::spawn(move || {
        let issues = state_outra_thread.redmine.issues().list(0, 25).unwrap();
        println!("{} issues", issues.len());
    });

    Ok(())
}
```

### Tabela de Decisão: Factory vs Singleton vs Arc

| Cenário                          | Abordagem            | Quando usar                                              |
|----------------------------------|----------------------|----------------------------------------------------------|
| Aplicação single-thread          | `OnceLock`           | Setup único, sem concorrência                            |
| Aplicação multi-thread           | `Arc<RedmineClient>` | Compartilhar entre threads com clone barato              |
| Testes unitários / mocks         | DI com `Arc`         | Substituir cliente real por mock                         |
| CLI / scripts descartáveis       | `RedmineClient` direto | Sem necessidade de singleton ou compartilhamento       |
| Web server (Actix/Axum/Rocket)   | `Arc<RedmineClient>` | Injetado como estado da aplicação                        |

---

## Retry com Exponential Backoff

Operações de rede falham. O wrapper não implementa retry automático internamente para manter a
previsibilidade, mas fornece um utilitário genérico que pode ser usado em qualquer chamada.

### Função de Retry Completa

```rust,ignore
use std::time::Duration;
use std::thread::sleep;

/// Executa uma operação com retry exponencial e jitter.
///
/// Respeita o [`429 Too Many Requests`] e [`503 Service Unavailable`] do Redmine.
///
/// # Parâmetros
///
/// * `operation`       – Closure que retorna `Result<T, RedmineError>`
/// * `max_retries`     – Número máximo de tentativas (default: 4)
/// * `base_delay_ms`   – Atraso inicial em milissegundos (default: 500)
/// * `operation_name`  – Nome legível para logging (ex: "issues.list")
///
/// # Erro
///
/// Retorna o último erro encontrado se todas as tentativas falharem.
fn with_retry<F, T>(
    mut operation: F,
    max_retries: u32,
    base_delay_ms: u64,
    operation_name: &str,
) -> Result<T, RedmineError>
where
    F: FnMut() -> Result<T, RedmineError>,
{
    let mut last_error = None;

    for attempt in 1..=max_retries {
        match operation() {
            Ok(value) => return Ok(value),
            Err(err) => {
                let is_retryable = err.is_retryable();
                log::warn!(
                    "[{}] tentativa {}/{} falhou: {} (retryável: {})",
                    operation_name,
                    attempt,
                    max_retries,
                    err,
                    is_retryable,
                );

                if !is_retryable || attempt == max_retries {
                    return Err(err);
                }

                // Exponential backoff com jitter de ±25%
                let delay_ms = base_delay_ms * 2u64.pow(attempt - 1);
                let jitter_factor = 0.75 + (rand::random::<f64>() * 0.5); // 0.75 .. 1.25
                let actual_delay = (delay_ms as f64 * jitter_factor) as u64;

                log::info!(
                    "[{}] aguardando {}ms antes da tentativa {}",
                    operation_name,
                    actual_delay,
                    attempt + 1,
                );

                sleep(Duration::from_millis(actual_delay));
                last_error = Some(err);
            }
        }
    }

    Err(last_error.unwrap_or_else(|| {
        RedmineError::internal("retry exaurido sem erro original")
    }))
}

/// Extensão para `RedmineError` que determina se o erro é recuperável.
impl RedmineError {
    fn is_retryable(&self) -> bool {
        use redmine_wrapper_rs::core::errors::ErrorCategory;
        matches!(
            self.category(),
            ErrorCategory::RateLimited       // 429
                | ErrorCategory::ServiceUnavailable // 503
                | ErrorCategory::Timeout       // tempo limite excedido
                | ErrorCategory::Network       // falha de conexão
        )
    }
}
```

### Uso

```rust,ignore
let issues = with_retry(
    || client.issues().list(0, 25),
    4,
    500,
    "issues.list",
)?;
```

### Tabela de Decisão de Retry

| Código HTTP | Categoria              | Retry? | Delay sugerido | Justificativa                                    |
|-------------|------------------------|--------|----------------|--------------------------------------------------|
| 429         | `RateLimited`          | Sim    | 1–5s           | Redmine impõe rate limiting por IP/token         |
| 503         | `ServiceUnavailable`   | Sim    | 2–10s          | Manutenção ou sobrecarga temporária              |
| 502 / 504   | `Network`              | Sim    | 1–3s           | Proxy/Balanceador com falha momentânea           |
| 401         | `Unauthorized`         | Não    | —              | Token inválido — retry não resolve               |
| 403         | `Forbidden`            | Não    | —              | Permissão negada — não adianta repetir           |
| 404         | `NotFound`             | Não    | —              | Recurso inexistente — não vai aparecer            |
| 422         | `UnprocessableEntity`  | Não    | —              | Erro de validação — o payload precisa ser corrigido |
| 500         | `InternalServerError`  | Talvez | 3–5s           | Erro interno do Redmine — pode ser intermitente  |

---

## Estratégias de Cache

Cache reduz chamadas à API e melhora a latência. Duas estratégias são suportadas: cache em
memória simples (`HashMap`) e cache com expiração temporal (`TimedCache`).

### Cache Simples com HashMap

Ideal para dados que mudam com pouca frequência (ex: lista de trackers, status,
enumerations).

```rust,ignore
use std::collections::HashMap;
use std::sync::Mutex;
use redmine_wrapper_rs::types::Tracker;

pub struct TrackerCache {
    client: Arc<RedmineClient>,
    cache: Mutex<HashMap<&'static str, Vec<Tracker>>>,
}

impl TrackerCache {
    pub fn new(client: Arc<RedmineClient>) -> Self {
        Self {
            client,
            cache: Mutex::new(HashMap::new()),
        }
    }

    pub fn get_trackers(&self) -> Result<Vec<Tracker>, RedmineError> {
        let mut cache = self.cache.lock().unwrap();

        if let Some(trackers) = cache.get("trackers") {
            log::debug!("cache hit: trackers ({} itens)", trackers.len());
            return Ok(trackers.clone());
        }

        log::debug!("cache miss: trackers");
        let trackers = self.client.trackers().list()?;

        cache.insert("trackers", trackers.clone());
        Ok(trackers)
    }

    pub fn invalidate(&self, key: &'static str) {
        self.cache.lock().unwrap().remove(key);
    }

    pub fn clear(&self) {
        self.cache.lock().unwrap().clear();
    }
}
```

### TimedCache com Expiração

Para dados que mudam com frequência moderada mas aceitam uma janela de desatualização
(ex: projectos, issues, usuários).

```rust,ignore
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};
use redmine_wrapper_rs::types::Project;

#[derive(Clone)]
struct TimedEntry<T> {
    data: T,
    expires_at: Instant,
}

pub struct TimedCache<T> {
    inner: Mutex<HashMap<String, TimedEntry<T>>>,
    ttl: Duration,
}

impl<T: Clone> TimedCache<T> {
    pub fn new(ttl: Duration) -> Self {
        Self {
            inner: Mutex::new(HashMap::new()),
            ttl,
        }
    }

    pub fn get(&self, key: &str) -> Option<T> {
        let cache = self.inner.lock().unwrap();
        cache.get(key).and_then(|entry| {
            if Instant::now() < entry.expires_at {
                log::debug!("TimedCache hit: {}", key);
                Some(entry.data.clone())
            } else {
                log::debug!("TimedCache expired: {}", key);
                None
            }
        })
    }

    pub fn set(&self, key: String, value: T) {
        let mut cache = self.inner.lock().unwrap();
        cache.insert(key, TimedEntry {
            data: value,
            expires_at: Instant::now() + self.ttl,
        });
    }

    pub fn remove(&self, key: &str) {
        self.inner.lock().unwrap().remove(key);
    }

    pub fn clear_expired(&self) {
        let mut cache = self.inner.lock().unwrap();
        cache.retain(|_, entry| Instant::now() < entry.expires_at);
    }

    pub fn clear(&self) {
        self.inner.lock().unwrap().clear();
    }
}

// Exemplo de uso: cache de projectos com TTL de 5 minutos
fn exemplo_cache_projetos() -> Result<(), Box<dyn std::error::Error>> {
    let client = Arc::new(RedmineClient::new(
        RedmineConfig::builder()
            .base_url("https://redmine.exemplo.com")
            .token("token")
            .auth_method(AuthMethod::ApiKey)
            .build()?,
    )?);

    let cache: Arc<TimedCache<Vec<Project>>> = Arc::new(TimedCache::new(Duration::from_secs(300)));

    let projetos = match cache.get("all") {
        Some(p) => p,
        None => {
            let p = client.projects().list(0, 100)?;
            cache.set("all".to_string(), p.clone());
            p
        }
    };

    println!("{} projectos", projetos.len());
    Ok(())
}
```

### Tabela de Decisão de Cache

| Tipo de dado             | Estratégia   | TTL sugerido | Motivo                                  |
|--------------------------|--------------|--------------|------------------------------------------|
| Trackers / Status / Enums | `HashMap`    | Infinito     | Raramente mudam (quase estáticos)        |
| Projectos                | `TimedCache` | 5–15 min     | Podem ser criados/arquivados             |
| Usuários                 | `TimedCache` | 5–10 min     | Podem ser desativados                    |
| Issues (listas)          | Sem cache    | —            | Mutam rápido — melhor usar filtros       |
| Issues (individuais)     | `TimedCache` | 30–60s       | Cache curto para detalhes de issue       |
| Roles / Groups           | `HashMap`    | Infinito     | Mudança rara, requer admin               |

---

## Dicas de Performance

### Filtros no Servidor

Sempre filtre no lado do servidor. O Redmine aceita filtros via query string. Isso reduz o
payload e o processamento.

```rust,ignore
use redmine_wrapper_rs::resources::issues::IssueFilter;

// RUIM: buscar tudo e filtrar em memória
let todas = client.issues().list(0, 100)?;
let abertas: Vec<_> = todas.into_iter()
    .filter(|i| i.status.name == "New")
    .collect();

// BOM: deixar o Redmine filtrar
let filtro = IssueFilter::new()
    .with_status_id("open")
    .with_assigned_to_id("me")
    .with_created_on(">=2025-01-01");

let issues = client.issues().list_filtered(0, 25, &filtro)?;
```

### Rate Limit

O wrapper implementa rate limiting interno com sliding window. Configure o limite máximo de
requisições por segundo (RPS) conforme o plano da sua instância Redmine.

```rust,ignore
use redmine_wrapper_rs::config::RedmineConfig;
use redmine_wrapper_rs::core::constants;

let config = RedmineConfig::builder()
    .base_url("https://redmine.exemplo.com")
    .token("token")
    .auth_method(AuthMethod::ApiKey)
    // Redmine oficial: ~60 req/min => 1 RPS
    // Ajuste conforme plano e instância
    .max_rps(1)
    .build()?;
```

### Timeout

Configure timeouts para evitar que chamadas fiquem pendentes indefinidamente.

```rust,ignore
use std::time::Duration;

let config = RedmineConfig::builder()
    .base_url("https://redmine.exemplo.com")
    .token("token")
    .auth_method(AuthMethod::ApiKey)
    .timeout(Duration::from_secs(30))   // timeout total da requisição
    .connect_timeout(Duration::from_secs(10)) // timeout de conexão TCP
    .build()?;
```

### per_page

O Redmine retorna 25 itens por página por padrão. Para reduzir o número de chamadas,
ajuste `per_page` para o máximo suportado (100 no Redmine 5.x).

```rust,ignore
use redmine_wrapper_rs::http::pagination::PaginationParams;

// Máximo por página: 100
let params = PaginationParams::new(0, 100);
let issues = client.issues().list_paginated(&params)?;
```

### Sumário de Boas Práticas

| Prática                     | Impacto                    | Recomendação                          |
|-----------------------------|----------------------------|---------------------------------------|
| Filtrar no servidor         | Reduz payload e latência   | Sempre usar `IssueFilter` e similares |
| Paginar com per_page=100    | Menos requisições          | Usar `PaginationParams` com limit=100 |
| Rate limit (1 RPS)          | Evita bloqueio 429         | Configurar `max_rps`                  |
| Timeout (30s)               | Libera recursos            | Configurar `timeout` e `connect_timeout` |
| Cache de dados estáticos    | Zero latência em leituras  | Usar `HashMap` ou `TimedCache`        |
| Retry apenas em erros 429/503 | Evita agravar cenários    | Usar `is_retryable()` para decidir   |

---

## Impersonação (X-Redmine-Switch-User)

O Redmine oferece um mecanismo de **impersonação** que permite que um administrador realize
ações em nome de outro usuário sem conhecer sua senha. Isso é feito através do header HTTP
`X-Redmine-Switch-User`.

### Como Funciona

1. O usuário autenticado (via API key) **deve ser administrador**.
2. O header `X-Redmine-Switch-User` é enviado com o login (ou ID) do usuário a ser impersonado.
3. Todas as operações subsequentes são executadas **como se fossem** do usuário impersonado,
   respeitando as permissões dele.

### Uso no Wrapper

O `RedmineClient` expõe o método `impersonate()` que retorna um novo cliente compartilhando
o mesmo `HttpClient` mas com o header de impersonação configurado.

```rust,ignore
use redmine_wrapper_rs::RedmineClient;
use redmine_wrapper_rs::config::{AuthMethod, RedmineConfig};

fn exemplo_impersonacao() -> Result<(), Box<dyn std::error::Error>> {
    let admin_client = RedmineClient::new(
        RedmineConfig::builder()
            .base_url("https://redmine.exemplo.com")
            .token("token_do_admin")
            .auth_method(AuthMethod::ApiKey)
            .build()?,
    )?;

    // Cria um cliente impersonando o usuário "joao.silva"
    let joao_client = admin_client.impersonate("joao.silva")?;

    // Cria uma issue como se fosse o João
    let issue = joao_client.issues().create(IssueCreatePayload {
        project_id: 1,
        subject: "Issue criada via impersonação".into(),
        description: "O João vê isso como se ele mesmo tivesse criado.".into(),
        ..Default::default()
    })?;

    println!("Issue #{} criada como João", issue.id);

    // Remove a impersonação — volta a agir como admin
    let admin_client = joao_client.reset_impersonation()?;
    // Equivalente a: admin_client.impersonate("")

    Ok(())
}
```

### Requisitos

| Requisito               | Detalhe                                                    |
|-------------------------|------------------------------------------------------------|
| Permissão               | Usuário autenticado deve ser **administrador**             |
| Formato do identificador| Login do usuário (`String`) ou ID (`u64`)                  |
| Header HTTP             | `X-Redmine-Switch-User: login_do_usuario`                  |
| Escopo                  | Todas as operações no cliente impersonado                  |
| Reset                   | Enviar `X-Redmine-Switch-User` vazio ou chamar `reset_impersonation()` |

### Casos de Uso

- **Automação em nome de usuários**: criar issues, timesheets ou comentários como se fossem
  do usuário final.
- **Suporte técnico**: reproduzir problemas ou executar ações corretivas na conta de um
  usuário sem solicitar credenciais.
- **Auditoria**: ações ficam registradas no histórico do Redmine como executadas pelo
  usuário impersonado, mantendo a trilha de auditoria.

### Limitações

- Apenas administradores podem impersonar.
- O administrador precisa do **login exato** do usuário.
- Não é possível impersonar outro administrador (restrição do Redmine).

---

## Upload de Arquivos

O upload de arquivos no Redmine é um **processo de 2 passos**:

1. **Upload do binário** → recebe um `token` de upload.
2. **Associação do token** → vincula o arquivo a um recurso (issue, projecto, wiki, etc.).

### Passo 1: Upload Token

```rust,ignore
use std::fs;
use redmine_wrapper_rs::types::file::UploadResult;

fn upload_arquivo(
    client: &RedmineClient,
    caminho: &str,
    content_type: &str,
    filename: &str,
) -> Result<UploadResult, RedmineError> {
    let dados = fs::read(caminho)
        .map_err(|e| RedmineError::internal(format!("falha ao ler arquivo: {}", e)))?;

    let upload = client.files().upload(&dados, content_type, filename)?;

    log::info!(
        "Upload concluído: token={}, size={}",
        upload.token,
        dados.len(),
    );

    Ok(upload)
}
```

### Passo 2: Associação a uma Issue

```rust,ignore
use redmine_wrapper_rs::types::issue::IssueUpdatePayload;
use redmine_wrapper_rs::types::file::UploadToken;

fn anexar_a_issue(
    client: &RedmineClient,
    issue_id: u64,
    upload_token: &UploadToken,
    filename: &str,
    content_type: &str,
) -> Result<(), RedmineError> {
    let payload = IssueUpdatePayload {
        notes: Some("Anexando arquivo via API".into()),
        uploads: Some(vec![UploadToken {
            token: upload_token.token.clone(),
            filename: filename.to_string(),
            content_type: content_type.to_string(),
        }]),
        ..Default::default()
    };

    client.issues().update(issue_id, &payload)?;
    log::info!("Arquivo '{}' anexado à issue #{}", filename, issue_id);
    Ok(())
}
```

### Fluxo Completo

```rust,ignore
fn upload_e_associar(
    client: &RedmineClient,
    issue_id: u64,
    caminho: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Passo 1: upload do binário
    let upload = client.files().upload(
        &std::fs::read(caminho)?,
        "application/pdf",
        "relatorio.pdf",
    )?;

    // Passo 2: associar à issue
    client.issues().update(
        issue_id,
        &IssueUpdatePayload {
            notes: Some("Relatório anexado".into()),
            uploads: Some(vec![UploadToken {
                token: upload.token,
                filename: "relatorio.pdf".into(),
                content_type: "application/pdf".into(),
            }]),
            ..Default::default()
        },
    )?;

    println!("Arquivo anexado com sucesso à issue #{}", issue_id);
    Ok(())
}
```

### Tabela de Content-Type Comuns

| Tipo de arquivo | Extensão    | Content-Type                    |
|-----------------|-------------|---------------------------------|
| PDF             | `.pdf`      | `application/pdf`               |
| Imagem          | `.png`      | `image/png`                     |
| Imagem          | `.jpg`      | `image/jpeg`                    |
| Planilha        | `.xlsx`     | `application/vnd.openxmlformats-officedocument.spreadsheetml.sheet` |
| Texto           | `.txt`      | `text/plain`                    |
| ZIP             | `.zip`      | `application/zip`               |
| CSV             | `.csv`      | `text/csv`                      |

### Observações

- O token de upload **expira** após alguns minutos (tipicamente 30 minutos no Redmine).
- Associe o token ao recurso imediatamente após o upload.
- O tamanho máximo do arquivo é definido pela configuração do servidor Redmine
  (padrão: 5 MB, configurável via `attachment_max_size`).
- O método `upload()` faz uma requisição `POST /uploads.json` com o conteúdo binário
  diretamente no body (não é multipart/form-data).

---

## Monitoramento de Erros com UUIDs

Cada requisição feita pelo wrapper gera um **UUID v7** (correlation ID) que é incluído em
todos os logs e erros. Isso permite rastrear uma requisição individual através de todo o
sistema.

### Correlation ID Automático

```rust,ignore
use redmine_wrapper_rs::RedmineClient;
use redmine_wrapper_rs::config::RedmineConfig;

let client = RedmineClient::new(
    RedmineConfig::builder()
        .base_url("https://redmine.exemplo.com")
        .token("token")
        .auth_method(AuthMethod::ApiKey)
        .build()?,
)?;

// Cada chamada gera automaticamente um UUID v7
let result = client.issues().get(42);
// Log interno: [a1b2c3d4-e5f6-7890-abcd-ef1234567890] issues.get(42)

if let Err(err) = result {
    // O UUID está disponível no erro para rastreamento
    eprintln!("Erro ao buscar issue: {}", err);
    eprintln!("Correlation ID: {}", err.correlation_id());
}
```

### Integração com Sentry

```rust,ignore
use redmine_wrapper_rs::core::errors::RedmineError;

fn reportar_para_sentry(err: &RedmineError) {
    sentry::with_scope(|scope| {
        scope.set_tag("correlation_id", &err.correlation_id().to_string());
        scope.set_tag("operation", err.operation());
        scope.set_tag("http_status", &err.status_code().map(|c| c.to_string()).unwrap_or_default());
        scope.set_tag("error_category", &err.category().to_string());
        scope.set_extra("url", err.url().map(|u| u.to_string()).unwrap_or_default());
        scope.set_extra("request_body", err.request_body().unwrap_or_default());
    });

    sentry::capture_error(err);
}

// Uso
if let Err(err) = client.issues().list(0, 25) {
    reportar_para_sentry(&err);
}
```

### Integração com OpenTelemetry

```rust,ignore
use opentelemetry::trace::{Tracer, SpanKind, TraceContextExt};
use opentelemetry::Context;
use redmine_wrapper_rs::core::errors::RedmineError;

fn criar_span_com_correlation_id(
    tracer: &impl Tracer,
    err: &RedmineError,
) {
    let correlation_id = err.correlation_id();

    let mut span = tracer
        .span_builder(format!("redmine.{}", err.operation()))
        .with_kind(SpanKind::Client)
        .start(tracer);

    span.set_attribute("correlation_id", correlation_id.to_string());
    span.set_attribute("http.status_code", err.status_code().unwrap_or(0) as i64);
    span.set_attribute("error.category", err.category().to_string());
    span.set_attribute("error.message", err.to_string());

    if let Some(url) = err.url() {
        span.set_attribute("http.url", url.to_string());
    }

    span.set_status(opentelemetry::trace::Status::error(err.to_string()));
    span.end();
}
```

### Estrutura do Log

```text
[2026-07-11T14:30:00Z INFO  redmine_wrapper::http::client]
  {correlation_id: "0191f0a0-1234-7000-8000-000000000001"}
  operation="issues.get"
  method=GET
  url="https://redmine.exemplo.com/issues/42.json"
  status=200
  duration_ms=145
```

### Benefícios

| Benefício                 | Descrição                                                     |
|---------------------------|---------------------------------------------------------------|
| Rastreabilidade           | Cada erro tem um UUID que pode ser buscado nos logs           |
| Debugging                 | Correlation ID permite ligar o erro ao payload e à requisição |
| Agregação                 | Sentry/OpenTelemetry agrupam erros iguais com contextos únicos|
| Auditoria                 | Saber exatamente qual operação falhou e com quais parâmetros  |

---

## Uso com Tokio/Async

O wrapper é **síncrono (blocking)** por padrão, usando `reqwest::blocking`. Para integrá-lo
em aplicações assíncronas (Tokio, Actix, Axum), utilize `spawn_blocking` para evitar bloquear
o runtime assíncrono.

### Exemplo com Tokio

```rust,ignore
use tokio::task::spawn_blocking;
use std::sync::Arc;
use redmine_wrapper_rs::RedmineClient;
use redmine_wrapper_rs::config::{AuthMethod, RedmineConfig};

async fn buscar_issues_async(
    client: Arc<RedmineClient>,
) -> Result<Vec<Issue>, RedmineError> {
    // spawn_blocking move o closure para uma thread dedicada do rayon/blocking pool
    let issues = spawn_blocking(move || {
        client.issues().list(0, 25)
    })
    .await
    .map_err(|join_err| {
        RedmineError::internal(format!("spawn_blocking panicked: {}", join_err))
    })??;

    Ok(issues)
}
```

### Semaphore para Controlar Concorrência

```rust,ignore
use tokio::sync::Semaphore;
use std::sync::Arc;

/// Pool de conexões Redmine com limite de concorrência.
///
/// Útil para evitar sobrecarregar a instância Redmine com muitas
/// chamadas simultâneas.
pub struct RedminePool {
    client: Arc<RedmineClient>,
    semaphore: Arc<Semaphore>,
}

impl RedminePool {
    pub fn new(client: Arc<RedmineClient>, max_concurrent: usize) -> Self {
        Self {
            client,
            semaphore: Arc::new(Semaphore::new(max_concurrent)),
        }
    }

    pub async fn call<F, T>(&self, operation: F) -> Result<T, RedmineError>
    where
        F: FnOnce(Arc<RedmineClient>) -> Result<T, RedmineError> + Send + 'static,
        T: Send + 'static,
    {
        let _permit = self.semaphore
            .acquire()
            .await
            .map_err(|e| RedmineError::internal(format!("semaphore closed: {}", e)))?;

        let client = Arc::clone(&self.client);

        spawn_blocking(move || operation(client))
            .await
            .map_err(|join_err| {
                RedmineError::internal(format!("task panicked: {}", join_err))
            })?
    }
}

// Uso
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Arc::new(RedmineClient::new(
        RedmineConfig::builder()
            .base_url("https://redmine.exemplo.com")
            .token("token")
            .auth_method(AuthMethod::ApiKey)
            .build()?,
    )?);

    let pool = RedminePool::new(client, 4); // máximo 4 chamadas simultâneas

    let issues = pool.call(|c| c.issues().list(0, 25)).await?;
    let projects = pool.call(|c| c.projects().list(0, 25)).await?;

    println!("{} issues, {} projects", issues.len(), projects.len());
    Ok(())
}
```

### Arquitetura Recomendada

```
┌─────────────────────────────────────────┐
│             Aplicação Async             │
│  ┌───────────┐  ┌───────────┐          │
│  │  Handler  │  │  Handler  │  ...      │
│  │  HTTP     │  │  Cron     │          │
│  └─────┬─────┘  └─────┬─────┘          │
│        │               │                │
│  ┌─────▼───────────────▼─────┐          │
│  │      spawn_blocking       │          │
│  │   (thread pool do Tokio)  │          │
│  └─────┬───────────────┬─────┘          │
│        │               │                │
│  ┌─────▼─────┐   ┌─────▼─────┐          │
│  │ Redmine    │   │ Redmine    │          │
│  │ Client     │   │ Client     │          │
│  │ (blocking) │   │ (blocking) │          │
│  └───────────┘   └───────────┘          │
│        │               │                │
│        ▼               ▼                │
│    ┌──────────────────────────┐          │
│    │   Rate Limiter (Mutex)   │          │
│    └──────────────────────────┘          │
└─────────────────────────────────────────┘
```

### Avisos Importantes

| Situação                          | Problema                                                   | Solução                                        |
|-----------------------------------|------------------------------------------------------------|------------------------------------------------|
| Chamar cliente blocking no async  | Bloqueia o runtime Tokio, causando starvation             | Usar `spawn_blocking`                          |
| Muitas chamadas simultâneas       | Sobrecarrega o Redmine e pode causar 429                  | Usar `Semaphore` para limitar concorrência     |
| `Arc<RedmineClient>` compartilhado| Rate limiter com `Mutex` pode ser ponto de contenção      | Usar `Arc<RedmineClient>` por pool de threads  |
| Panic no `spawn_blocking`         | Erro é capturado como `JoinError`                         | Tratar o `JoinError` e converter para `RedmineError` |

### Exemplo Completo: Web Server com Actix

```rust,ignore
use actix_web::{web, App, HttpServer, HttpResponse};
use std::sync::Arc;
use tokio::task::spawn_blocking;
use redmine_wrapper_rs::RedmineClient;
use redmine_wrapper_rs::config::{AuthMethod, RedmineConfig};

struct AppState {
    client: Arc<RedmineClient>,
}

async fn listar_issues(state: web::Data<AppState>) -> HttpResponse {
    let client = Arc::clone(&state.client);

    let result = spawn_blocking(move || {
        client.issues().list(0, 25)
    })
    .await;

    match result {
        Ok(Ok(issues)) => HttpResponse::Ok().json(issues),
        Ok(Err(err)) => {
            log::error!("Erro Redmine: {} (correlation: {})", err, err.correlation_id());
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": err.to_string(),
                "correlation_id": err.correlation_id().to_string(),
            }))
        }
        Err(join_err) => {
            log::error!("spawn_blocking panic: {}", join_err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = Arc::new(
        RedmineClient::new(
            RedmineConfig::builder()
                .base_url("https://redmine.exemplo.com")
                .token(std::env::var("REDMINE_API_TOKEN").unwrap())
                .auth_method(AuthMethod::ApiKey)
                .build()
                .unwrap(),
        )
        .unwrap(),
    );

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                client: Arc::clone(&client),
            }))
            .route("/issues", web::get().to(listar_issues))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

---

> **Nota**: Este guia assume a configuração de autenticação via **API key** (método
> `AuthMethod::ApiKey`). O Redmine **não** suporta OAuth nativamente. Para instâncias
> com plugins de OAuth, consulte a documentação do plugin específico.
