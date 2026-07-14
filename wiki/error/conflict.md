# Conflict

**Categoria:** `ErrorCategory::Conflict`
**HTTP Status:** 409
**Gatilho:** Conflito de versão

## Causa

Ocorre principalmente ao atualizar páginas wiki quando a versão no servidor difere da esperada.

## Exemplo

```rust,ignore
match result {
    Err(RedmineError::Api { category: ErrorCategory::Conflict, detail, instance, .. }) => {
        eprintln!("{detail} (correlation: {instance})");
    }
    _ => {}
}
```

## Prevenção

Recarregue o recurso antes de atualizar para obter a versão mais recente.
