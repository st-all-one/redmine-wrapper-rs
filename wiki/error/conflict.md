# Conflict

**Categoria:** `ErrorCategory::Conflict`
**HTTP Status:** 409
**Gatilho:** Conflito de versão

## Causa

Conflito ao atualizar página wiki devido a versão desatualizada.

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

Sempre leia a versão atual da página wiki antes de atualizar.
