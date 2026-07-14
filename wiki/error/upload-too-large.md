# Upload Too Large

**Categoria:** `ErrorCategory::UploadTooLarge`
**HTTP Status:** 413
**Gatilho:** Arquivo excede tamanho máximo permitido

## Causa

O arquivo enviado excede o limite configurado no servidor Redmine (geralmente 5MB por padrão).

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

Verifique o limite de upload no servidor Redmine (Administração → Configurações → Anexos). Comprima ou divida o arquivo.
