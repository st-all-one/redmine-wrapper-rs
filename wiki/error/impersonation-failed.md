# Impersonation Failed

**Categoria:** `ErrorCategory::ImpersonationFailed`
**HTTP Status:** 412
**Gatilho:** Impersonação inválida

## Causa

O header `X-Redmine-Switch-User` especifica um login de usuário inexistente ou inativo.

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

Verifique se o login informado existe e está ativo no Redmine.
