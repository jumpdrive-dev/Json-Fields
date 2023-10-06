use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Error)]
#[cfg_attr(test, derive(PartialEq))]
pub enum PathResolveError {
    #[error("A JSON path needs to have a root: $")]
    NoRoot,

    #[error("Expected key '{0}', but none was found")]
    KeyNotFound(String),

    #[error("The provided value at is an array, so expected an index, but found '{0}'")]
    NotAnIndex(String),

    #[error("Expected index '{0}', but none was found")]
    IndexNotFound(usize),

    #[error("Expected either an object or an array, got a null value")]
    CannotMathOnANullValue,

    #[error("Expected either an object or an array, got a boolean")]
    CannotMathOnABoolean,

    #[error("Expected either an object or an array, got a number")]
    CannotMathOnANumber,

    #[error("Expected either an object or an array, got a string")]
    CannotMathOnAString,
}

pub fn resolve_path(path: impl Into<String>, value: &Value) -> Result<&Value, PathResolveError> {
    let path = path.into();
    let mut parts = path.split('.');

    if !matches!(parts.next(), Some("$")) {
        return Err(PathResolveError::NoRoot);
    }

    let mut current = value;

    while let Some(part) = parts.next() {
        match current {
            Value::Null => return Err(PathResolveError::CannotMathOnANullValue),
            Value::Bool(_) => return Err(PathResolveError::CannotMathOnABoolean),
            Value::Number(_) => return Err(PathResolveError::CannotMathOnANumber),
            Value::String(_) => return Err(PathResolveError::CannotMathOnAString),
            Value::Array(list) => {
                let index: usize = part.parse()
                    .map_err(|_| PathResolveError::NotAnIndex(part.to_string()))?;

                let Some(value) = list.get(index) else {
                    return Err(PathResolveError::IndexNotFound(index));
                };

                current = value;
            }
            Value::Object(map) => {
                let Some(value) = map.get(part) else {
                    return Err(PathResolveError::KeyNotFound(part.to_string()));
                };

                current = value;
            }
        }
    }

    Ok(current)
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use crate::migration::resolve_path::{PathResolveError, resolve_path};

    #[test]
    fn root_needs_to_be_set() {
        // Fail
        assert!(resolve_path("", &json!(null)).is_err());
        assert!(resolve_path("dqed", &json!(null)).is_err());
        assert!(resolve_path(".", &json!(null)).is_err());

        // Succeed
        assert_eq!(resolve_path("$", &json!(null)), Ok(&json!(null)));
    }

    #[test]
    fn single_path_of_object_is_resolved_correctly() {
        let value = json!({
            "a": 10,
        });

        let resolved_value = resolve_path("$.a", &value);

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

        let resolved_value = resolve_path("$.a.b.c", &value);

        assert_eq!(resolved_value, Ok(&json!(10)));
    }

    #[test]
    fn invalid_key_returns_an_err() {
        let value = json!({
            "a": 10,
        });

        let resolved_value = resolve_path("$.b", &value);

        assert_eq!(resolved_value, Err(PathResolveError::KeyNotFound("b".to_string())));
    }

    #[test]
    fn single_path_of_array_is_resolved_correctly() {
        let value = json!([
            10
        ]);

        let resolved_value = resolve_path("$.0", &value);

        assert_eq!(resolved_value, Ok(&json!(10)));
    }

    #[test]
    fn invalid_index_returns_an_err() {
        let value = json!([
            10
        ]);

        let resolved_value = resolve_path("$.abc", &value);

        assert_eq!(resolved_value, Err(PathResolveError::NotAnIndex("abc".to_string())));
    }

    #[test]
    fn missing_index_returns_an_err() {
        let value = json!([
            10
        ]);

        let resolved_value = resolve_path("$.1", &value);

        assert_eq!(resolved_value, Err(PathResolveError::IndexNotFound(1)));
    }
}
