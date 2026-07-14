# Timeout

**Categoria:** `ErrorCategory::Timeout`
**HTTP Status:** 504
**Gatilho:** Tempo limite da requisição excedido

## Causa

O servidor não respondeu dentro do prazo configurado no cliente.

## Exemplo

```rust,ignore
match result {
    Err(RedmineError::Timeout { duration, .. }) => {
        eprintln!("Timeout após {duration:?}");
    }
    _ => {}
}
```

## Prevenção

Aumente o timeout com `timeout_secs()` no builder. Verifique a latência da rede e do servidor Redmine.
