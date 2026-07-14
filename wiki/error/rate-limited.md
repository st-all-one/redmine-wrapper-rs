# Rate Limited

**Categoria:** `ErrorCategory::RateLimited`
**HTTP Status:** 429
**Gatilho:** Limite de requisições excedido

## Causa

Muitas requisições em curto período no servidor.

## Exemplo

```rust,ignore
match result {
    Err(RedmineError::RateLimited { retry_after, .. }) => {
        if let Some(secs) = retry_after {
            eprintln!("Rate limited — aguarde {secs}s");
        }
    }
    _ => {}
}
```

## Prevenção

Ajuste `max_rps` na configuração do cliente (padrão: 10). O cliente já implementa rate limiting local com sliding window.
