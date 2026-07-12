# redmine-wrapper-rs

[![MPL-2.0]](https://mozilla.org/MPL/2.0/)

Wrapper Rust tipado, completo e idiomático para a API REST do Redmine.

## Instalação

```toml
[dependencies]
redmine-wrapper-rs = "0.1"
```

## Exemplo rápido

```rust
use redmine_wrapper::{RedmineClient, RedmineConfigBuilder};
use redmine_wrapper::types::issue::IssueFilter;

let client = RedmineClient::new(
    RedmineConfigBuilder::default()
        .base_url("https://redmine.example.com")
        .token("seu-api-key")
        .build()?
)?;

let issues = client.issues.list(Some(&IssueFilter {
    assigned_to_id: Some("me".into()),
    status_id: Some("open".into()),
    ..Default::default()
}))?;
println!("Issues abertas: {}", issues.len());
```

## Recursos

- 22 recursos da API (issues, projects, users, time entries, wiki, etc.)
- 86 métodos com tipos serde e validação em compile-time
- Rate limiting (sliding window, configurável)
- Rate limiting (sliding window, default 10 req/s)
- Paginação automática (coleta eager)
- Upload de arquivos em 2 passos
- Impersonação via `X-Redmine-Switch-User`
- Erros categorizados (RFC 7807) com correlation IDs (UUID v7)
- Documentação completa em português

## Documentação

A [wiki](wiki/) completa inclui:
- [Guia de início](wiki/getting-started.md)
- [Guia de uso — todos os resources](wiki/usage-guide.md)
- [Guia de integração](wiki/integration-guide.md) (DI, retry, cache, async)
- [Particularidades da API](wiki/particularities.md)
- [Referência da API](wiki/api-reference.md)
- [Catálogo de erros](wiki/error/errors.md) (12 categorias)

## Feature flags

| Flag | TLS backend |
|------|-------------|
| `default` / `rustls` | rustls (nativo Rust) |
| `native-tls` | OpenSSL / sistema |

## Licença

MPL-2.0
