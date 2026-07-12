# Primeiros Passos

Guia de instalação e primeira chamada à API com o `redmine-wrapper-rs`.

## Instalação

Adicione a dependência ao seu `Cargo.toml`:

```toml
[dependencies]
redmine-wrapper-rs = "0.1"
```

### Feature flags

| Flag          | Padrão  | Descrição                                       |
|---------------|---------|-------------------------------------------------|
| `rustls`      | `sim`   | Usa rustls como TLS nativo (ligação estática)   |
| `native-tls`  | `não`   | Usa a TLS do sistema operativo (OpenSSL/SChannel) |

Para trocar para `native-tls`:

```toml
[dependencies]
redmine-wrapper-rs = { version = "0.1", default-features = false, features = ["native-tls"] }
```

## Configuração

A estrutura [`RedmineConfig`](api-reference#RedmineConfig) agrupa todos os parâmetros de
ligação ao servidor Redmine. Utilize o padrão _builder_ para a construir.

| Campo          | Tipo               | Obrigatório | Descrição                                           |
|----------------|--------------------|-------------|-----------------------------------------------------|
| `base_url`     | `String`           | sim         | URL base do Redmine (ex: `https://redmine.exemplo.com`) |
| `token`        | `String`           | sim         | Chave de API do Redmine                             |
| `auth_method`  | `AuthMethod`       | não         | Método de autenticação (`ApiKey` por omissão)       |
| `switch_user`  | `Option<String>`   | não         | Nome de utilizador para _sudo_ na API               |
| `timeout`      | `Duration`         | não         | Timeout global para pedidos HTTP (padrão: 30s)      |
| `max_rps`      | `u32`              | não         | Máximo de pedidos por segundo (rate limiting local) |

### Variáveis de ambiente

| Variável           | Campo correspondente | Descrição                       |
|--------------------|----------------------|---------------------------------|
| `REDMINE_URL`      | `base_url`           | URL base da instância Redmine   |
| `REDMINE_TOKEN`    | `token`              | Chave de API do Redmine         |

Se estas variáveis estiverem definidas, o valor é usado como padrão no builder,
mas pode ser sobrescrito programaticamente.

Exemplo de construção:

```rust,ignore
use redmine_wrapper_rs::config::{AuthMethod, RedmineConfig};
use std::time::Duration;

let config = RedmineConfig::builder()
    .base_url("https://redmine.exemplo.com")
    .token("abc123def456")
    .auth_method(AuthMethod::ApiKey)
    .switch_user("joao")
    .timeout(Duration::from_secs(60))
    .max_rps(10)
    .build()?;
```

## Primeira chamada à API

O ponto de entrada é [`RedmineClient`](api-reference#RedmineClient). Todas as operações
são _blocking_ (síncronas).

```rust,ignore
use redmine_wrapper_rs::RedmineClient;
use redmine_wrapper_rs::config::RedmineConfig;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Carrega configuração (daqui podia vir de variáveis de ambiente)
    let config = RedmineConfig::builder()
        .base_url("https://redmine.exemplo.com")
        .token("abc123def456")
        .build()?;

    // 2. Cria o cliente (a conexão é lazy — só valida a URL neste ponto)
    let client = RedmineClient::new(config)?;

    // 3. Lista projectos
    let projetos = client.projects().list(0, 25)?;
    println!("Total de projectos: {}", projetos.len());
    for p in &projetos {
        println!("  [{}] {}", p.id, p.name);
    }

    // 4. Obtém dados do utilizador autenticado (endpoint /users/current.json)
    let user = client.users().current()?;
    println!("Autenticado como: {} <{}>", user.login, user.mail);

    Ok(())
}
```

## Tratamento de erros

A crate define o enum [`RedmineError`](errors#RedmineError) para todos os cenários de falha.

```rust,ignore
use redmine_wrapper_rs::RedmineClient;
use redmine_wrapper_rs::error::RedmineError;

fn listar(client: &RedmineClient) -> Result<(), RedmineError> {
    match client.projects().list(0, 25) {
        Ok(projetos) => {
            println!("OK — {} projectos", projetos.len());
            Ok(())
        }
        Err(RedmineError::Http { status, body }) => {
            eprintln!("Erro HTTP {}: {}", status, body);
            Err(RedmineError::Http { status, body })
        }
        Err(RedmineError::Api { errors }) => {
            eprintln!("Erro Redmine: {:?}", errors);
            Err(RedmineError::Api { errors })
        }
        Err(e) => {
            eprintln!("Erro inesperado: {}", e);
            Err(e)
        }
    }
}
```

## Paginação

A API do Redmine usa `offset` e `limit` nos _list endpoints_. O wrapper segue o mesmo
contrato: os métodos de listagem devolvem `Vec<T>` com os registos da página atual.

```rust,ignore
// Página 1: registos 0..24
let pagina1 = client.issues().list(0, 25)?;

// Página 2: registos 25..49
let pagina2 = client.issues().list(25, 25)?;
```

O _offset_ é baseado em zero. O _limit_ máximo aceite pelo Redmine é 100.
A crate **não** faz paginação eager — devolve apenas a página solicitada.
Cabe a quem consome iterar se precisar de todos os registos.

Exemplo de recolha total (paginação manual):

```rust,ignore
let mut todos = Vec::new();
let limit = 100;
let mut offset = 0;

loop {
    let page = client.issues().list(offset, limit)?;
    if page.is_empty() {
        break;
    }
    todos.extend(page);
    offset += limit;
}

println!("Total de issues: {}", todos.len());
```

## Imutabilidade

A configuração é **congelada** após a criação do `RedmineClient`. Não é possível
alterar `base_url`, `token`, `switch_user` ou qualquer outro campo depois de o
cliente estar construído. Para usar parâmetros diferentes, crie uma nova instância.

Isto garante que todos os pedidos partilham o mesmo contexto de autenticação e
rastreabilidade (UUID v7 é gerado por pedido, mas a identidade do cliente é fixa).
