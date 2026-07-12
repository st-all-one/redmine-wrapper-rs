# Authentication Failed

**Categoria:** `ErrorCategory::AuthenticationFailed`
**HTTP Status:** 401
**Gatilho:** Token ausente ou inválido

## Causa

A requisição não incluiu o header `X-Redmine-API-Key` ou o token fornecido é inválido.

## Exemplo

```rust,ignore
match result {
    Err(RedmineError::Api { category: ErrorCategory::AuthenticationFailed, detail, instance, .. }) => {
        eprintln!("{detail} (correlation: {instance})");
    }
    _ => {}
}
```

## Prevenção

Verifique se a chave de API está configurada corretamente no `RedmineConfig.token`. Gere uma nova chave em Minha Conta → Chave de API.
