# Catálogo de Erros

Catálogo completo dos erros retornados pelo `redmine-wrapper-rs`, seguindo o padrão RFC 7807.

## Estrutura de um Erro

Todo erro `RedmineError::Api` contém:

| Campo | Tipo | Descrição |
|-------|------|-----------|
| `category` | `ErrorCategory` | Categoria semântica do erro |
| `status` | `u16` | Código HTTP |
| `detail` | `String` | Mensagem descritiva |
| `instance` | `String` | UUID v7 único para correlação |
| `context` | `ErrorContext` | Operação, erros da API, corpo da resposta |

## Categorias

| Categoria | HTTP | Descrição |
|-----------|------|-----------|
| [authentication-failed](./authentication-failed.md) | 401 | Chave de API ausente ou inválida |
| [authorization-denied](./authorization-denied.md) | 403 | Acesso negado ao recurso |
| [resource-not-found](./resource-not-found.md) | 404 | Recurso inexistente |
| [validation-error](./validation-error.md) | 422 | Payload inválido |
| [conflict](./conflict.md) | 409 | Conflito de versão (wiki) |
| [rate-limited](./rate-limited.md) | 429 | Limite de taxa excedido |
| [impersonation-failed](./impersonation-failed.md) | 412 | Impersonação inválida |
| [upload-too-large](./upload-too-large.md) | 413 | Upload excede tamanho máximo |
| [timeout](./timeout.md) | 504 | Tempo limite excedido |
| [network-error](./network-error.md) | 503 | Erro de rede |
| [parse-error](./parse-error.md) | 500 | Falha ao interpretar resposta |
| [internal-error](./internal-error.md) | 500 | Erro interno |

## Tratamento Genérico

```rust,ignore
use redmine_wrapper::core::errors::{RedmineError, ErrorCategory};

async fn safe_call<T>(result: Result<T, RedmineError>) -> Option<T> {
    match result {
        Ok(val) => Some(val),
        Err(RedmineError::Api { category, detail, instance, .. }) => {
            eprintln!("[{}] {} (UUID: {})", category, detail, instance);
            None
        }
        Err(e) => {
            eprintln!("Erro inesperado: {e}");
            None
        }
    }
}
```

## UUID v7 para Rastreamento

Cada erro `RedmineError::Api` contém um UUID v7 único no campo `instance`,
gerado no momento do erro. Use este UUID para correlacionar logs:

```rust,ignore
match result {
    Err(RedmineError::Api { instance, .. }) => {
        tracing::error!("Erro Redmine (correlation_id: {})", instance);
    }
    _ => {}
}
```
