# Validation Error

**Categoria:** `ErrorCategory::ValidationError`
**HTTP Status:** 422
**Gatilho:** Payload inválido

## Causa

Os dados enviados não passaram na validação do servidor (campo obrigatório ausente, formato inválido, etc.).

## Exemplo

```rust,ignore
match result {
    Err(RedmineError::Api { category: ErrorCategory::ValidationError, detail, instance, context, .. }) => {
        eprintln!("{detail} (correlation: {instance})");
        if let Some(errors) = context.api_errors {
            for e in errors {
                eprintln!("  → {e}");
            }
        }
    }
    _ => {}
}
```

## Prevenção

Revise os campos obrigatórios do payload. Verifique a mensagem de erro retornada no campo `context.api_errors` para detalhes.
