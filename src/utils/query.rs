// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/// Converte um filtro serializável em pares chave-valor para query string.
///
/// Útil para enviar filtros estruturados como parâmetros de URL.
pub fn filter_to_query<T: serde::Serialize>(
    filter: Option<&T>,
) -> Vec<(String, String)> {
    match filter {
        None => Vec::new(),
        Some(f) => {
            let value = serde_json::to_value(f).unwrap_or_default();
            flatten_value("", &value)
        }
    }
}

/// Achata recursivamente um `serde_json::Value` em pares chave=valor.
fn flatten_value(prefix: &str, value: &serde_json::Value) -> Vec<(String, String)> {
    match value {
        serde_json::Value::Object(map) => {
            let mut result = Vec::new();
            for (key, val) in map {
                let new_prefix = if prefix.is_empty() {
                    key.clone()
                } else {
                    format!("{}[{}]", prefix, key)
                };
                result.extend(flatten_value(&new_prefix, val));
            }
            result
        }
        serde_json::Value::Array(arr) => {
            let mut result = Vec::new();
            for (i, val) in arr.iter().enumerate() {
                let new_prefix = format!("{}[{}]", prefix, i);
                result.extend(flatten_value(&new_prefix, val));
            }
            result
        }
        serde_json::Value::String(s) => {
            vec![(prefix.to_string(), s.clone())]
        }
        serde_json::Value::Number(n) => {
            vec![(prefix.to_string(), n.to_string())]
        }
        serde_json::Value::Bool(b) => {
            vec![(prefix.to_string(), if *b { "1" } else { "0" }.to_string())]
        }
        serde_json::Value::Null => Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;

    #[derive(Debug, Serialize)]
    struct TestFilter {
        name: Option<String>,
        active: Option<bool>,
    }

    #[test]
    fn test_filter_to_query_none() {
        let result = filter_to_query::<TestFilter>(None);
        assert!(result.is_empty());
    }

    #[test]
    fn test_flatten_simple() {
        let f = TestFilter {
            name: Some("test".into()),
            active: Some(true),
        };
        let result = filter_to_query(Some(&f));
        assert!(result.contains(&("name".to_string(), "test".to_string())));
        assert!(result.contains(&("active".to_string(), "1".to_string())));
    }
}
