# Validation Error

**Categoria:** `ErrorCategory::ValidationError`
**HTTP Status:** 422
**Gatilho:** Dados inválidos

## Causa

O payload enviado não passou nas validações do Redmine (campo obrigatório ausente, valor inválido, referência inexistente).

## Exemplo

```rust,ignore
match result {
    Err(RedmineError::Api { category: ErrorCategory::ValidationError, detail, instance, .. }) => {
        eprintln!("{detail} (correlation: {instance})");
    }
    _ => {}
}
```

## Prevenção

Consulte o campo `api_errors` no `ErrorContext` para mensagens específicas. Ex: 'Subject não pode ficar em branco'.
