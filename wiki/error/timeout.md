# Timeout

**Categoria:** `ErrorCategory::Timeout`
**HTTP Status:** 504
**Gatilho:** Requisição excedeu o tempo limite

## Causa

O servidor não respondeu dentro do período configurado.

## Exemplo

```rust,ignore
match result {
    Err(RedmineError::Api { category: ErrorCategory::Timeout, detail, instance, .. }) => {
        eprintln!("{detail} (correlation: {instance})");
    }
    _ => {}
}
```

## Prevenção

Ajuste o timeout com `timeout: Some(Duration::from_secs(120))` para operações lentas, ou verifique a conectividade de rede.
