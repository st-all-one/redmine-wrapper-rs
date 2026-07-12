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
use redmine_wrapper_rs::RedmineClient;
use redmine_wrapper_rs::config::{AuthMethod, RedmineConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Cria um cliente apontando para uma instância Redmine
    let client = RedmineClient::new(
        RedmineConfig::builder()
            .base_url("https://redmine.exemplo.com")
            .token("seu_token_api_aqui")
            .auth_method(AuthMethod::ApiKey)
            .build()?,
    )?;

    // Lista os primeiros projectos (offset 0, limit 25)
    let projetos = client.projects().list(0, 25)?;
    println!("{} projectos encontrados", projetos.len());

    // Exibe o nome de cada projecto
    for projeto in &projetos {
        println!("- {} (id: {})", projeto.name, projeto.id);
    }

    Ok(())
}
```

## Licença

Este projeto é licenciado sob a **MPL-2.0** (Mozilla Public License 2.0).

Consulte o arquivo [`LICENSE`](../LICENSE) para obter o texto completo da licença.
