# Primeiros Passos

Guia de instalação e primeira chamada à API com o `redmine-wrapper-rs`.

## Instalação

Adicione a dependência ao seu `Cargo.toml`:

```toml
[dependencies]
redmine-wrapper-rs = "0.1"
```

### Feature flags

| Flag          | Padrão | Descrição                                       |
|---------------|--------|-------------------------------------------------|
| `rustls`      | sim    | Usa rustls como TLS nativo (ligação estática)   |
| `native-tls`  | não    | Usa a TLS do sistema (OpenSSL/SChannel)         |

Para trocar para `native-tls`:

```toml
[dependencies]
redmine-wrapper-rs = { version = "0.1", default-features = false, features = ["native-tls"] }
```

## Configuração

Use o **builder** `RedmineConfigBuilder` para construir a configuração.
O builder valida que `base_url` não está vazio e aplica padrões para
os campos opcionais.

| Método do builder | Tipo | Obrigatório | Descrição |
|-------------------|------|-------------|-----------|
| `.base_url(url)` | `String` | sim | URL base do Redmine |
| `.token(token)` | `String` | não | Chave de API |
| `.switch_user(login)` | `String` | não | Impersonação (requer admin) |
| `.timeout_secs(s)` | `u64` | não | Timeout HTTP (padrão: 30s) |
| `.max_rps(n)` | `u32` | não | Rate limiting (padrão: 10 req/s) |

```rust,ignore
use redmine_wrapper::RedmineConfigBuilder;

let config = RedmineConfigBuilder::default()
    .base_url("https://redmine.exemplo.com")
    .token("sua-chave-api")
    .build()?;
```

### Variáveis de ambiente

| Variável        | Campo   | Descrição                     |
|-----------------|---------|-------------------------------|
| `REDMINE_URL`   | `url`   | URL base da instância Redmine |
| `REDMINE_TOKEN` | `token` | Chave de API                  |

## Primeira chamada à API

O ponto de entrada é `RedmineClient`. Todas as operações são síncronas (blocking).
O acesso aos recursos é via `Deref` — use `client.projects`, `client.issues`, etc.

```rust,ignore
use redmine_wrapper::RedmineClient;
use redmine_wrapper::RedmineConfigBuilder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RedmineClient::new(
        RedmineConfigBuilder::default()
            .base_url("https://redmine.exemplo.com")
            .token("sua-chave-api")
            .build()?,
    )?;

    // Lista projetos
    let projetos = client.projects.list()?;
    for p in &projetos {
        println!("#{}: {} ({})", p.id, p.name.as_deref().unwrap_or("?"), p.identifier.as_deref().unwrap_or("?"));
    }

    // Dados do usuário autenticado
    let account = client.my_account.get()?;
    println!("Autenticado como: {} {}", account.firstname.as_deref().unwrap_or(""), account.lastname.as_deref().unwrap_or(""));

    Ok(())
}
```

## Tratamento de erros

A crate define o enum `RedmineError` para todos os cenários de falha.

```rust,ignore
use redmine_wrapper::RedmineClient;
use redmine_wrapper::core::errors::RedmineError;

fn listar_projetos(client: &RedmineClient) -> Result<(), RedmineError> {
    match client.projects.list() {
        Ok(projetos) => {
            println!("OK — {} projetos", projetos.len());
            Ok(())
        }
        Err(RedmineError::Api { category, detail, instance, .. }) => {
            eprintln!("Erro da API [{}]: {} (id: {})", category, detail, instance);
            Err(RedmineError::Api { category, status: 0, detail: detail.clone(), instance: instance.clone(), context: Box::default() })
        }
        Err(e) => {
            eprintln!("Erro inesperado: {e}");
            Err(e)
        }
    }
}
```

## Paginação

Os métodos de listagem (`client.issues.list()`, `client.projects.list()`, etc.)
fazem **auto-paginação** e retornam `Vec<T>` com todos os registros disponíveis.
Não é necessário gerenciar `offset`/`limit` manualmente.

```rust,ignore
// Todas as issues (várias requisições internas, se necessário)
let todas = client.issues.list(None)?;
println!("Total de issues: {}", todas.len());
```

## Imutabilidade

A configuração é congelada após a criação do `RedmineClient`. Não é possível
alterar `base_url`, `token` ou qualquer outro campo depois de o cliente estar
construído. Para usar parâmetros diferentes, crie uma nova instância.
