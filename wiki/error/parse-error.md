# Parse Error

**Categoria:** `ErrorCategory::ParseError`
**HTTP Status:** 500
**Gatilho:** Resposta JSON inválida

## Causa

O servidor retornou uma resposta que não pôde ser desserializada no tipo esperado.

## Exemplo

```rust,ignore
match result {
    Err(RedmineError::Api { category: ErrorCategory::ParseError, detail, instance, .. }) => {
        eprintln!("{detail} (correlation: {instance})");
    }
    _ => {}
}
```

## Prevenção

Verifique o corpo da resposta no `ErrorContext.response_body`. Pode indicar mudança na API ou erro interno do Redmine.
