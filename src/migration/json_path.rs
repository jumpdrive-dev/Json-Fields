use std::fmt::{Display, Formatter};
use std::str::FromStr;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{Error, Visitor};
use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum JsonPathError {
    #[error("Failed to parse JSON path")]
    FailedToParse,

    #[error("Expected key '{0}', but none was found")]
    KeyNotFound(String),

    #[error("The provided value at is an array, so expected an index, but found '{0}'")]
    NotAnIndex(String),

    #[error("Expected index '{0}', but none was found")]
    IndexNotFound(usize),

    #[error("Expected either an object or an array, got a null value")]
    CannotMatchOnANullValue,

    #[error("Expected either an object or an array, got a boolean")]
    CannotMatchOnABoolean,

    #[error("Expected either an object or an array, got a number")]
    CannotMatchOnANumber,

    #[error("Expected either an object or an array, got a string")]
    CannotMatchOnAString,

    #[error("The target array is empty and does not contain a first item")]
    NoFirstItem,

    #[error("The target array is empty and does not contain a last item")]
    NoLastItem,
}

#[derive(Debug, PartialEq, Clone)]
pub struct JsonPath {
    parts: Vec<String>,
}

impl JsonPath {
    pub fn new() -> Self {
        Self {
            parts: Vec::new(),
        }
    }

    pub fn push(&mut self, part: impl Into<String>) {
        self.parts.push(part.into());
    }

    pub fn pop(&mut self) {
        self.parts.pop();
    }

    pub fn clone_last(&self) -> Option<String> {
        self.parts.last().map(|a| a.to_string())
    }

    pub fn is_root(&self) -> bool {
        self.parts.is_empty()
    }

    pub fn parent(&self) -> Option<Self> {
        if self.is_root() {
            return None;
        }

        let mut clone = self.clone();
        clone.pop();

        Some(clone)
    }

    pub fn resolve<'s, 'a>(&'s self, value: &'a Value) -> Result<&'a Value, JsonPathError> {
        let mut current = value;
        let mut iterator = self.parts.iter();

        while let Some(part) = iterator.next() {
            match current {
                Value::Null => return Err(JsonPathError::CannotMatchOnANullValue),
                Value::Bool(_) => return Err(JsonPathError::CannotMatchOnABoolean),
                Value::Number(_) => return Err(JsonPathError::CannotMatchOnANumber),
                Value::String(_) => return Err(JsonPathError::CannotMatchOnAString),
                Value::Array(list) => {
                    if part.starts_with('<') {
                        let n_back: usize = part.replace('<', "")
                            .parse()
                            .unwrap_or(1);

                        current = list.iter()
                            .nth_back(n_back - 1)
                            .ok_or(JsonPathError::NoLastItem)?;

                        continue;
                    }

                    if part.starts_with('>') {
                        let n_front: usize = part.replace('>', "")
                            .parse()
                            .unwrap_or(1);

                        current = list.iter()
                            .nth(n_front - 1)
                            .ok_or(JsonPathError::NoFirstItem)?;

                        continue;
                    }

                    let index: usize = part.parse()
                        .map_err(|_| JsonPathError::NotAnIndex(part.to_string()))?;

                    let Some(value) = list.get(index) else {
                        return Err(JsonPathError::IndexNotFound(index));
                    };

                    current = value;
                }
                Value::Object(map) => {
                    let Some(value) = map.get(part) else {
                        return Err(JsonPathError::KeyNotFound(part.to_string()));
                    };

                    current = value;
                }
            }
        }

        Ok(current)
    }

    pub fn resolve_mut<'s, 'a>(&'s self, value: &'a mut Value) -> Result<&'a mut Value, JsonPathError> {
        let mut current = value;
        let mut iterator = self.parts.iter();

        while let Some(part) = iterator.next() {
            match current {
                Value::Null => return Err(JsonPathError::CannotMatchOnANullValue),
                Value::Bool(_) => return Err(JsonPathError::CannotMatchOnABoolean),
                Value::Number(_) => return Err(JsonPathError::CannotMatchOnANumber),
                Value::String(_) => return Err(JsonPathError::CannotMatchOnAString),
                Value::Array(list) => {
                    if part.starts_with('<') {
                        let n_back: usize = part.replace('<', "")
                            .parse()
                            .unwrap_or(1);

                        current = list.iter_mut()
                            .nth_back(n_back - 1)
                            .ok_or(JsonPathError::NoLastItem)?;

                        continue;
                    }

                    if part.starts_with('>') {
                        let n_front: usize = part.replace('>', "")
                            .parse()
                            .unwrap_or(1);

                        current = list.iter_mut()
                            .nth(n_front - 1)
                            .ok_or(JsonPathError::NoFirstItem)?;

                        continue;
                    }

                    let index: usize = part.parse()
                        .map_err(|_| JsonPathError::NotAnIndex(part.to_string()))?;

                    let Some(value) = list.get_mut(index) else {
                        return Err(JsonPathError::IndexNotFound(index));
                    };

                    current = value;
                }
                Value::Object(map) => {
                    let Some(value) = map.get_mut(part) else {
                        return Err(JsonPathError::KeyNotFound(part.to_string()));
                    };

                    current = value;
                }
            }
        }

        Ok(current)
    }
}

impl FromIterator<String> for JsonPath {
    fn from_iter<T: IntoIterator<Item=String>>(iter: T) -> Self {
        Self {
            parts: iter.into_iter().collect()
        }
    }
}

impl<const N: usize> From<[&str; N]> for JsonPath {
    fn from(value: [&str; N]) -> Self {
        Self {
            parts: value.iter()
                .map(|value| value.to_string())
                .collect()
        }
    }
}

impl Serialize for JsonPath {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_str(&self.to_string())
    }
}

struct JsonPathVisitor;

impl<'de> Visitor<'de> for JsonPathVisitor {
    type Value = JsonPath;

    fn expecting(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "a json path")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: Error {
        JsonPath::from_str(v)
            .map_err(|err| E::custom(err.to_string()))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where E: Error {
        JsonPath::from_str(&v)
            .map_err(|err| E::custom(err.to_string()))
    }
}

impl<'de> Deserialize<'de> for JsonPath {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        deserializer.deserialize_string(JsonPathVisitor)
    }
}

impl Display for JsonPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "$")?;

        for part in &self.parts {
            write!(f, ".{}", part)?;
        }

        Ok(())
    }
}

impl FromStr for JsonPath {
    type Err = JsonPathError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('.');

        if !matches!(parts.next(), Some("$")) {
            return Err(JsonPathError::FailedToParse);
        }

        Ok(Self {
            parts: parts
                .map(|str| str.to_string())
                .collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use serde_json::json;
    use crate::migration::json_path::{JsonPath, JsonPathError};

    #[test]
    fn path_can_be_parsed_from_str() {
        assert_eq!(JsonPath::from_str("$"), Ok(JsonPath { parts: vec![] }));
        assert_eq!(JsonPath::from_str("$.a"), Ok(JsonPath { parts: vec!["a".to_string()] }));
        assert_eq!(JsonPath::from_str("$.b.0"), Ok(JsonPath { parts: vec!["b".to_string(), "0".to_string()] }));
    }

    #[test]
    fn path_requires_a_root() {
        assert_eq!(JsonPath::from_str(""), Err(JsonPathError::FailedToParse));
        assert_eq!(JsonPath::from_str("a"), Err(JsonPathError::FailedToParse));
        assert_eq!(JsonPath::from_str(".b.0"), Err(JsonPathError::FailedToParse));
    }

    #[test]
    fn single_path_of_object_is_resolved_correctly() {
        let value = json!({
            "a": 10,
        });

        let resolved_value = JsonPath::from_str("$.a")
            .unwrap()
            .resolve(&value);

        assert_eq!(resolved_value, Ok(&json!(10)));
    }

    #[test]
    fn deep_nested_object_path_is_resolved_correctly() {
        let value = json!({
            "a": {
                "b": {
                    "c": 10
                }
            },
        });

        let resolved_value = JsonPath::from_str("$.a.b.c")
            .unwrap()
            .resolve(&value);

        assert_eq!(resolved_value, Ok(&json!(10)));
    }

    #[test]
    fn invalid_key_returns_an_err() {
        let value = json!({
            "a": 10,
        });

        let resolved_value = JsonPath::from_str("$.b")
            .unwrap()
            .resolve(&value);

        assert_eq!(resolved_value, Err(JsonPathError::KeyNotFound("b".to_string())));
    }

    #[test]
    fn single_path_of_array_is_resolved_correctly() {
        let value = json!([
            10
        ]);

        let resolved_value = JsonPath::from_str("$.0")
            .unwrap()
            .resolve(&value);

        assert_eq!(resolved_value, Ok(&json!(10)));
    }

    #[test]
    fn invalid_index_returns_an_err() {
        let value = json!([
            10
        ]);

        let resolved_value = JsonPath::from_str("$.abc")
            .unwrap()
            .resolve(&value);

        assert_eq!(resolved_value, Err(JsonPathError::NotAnIndex("abc".to_string())));
    }

    #[test]
    fn missing_index_returns_an_err() {
        let value = json!([
            10
        ]);

        let resolved_value = JsonPath::from_str("$.1")
            .unwrap()
            .resolve(&value);

        assert_eq!(resolved_value, Err(JsonPathError::IndexNotFound(1)));
    }

    #[test]
    fn last_item_of_array_is_returned_correctly() {
        let value = json!([ 1, 2, 3 ]);

        let resolved_value = JsonPath::from_str("$.<")
            .unwrap()
            .resolve(&value);

        assert_eq!(resolved_value, Ok(&json!(3)));
    }

    #[test]
    fn missing_last_index_of_array_returns_error() {
        let value = json!([]);

        let resolved_value = JsonPath::from_str("$.<")
            .unwrap()
            .resolve(&value);

        assert_eq!(resolved_value, Err(JsonPathError::NoLastItem));
    }

    #[test]
    fn specific_item_back_of_array_is_returned_correctly() {
        let value = json!([ 1, 2, 3 ]);

        let resolved_value = JsonPath::from_str("$.<3")
            .unwrap()
            .resolve(&value);

        assert_eq!(resolved_value, Ok(&json!(1)));
    }

    #[test]
    fn first_item_of_array_is_returned_correctly() {
        let value = json!([ 1, 2, 3 ]);

        let resolved_value = JsonPath::from_str("$.>")
            .unwrap()
            .resolve(&value);

        assert_eq!(resolved_value, Ok(&json!(1)));
    }

    #[test]
    fn specific_item_front_of_array_is_returned_correctly() {
        let value = json!([ 1, 2, 3 ]);

        let resolved_value = JsonPath::from_str("$.>3")
            .unwrap()
            .resolve(&value);

        assert_eq!(resolved_value, Ok(&json!(3)));
    }

    #[test]
    fn json_path_can_be_serialized_deserialized() {
        let serialized = serde_json::to_string(&JsonPath::from_str("$.a.0").unwrap());

        assert!(serialized.is_ok());

        let serialized = serialized.unwrap();
        assert_eq!(serialized, "\"$.a.0\"");

        let deserialized = serde_json::from_str::<JsonPath>(&serialized);

        assert!(deserialized.is_ok());
        assert_eq!(deserialized.unwrap(), JsonPath::from_str("$.a.0").unwrap());
    }
}
