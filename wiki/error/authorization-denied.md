# Authorization Denied

**Categoria:** `ErrorCategory::AuthorizationDenied`
**HTTP Status:** 403
**Gatilho:** Acesso negado ao recurso

## Causa

O token autenticado não tem permissão para acessar o recurso solicitado.

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

Verifique as permissões do usuário no Redmine. Alguns recursos (ex: `users.list`, `groups.list`) requerem permissão de administrador.
