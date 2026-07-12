# Authorization Denied

**Categoria:** `ErrorCategory::AuthorizationDenied`
**HTTP Status:** 403
**Gatilho:** Usuário não tem permissão para o recurso

## Causa

O usuário autenticado não possui o papel ou permissão necessária.

## Exemplo

```rust,ignore
match result {
    Err(RedmineError::Api { category: ErrorCategory::AuthorizationDenied, detail, instance, .. }) => {
        eprintln!("{detail} (correlation: {instance})");
    }
    _ => {}
}
```

## Prevenção

Verifique as permissões do usuário no projeto. Consulte `GET /roles/{id}.json` para lista de permissões disponíveis.
