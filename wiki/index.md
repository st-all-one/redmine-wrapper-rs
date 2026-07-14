# redmine-wrapper-rs — Documentação

Wrapper Rust tipado para a API REST do Redmine, construído em Rust síncrono (blocking)
com foco em segurança, rastreabilidade via UUID v7 e cobertura completa de todos os 86
endpoints da API.

## Índice

- [Primeiros Passos](getting-started)
- [Guia de Uso](usage)
- [Guia de Integração](integration)
- [Particularidades da API](api-particularities)
- [Referência da API](api-reference)
- [Erros](errors)

## Exemplo Rápido

```rust,ignore
use redmine_wrapper::RedmineClient;
use redmine_wrapper::RedmineConfigBuilder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RedmineClient::new(
        RedmineConfigBuilder::default()
            .base_url("https://redmine.exemplo.com")
            .token("seu_token_api")
            .build()?,
    )?;

    let projetos = client.projects.list()?;
    for p in &projetos {
        println!("#{}: {} ({})", p.id, p.name.as_deref().unwrap_or("?"), p.identifier.as_deref().unwrap_or("?"));
    }

    Ok(())
}
```

## Licença

Este projeto é licenciado sob a **MPL-2.0** (Mozilla Public License 2.0).

Consulte o arquivo [`LICENSE`](../LICENSE) para obter o texto completo da licença.
