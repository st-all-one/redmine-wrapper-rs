# Internal Error

**Categoria:** `ErrorCategory::InternalError`
**HTTP Status:** 500 (interno)
**Gatilho:** Erro interno não categorizado

## Causa

Erro HTTP 500 sem categoria específica mapeada pelo wrapper.

## Exemplo

```rust,ignore
match result {
    Err(RedmineError::Api { category: ErrorCategory::InternalError, detail, instance, context, .. }) => {
        eprintln!("Erro interno do servidor: {detail}");
        if let Some(body) = context.response_body {
            eprintln!("Corpo: {body}");
        }
    }
    _ => {}
}
```

## Prevenção

Verifique os logs do servidor Redmine para identificar a causa. Pode ser um bug no servidor ou uma configuração incorreta.
