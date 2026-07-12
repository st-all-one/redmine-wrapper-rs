# Internal Error

**Categoria:** `ErrorCategory::InternalError`
**HTTP Status:** 500
**Gatilho:** Erro interno não categorizado

## Causa

Código HTTP sem mapeamento para uma categoria específica.

## Exemplo

```rust,ignore
match result {
    Err(RedmineError::Api { category: ErrorCategory::InternalError, detail, instance, .. }) => {
        eprintln!("{detail} (correlation: {instance})");
    }
    _ => {}
}
```

## Prevenção

Consulte o código HTTP e o corpo da resposta para diagnóstico. Reporte ao administrador do Redmine se persistir.
