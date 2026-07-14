# Parse Error

**Categoria:** `ErrorCategory::ParseError`
**HTTP Status:** 500 (interno)
**Gatilho:** Falha ao interpretar a resposta JSON do servidor

## Causa

O servidor retornou um JSON inesperado (estrutura diferente da documentada, campo ausente, ou formato inválido).

## Exemplo

```rust,ignore
match result {
    Err(RedmineError::Api { category: ErrorCategory::ParseError, detail, instance, context, .. }) => {
        eprintln!("Falha ao parsear resposta: {detail}");
        if let Some(body) = context.response_body {
            eprintln!("Corpo recebido: {body}");
        }
    }
    _ => {}
}
```

## Prevenção

Isso geralmente indica um bug no wrapper ou uma mudança na API do Redmine. Verifique se a versão do wrapper é compatível com a versão do Redmine.
