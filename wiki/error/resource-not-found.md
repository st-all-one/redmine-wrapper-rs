# Resource Not Found

**Categoria:** `ErrorCategory::ResourceNotFound`
**HTTP Status:** 404
**Gatilho:** Recurso não encontrado

## Causa

O ID ou caminho solicitado não existe.

## Exemplo

```rust,ignore
match result {
    Err(RedmineError::Api { category: ErrorCategory::ResourceNotFound, detail, instance, .. }) => {
        eprintln!("{detail} (correlation: {instance})");
    }
    _ => {}
}
```

## Prevenção

Verifique se o ID está correto. Use filtros de listagem para confirmar a existência do recurso antes de acessá-lo.
