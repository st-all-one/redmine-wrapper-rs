# Network Error

**Categoria:** `ErrorCategory::NetworkError`
**HTTP Status:** 503
**Gatilho:** Erro de conexão

## Causa

Falha de rede, DNS ou conexão recusada.

## Exemplo

```rust,ignore
match result {
    Err(RedmineError::Api { category: ErrorCategory::NetworkError, detail, instance, .. }) => {
        eprintln!("{detail} (correlation: {instance})");
    }
    _ => {}
}
```

## Prevenção

Verifique a conectividade com o servidor Redmine. Confirme que `base_url` está correto.
