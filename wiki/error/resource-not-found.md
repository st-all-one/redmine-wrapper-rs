# Resource Not Found

**Categoria:** `ErrorCategory::ResourceNotFound`
**HTTP Status:** 404
**Gatilho:** Recurso inexistente

## Causa

O ID ou caminho do recurso não existe no Redmine.

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

Verifique se o ID do recurso está correto. Consulte a listagem antes de acessar um recurso específico.
