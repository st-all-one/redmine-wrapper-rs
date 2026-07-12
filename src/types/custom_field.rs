// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};
use crate::types::base::RedmineId;

/// Tipo de entidade que pode ter campos personalizados.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CustomizedType {
    /// Campo personalizado para issues.
    #[serde(rename = "IssueCustomField")]
    Issue,
    /// Campo personalizado para entradas de tempo.
    #[serde(rename = "TimeEntryCustomField")]
    TimeEntry,
    /// Campo personalizado para projetos.
    #[serde(rename = "ProjectCustomField")]
    Project,
    /// Campo personalizado para usuários.
    #[serde(rename = "UserCustomField")]
    User,
    /// Campo personalizado para grupos.
    #[serde(rename = "GroupCustomField")]
    Group,
    /// Campo personalizado para documentos.
    #[serde(rename = "DocumentCustomField")]
    Document,
    /// Campo personalizado para versões.
    #[serde(rename = "VersionCustomField")]
    Version,
    /// Campo personalizado para tempo gasto.
    #[serde(rename = "SpentTimeCustomField")]
    SpentTime,
}

/// Formato de um campo personalizado.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FieldFormat {
    /// Formato de texto livre (string).
    #[serde(rename = "string")]
    String,
    /// Formato de lista de seleção.
    #[serde(rename = "list")]
    List,
    /// Formato de data.
    #[serde(rename = "date")]
    Date,
    /// Formato booleano (verdadeiro/falso).
    #[serde(rename = "bool")]
    Bool,
    /// Formato de número inteiro.
    #[serde(rename = "int")]
    Int,
    /// Formato de número decimal.
    #[serde(rename = "float")]
    Float,
    /// Formato de referência a usuário.
    #[serde(rename = "user")]
    User,
    /// Formato de referência a versão.
    #[serde(rename = "version")]
    Version,
}

/// Definição de campo personalizado.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomField {
    /// Identificador único do campo personalizado.
    pub id: RedmineId,
    /// Nome do campo personalizado.
    pub name: Option<String>,
    /// Tipo de entidade à qual o campo se aplica.
    pub customized_type: Option<CustomizedType>,
    /// Formato do campo (texto, lista, data, etc.).
    pub field_format: Option<FieldFormat>,
    /// Expressão regular para validação do campo.
    pub regexp: Option<String>,
    /// Comprimento mínimo permitido para o valor.
    pub min_length: Option<u32>,
    /// Comprimento máximo permitido para o valor.
    pub max_length: Option<u32>,
    /// Indica se o campo é obrigatório.
    pub is_required: Option<bool>,
    /// Indica se o campo pode ser usado como filtro.
    pub is_filter: Option<bool>,
    /// Indica se o campo é pesquisável.
    pub searchable: Option<bool>,
    /// Indica se o campo aceita múltiplos valores.
    pub multiple: Option<bool>,
    /// Valor padrão do campo.
    pub default_value: Option<serde_json::Value>,
    /// Valores possíveis (para campos do tipo lista).
    pub possible_values: Option<Vec<super::base::IdName>>,
    /// Indica se o campo é visível.
    pub visible: Option<bool>,
    /// Trackers aos quais o campo é aplicável.
    pub trackers: Option<Vec<super::base::IdName>>,
    /// Papéis (roles) que podem ver/editar o campo.
    pub roles: Option<Vec<super::base::IdName>>,
}
