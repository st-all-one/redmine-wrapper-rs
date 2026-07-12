# Upload Too Large

**Categoria:** `ErrorCategory::UploadTooLarge`
**HTTP Status:** 413
**Gatilho:** Arquivo excede tamanho máximo

## Causa

O arquivo enviado excede o limite configurado no servidor Redmine.

## Exemplo

```rust,ignore
match result {
    Err(RedmineError::Api { category: ErrorCategory::UploadTooLarge, detail, instance, .. }) => {
        eprintln!("{detail} (correlation: {instance})");
    }
    _ => {}
}
```

## Prevenção

Verifique o limite de upload no servidor (Administração → Configurações → Anexos). Compacte ou divida o arquivo.
