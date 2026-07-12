# Rate Limited

**Categoria:** `ErrorCategory::RateLimited`
**HTTP Status:** 429
**Gatilho:** Limite de requisições excedido

## Causa

Muitas requisições em curto período.

## Exemplo

```rust,ignore
match result {
    Err(RedmineError::Api { category: ErrorCategory::RateLimited, detail, instance, .. }) => {
        eprintln!("{detail} (correlation: {instance})");
    }
    _ => {}
}
```

## Prevenção

Ajuste `max_rps` na configuração do cliente (padrão: 10). O cliente já implementa rate limiting local com sliding window.
