//! Conversão de `--params` em query string e resolução de caminhos.

use percent_encoding::{AsciiSet, CONTROLS, utf8_percent_encode};
use serde_json::Value;

use crate::error::CliError;

/// Lista de pares (`chave`, `valor`) de query string.
pub type QueryPairs = Vec<(String, String)>;

/// Conjunto de caracteres escapados em um segmento de caminho.
const PATH_SEGMENT: &AsciiSet = &CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'#')
    .add(b'<')
    .add(b'>')
    .add(b'?')
    .add(b'`')
    .add(b'{')
    .add(b'}')
    .add(b'/')
    .add(b'%');

/// Percent-encoda um valor para uso como segmento de caminho.
#[must_use]
pub fn encode_segment(value: &str) -> String {
    utf8_percent_encode(value, PATH_SEGMENT).to_string()
}

/// Substitui placeholders `{nome}` do template por valores escapados.
///
/// # Errors
/// Template malformado ou placeholder sem valor correspondente.
pub fn resolve_path(template: &str, vars: &[(&str, String)]) -> Result<String, CliError> {
    let mut output = String::with_capacity(template.len());
    let mut rest = template;

    while let Some(start) = rest.find('{') {
        output.push_str(&rest[..start]);
        let after = &rest[start + 1..];
        let end = after.find('}').ok_or_else(|| {
            CliError::Internal(format!("template de caminho inválido: {template}"))
        })?;
        let name = &after[..end];
        let value = vars
            .iter()
            .find(|(key, _)| *key == name)
            .map(|(_, value)| value)
            .ok_or_else(|| {
                CliError::Usage(format!("informe --{name} (necessário em {template})"))
            })?;
        output.push_str(&encode_segment(value));
        rest = &after[end + 1..];
    }
    output.push_str(rest);
    Ok(output)
}

/// Achata um objeto JSON em pares de query string.
///
/// Escalares viram `chave=valor`; arrays de escalares viram `chave[]=valor`;
/// objetos aninhados viram `chave[sub]=valor`; `null` é omitido.
///
/// # Errors
/// `--params` que não seja objeto ou array com elementos não escalares.
pub fn params_to_query(value: &Value) -> Result<QueryPairs, CliError> {
    let object = value
        .as_object()
        .ok_or_else(|| CliError::Usage("--params de consulta deve ser um objeto JSON".into()))?;
    let mut pairs = Vec::new();
    for (key, value) in object {
        flatten(Some(key), value, &mut pairs)?;
    }
    Ok(pairs)
}

fn flatten(prefix: Option<&str>, value: &Value, pairs: &mut QueryPairs) -> Result<(), CliError> {
    match value {
        Value::Null => Ok(()),
        Value::Bool(flag) => {
            push(prefix, if *flag { "1" } else { "0" }, pairs);
            Ok(())
        }
        Value::Number(number) => {
            push(prefix, number.to_string(), pairs);
            Ok(())
        }
        Value::String(text) => {
            push(prefix, text.clone(), pairs);
            Ok(())
        }
        Value::Array(items) => {
            let key = prefix.ok_or_else(|| {
                CliError::Usage("array no topo de --params não é suportado".into())
            })?;
            for item in items {
                match item {
                    Value::Null => {}
                    Value::Bool(flag) => push(
                        Some(&format!("{key}[]")),
                        if *flag { "1" } else { "0" },
                        pairs,
                    ),
                    Value::Number(number) => {
                        push(Some(&format!("{key}[]")), number.to_string(), pairs);
                    }
                    Value::String(text) => {
                        push(Some(&format!("{key}[]")), text.clone(), pairs);
                    }
                    _ => {
                        return Err(CliError::Usage(format!(
                            "--params: array em `{key}` deve conter apenas escalares"
                        )));
                    }
                }
            }
            Ok(())
        }
        Value::Object(map) => {
            for (key, value) in map {
                let nested =
                    prefix.map_or_else(|| key.clone(), |parent| format!("{parent}[{key}]"));
                flatten(Some(&nested), value, pairs)?;
            }
            Ok(())
        }
    }
}

fn push(prefix: Option<&str>, value: impl Into<String>, pairs: &mut QueryPairs) {
    if let Some(key) = prefix {
        pairs.push((key.to_string(), value.into()));
    }
}

/// Aplica query fixa, sobrescrevendo chaves já presentes.
pub fn apply_fixed(pairs: &mut QueryPairs, fixed: &[(&str, &str)]) {
    for (key, value) in fixed {
        pairs.retain(|(existing, _)| existing != key);
        pairs.push(((*key).to_string(), (*value).to_string()));
    }
}

/// Define (ou substitui) uma chave de query.
pub fn set(pairs: &mut QueryPairs, key: &str, value: String) {
    pairs.retain(|(existing, _)| existing != key);
    pairs.push((key.to_string(), value));
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn flattens_scalars_arrays_objects_and_null() {
        let pairs = params_to_query(&json!({
            "status_id": "open",
            "set_filter": 1,
            "flag": true,
            "skip": null,
            "ids": [1, 2],
            "nested": {"a": "b", "c": [true, false]},
        }))
        .expect("params válidos");

        let mut sorted = pairs;
        sorted.sort();
        assert_eq!(
            sorted,
            vec![
                ("flag".to_string(), "1".to_string()),
                ("ids[]".to_string(), "1".to_string()),
                ("ids[]".to_string(), "2".to_string()),
                ("nested[a]".to_string(), "b".to_string()),
                ("nested[c][]".to_string(), "0".to_string()),
                ("nested[c][]".to_string(), "1".to_string()),
                ("set_filter".to_string(), "1".to_string()),
                ("status_id".to_string(), "open".to_string()),
            ]
        );
    }

    #[test]
    fn rejects_non_object_params() {
        let error = params_to_query(&json!([1, 2])).expect_err("array no topo");
        assert!(error.to_string().contains("objeto JSON"));
    }

    #[test]
    fn encodes_path_segments_and_rejects_missing_vars() {
        let path = resolve_path(
            "/projects/{project}/wiki/{title}.json",
            &[
                ("project", "my proj/x".to_string()),
                ("title", "Guia de Uso".to_string()),
            ],
        )
        .expect("caminho resolvido");
        assert_eq!(path, "/projects/my%20proj%2Fx/wiki/Guia%20de%20Uso.json");

        let error = resolve_path("/issues/{id}.json", &[]).expect_err("id ausente");
        assert!(error.to_string().contains("--id"));
    }

    #[test]
    fn fixed_query_overrides_existing_key() {
        let mut pairs = vec![
            ("include".to_string(), "x".to_string()),
            ("a".to_string(), "b".to_string()),
        ];
        apply_fixed(&mut pairs, &[("include", "allowed_statuses")]);
        let mut includes: Vec<&str> = pairs
            .iter()
            .filter(|(key, _)| key == "include")
            .map(|(_, value)| value.as_str())
            .collect();
        includes.sort_unstable();
        assert_eq!(includes, vec!["allowed_statuses"]);
    }
}
