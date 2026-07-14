# Particularidades da API do Redmine

## JSON Wrapping

A API REST do Redmine utiliza um padrão de **envelope** nas respostas JSON.
Recursos únicos vêm encapsulados numa chave com o nome do recurso, e listas
vêm num array com um campo `total_count` indicando o total de registros
disponíveis (não apenas os retornados na página atual).

**Exemplo — recurso único (`GET /issues/1.json`):**

```json
{
  "issue": {
    "id": 1,
    "subject": "Exemplo",
    "status": { "id": 1, "name": "Novo" }
  }
}
```

**Exemplo — lista (`GET /issues.json`):**

```json
{
  "issues": [
    { "id": 1, "subject": "Exemplo" },
    { "id": 2, "subject": "Outro" }
  ],
  "total_count": 2,
  "offset": 0,
  "limit": 25
}
```

### Como o wrapper lida com isso

O `HttpClient` expõe três métodos próprios para lidar com esse padrão:

| Método | Uso | Exemplo de caminho |
|--------|-----|-------------------|
| `get_single(path, key, query, op)` | GET de um recurso único | `get_single("/issues/1.json", "issue", ...)` |
| `post_single(path, key, body, op)` | POST (criação) de recurso | `post_single("/issues.json", "issue", &payload, ...)` |
| `get_paginated(path, item_key, params, op)` | GET de lista paginada | `get_paginated("/issues.json", "issues", params, ...)` |

Internamente, `get_single` e `post_single` fazem uma requisição GET/POST
comum, desserializam o JSON bruto num `serde_json::Value`, extraem o campo
indicado por `key` e desserializam-no no tipo `T` destino.

```rust,ignore
// Exemplo interno (simplificado):
pub fn get_single<T: DeserializeOwned>(
    &self,
    path: &str,
    key: &str,
    query: &[(&str, String)],
    op: &str,
) -> Result<T, RedmineError> {
    let v: serde_json::Value = self.get(path, query, op)?;
    let inner = v.get(key).ok_or_else(|| /* erro ParseError */)?;
    serde_json::from_value(inner.clone()).map_err(RedmineError::from)
}
```

Já `get_paginated` extrai o `item_key` (ex: `"issues"`) como `Vec<T>` e o
campo `total_count` como `u32`, retornando `(Vec<T>, u32)`.

Se o campo extraído não existir, o wrapper retorna um
`RedmineError::Api` com categoria `ParseError`.

---

## Paginação offset/limit

Diferentemente do padrão `page`/`per_page` do GitLab, o Redmine utiliza
**offset** e **limit** como parâmetros de paginação:

| Parâmetro | Descrição | Padrão | Máximo |
|-----------|-----------|--------|--------|
| `offset` | Deslocamento a partir do primeiro registro | `0` | — |
| `limit` | Número máximo de itens a retornar | `25` | `100` |

O limite máximo de **100 itens por página** é imposto pelo próprio servidor
Redmine. Valores acima de 100 são ignorados (silenciosamente truncados para 100).

O `total_count` vem **no corpo da resposta**, não em headers. Isso significa
que o cliente precisa desserializar o JSON completo para saber quantas páginas
existem.

### No wrapper

O `PaginationParams` permite controlar `offset` e `limit`:

```rust,ignore
use redmine_wrapper::http::pagination::PaginationParams;

// Página 1: registros 0–99
let p1 = PaginationParams::new(0, 100);

// Página 2: registros 100–199
let p2 = PaginationParams::new(100, 100);
```

Para obter **todos os registros** de uma vez, o `HttpClient` oferece
`get_all_paginated`, que faz requisições sucessivas incrementando o offset
até atingir `total_count`:

```rust,ignore
// Coleta todas as issues (várias chamadas internas)
let todas: Vec<Issue> = http.get_all_paginated(
    "/issues.json",
    "issues",
    &[],
    "issues.list_all",
)?;
```

---

## Autenticação via Header

O Redmine autentica via **`X-Redmine-API-Key`** no header HTTP. Não há
suporte a OAuth, Bearer tokens ou Basic Auth.

```rust,ignore
// O wrapper faz isso automaticamente:
// Header: X-Redmine-API-Key: <seu_token>
```

### Por que `?key=` é inseguro

A API também aceita a chave como parâmetro de query string:

```
GET /issues.json?key=seu_token_aqui
```

Isso é **expressamente desencorajado** pela [documentação oficial de
integração do Redmine](https://www.redmine.org/projects/redmine/wiki/Rest_api#Authentication)
pelos seguintes motivos:

1. **Logs do servidor** — a URL completa (incluindo `?key=...`) é registrada
   nos logs de acesso do servidor web, expondo a chave.
2. **Cache intermediário** — proxies e CDNs podem cachear a URL com a chave.
3. **Referrer Header** — a chave pode vazar no header `Referer` ao navegar
   para outros sites.
4. **Bookmarks** — a URL pode ser salva em bookmarks com a chave visível.

O wrapper **nunca** envia a chave via query string. A autenticação é sempre
feita via header `X-Redmine-API-Key`.

```rust,ignore
// No HttpClient::auth_headers():
if let Some(ref token) = self.config.token {
    headers.insert("X-Redmine-API-Key", HeaderValue::from_str(token).unwrap());
}
```

---

## Impersonação

Administradores podem atuar em nome de outro usuário enviando o header
**`X-Redmine-Switch-User`** com o nome de login do usuário alvo.

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

### Comportamento do servidor

- **Header presente + token de admin**: o Redmine executa a ação como o
  usuário especificado.
- **Header presente + token não-admin**: o Redmine ignora o header e
  executa como o dono do token.
- **Usuário inválido**: o servidor retorna **HTTP 412**
  (`ImpersonationFailed`). O wrapper mapeia automaticamente:

```rust,ignore
// Isto resulta num RedmineError::Api { category: ErrorCategory::ImpersonationFailed, status: 412, ... }
```

### Casos de uso

- Criar issues em nome de usuários que não têm acesso à API.
- Auditar permissões sem compartilhar a chave de admin.
- Scripts de automação que precisam refletir a identidade de um usuário
  específico.

---

## Upload em 2 Passos

O upload de arquivos no Redmine requer **duas etapas**:

### Passo 1: Obter token de upload

Envia-se o conteúdo binário do arquivo para `/uploads.json?filename=<nome>`.
O servidor retorna um **token** que representa o arquivo.

```rust,ignore
// Internamente, o AttachmentsResource faz:
let upload: UploadToken = http.post_binary(
    "/uploads.json?filename=relatorio.pdf",
    &bytes_do_pdf,
    "application/octet-stream",
    "attachments.upload",
)?;
// UploadToken { token: "abcd.1234.xxxx" }
```

### Passo 2: Associar o token ao recurso

Inclui-se o token no payload de criação/atualização do recurso (issue,
projeto, etc.):

```rust,ignore
// Criando uma issue com anexo:
let issue = client.issues.create(&CreateIssueRequest {
    subject: "Issue com anexo".into(),
    description: "Conforme discutido".into(),
    project_id: 1.into(),
    uploads: Some(vec![Upload {
        token: "abcd.1234.xxxx".into(),
        filename: "relatorio.pdf".into(),
        content_type: "application/pdf".into(),
        description: Some("Relatório mensal".into()),
    }]),
    ..Default::default()
})?;
```

### Por que 2 passos?

- O Redmine separa o armazenamento do binário da associação semântica.
- Permite reutilizar o mesmo token em múltiplos recursos (um arquivo
  anexado a várias issues).
- Evita timeout em uploads grandes durante operações longas de criação.

**Observação:** O token de upload é válido por tempo limitado
(geralmente algumas horas, configurável no servidor).

---

## Include Pattern

A API do Redmine usa **lazy loading** para associations. Por padrão, um GET
num recurso retorna apenas os campos básicos. Para obter dados relacionados
(journals, attachments, relations, etc.), usa-se o parâmetro `?include=`.

```rust,ignore
// Sem include — apenas campos básicos
let issue: Issue = client.issues.get(1)?;
// issue.journals é None, issue.attachments é None

// Com include — carrega jornais e anexos
let issue: Issue = client.issues.get_with_includes(1, &["journals", "attachments"])?;
// issue.journals é Some(vec![...]), issue.attachments é Some(vec![...])
```

### Valores comuns de `include`

| Inclusão | Descrição |
|----------|-----------|
| `journals` | Histórico de alterações da issue |
| `attachments` | Arquivos anexados |
| `relations` | Relações com outras issues |
| `children` | Subtarefas |
| `watchers` | Observadores (requer admin ou permissão) |
| `allowed_statuses` | Status disponíveis para transição |

### Limitação importante

**O `?include=` só funciona em GET de recurso único** (`GET /issues/1.json`).
Não é suportado em listagens (`GET /issues.json`). Para obter associations
numa lista, é necessário fazer uma requisição individual para cada recurso.

```rust,ignore
// ❌ Isto NÃO carrega journals dos itens da lista:
let issues: Vec<Issue> = client.issues.list(None, None)?;
// issues[i].journals será None para todos

// ✅ Solução: iterar e buscar cada um com includes
for issue in &issues {
    let full = client.issues.get_with_includes(issue.id, &["journals"])?;
    // processar full.journals
}
```

---

## Rate Limiting

O wrapper implementa **rate limiting client-side** usando uma janela
deslizante (sliding window) para evitar sobrecarga no servidor Redmine.

### Configuração

O limite máximo de requisições por segundo é configurável via `RedmineConfig`:

```rust,ignore
let client = RedmineClient::new(
    RedmineConfigBuilder::default()
        .base_url("https://redmine.exemplo.com")
        .token("token")
        .max_rps(5)
        .build()?,
)?;
```

Se não especificado, o padrão é **10 requisições/s** (definido em
`DEFAULT_MAX_RPS`).

### Funcionamento interno

O rate limiter (`SlidingWindow`) mantém um `VecDeque` com timestamps das
requisições recentes. Antes de cada requisição, o `HttpClient` chama:

```rust,ignore
fn acquire_rate_limit(&self) {
    if let Ok(mut limiter) = self.rate_limiter.lock() {
        limiter.acquire();
    }
}
```

O `SlidingWindow::acquire()`:

1. Remove timestamps anteriores a 1 segundo.
2. Se o número de requisições no último segundo já atingiu o limite,
   dorme (sleep) até que uma janela se abra.
3. Adiciona o timestamp atual.

### Rate limiting no servidor

O servidor Redmine **não possui rate limiting nativo**. A limitação no
servidor, quando existe, é configurada manualmente via nginx
(`limit_req_zone` / `limit_req`) ou outro proxy reverso.

O rate limiting client-side do wrapper serve como uma **proteção adicional**
para evitar que o cliente seja bloqueado por limitações do servidor.

---

## No OAuth

O Redmine **não suporta OAuth**. O único método de autenticação disponível
é a chave de API via header `X-Redmine-API-Key`.

### Comparação com outras plataformas

| Plataforma | Autenticação | Múltiplos métodos |
|------------|-------------|-------------------|
| Redmine | API Key (header) | Não (apenas key) |
| GitLab | OAuth, Personal Token, Job Token | Sim |
| GitHub | OAuth, PAT, GitHub App | Sim |
| Jira | Basic Auth, OAuth, PAT | Sim |

### Implicações

- **Sem refresh tokens** — a chave é válida até ser revogada manualmente.
- **Sem escopos** — a chave de um usuário tem as mesmas permissões do
  usuário. Não há escopos finos (leitura vs. escrita) como em PATs do
  GitHub/GitLab.
- **Sem OAuth module** — o wrapper não inclui e não precisa de módulo
  OAuth. A autenticação se resume a configurar o `token` no
  `RedmineConfig`.

```rust,ignore
// Toda a autenticação que o wrapper precisa:
let client = RedmineClient::new(
    RedmineConfigBuilder::default()
        .base_url("https://redmine.exemplo.com")
        .token("sua_chave_api")
        .build()?,
)?;
```

Para segurança adicional, recomenda-se:

- Usar chaves de API em variáveis de ambiente (`REDMINE_TOKEN`), nunca
  hardcoded.
- Criar usuários específicos para integração, com permissões mínimas
  necessárias.
- Rotacionar chaves periodicamente.
