# Network Error

**Categoria:** `ErrorCategory::NetworkError`
**HTTP Status:** 503
**Gatilho:** Erro de rede ou serviço indisponível

## Causa

Falha de conexão com o servidor Redmine (DNS, TLS, ou servidor fora do ar).

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

Verifique a conectividade de rede e se o servidor Redmine está operacional. Considere implementar retry com backoff.
