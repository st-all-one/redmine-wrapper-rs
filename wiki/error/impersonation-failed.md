# Impersonation Failed

**Categoria:** `ErrorCategory::ImpersonationFailed`
**HTTP Status:** 412
**Gatilho:** `X-Redmine-Switch-User` inválido

## Causa

O header `X-Redmine-Switch-User` foi enviado com um login de usuário que não existe no Redmine.

## Exemplo

```rust,ignore
match result {
    Err(RedmineError::Api { category: ErrorCategory::ImpersonationFailed, detail, instance, .. }) => {
        eprintln!("{detail} (correlation: {instance})");
    }
    _ => {}
}
```

## Prevenção

Verifique se o login informado em `switch_user` existe no Redmine e se o token usado é de um administrador.
