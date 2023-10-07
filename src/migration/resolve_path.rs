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

pub fn resolve_path(path: impl Into<String>, value: &Value) -> Result<&Value, PathResolveError> {
    let path = path.into();
    resolve_path_iter(path.split('.'), value)
}

pub fn resolve_path_iter<'a, 'b>(mut parts: impl Iterator<Item = &'b str>, value: &'a Value) -> Result<&'a Value, PathResolveError> {
    if !matches!(parts.next(), Some("$")) {
        return Err(PathResolveError::NoRoot);
    }

    let mut current = value;

    while let Some(part) = parts.next() {
        match current {
            Value::Null => return Err(PathResolveError::CannotMatchOnANullValue),
            Value::Bool(_) => return Err(PathResolveError::CannotMatchOnABoolean),
            Value::Number(_) => return Err(PathResolveError::CannotMatchOnANumber),
            Value::String(_) => return Err(PathResolveError::CannotMatchOnAString),
            Value::Array(list) => {
                if part.starts_with('<') {
                    let n_back: usize = part.replace('<', "")
                        .parse()
                        .unwrap_or(1);

                    current = list.iter()
                        .nth_back(n_back - 1)
                        .ok_or(PathResolveError::NoLastItem)?;

                    continue;
                }

                if part.starts_with('>') {
                    let n_front: usize = part.replace('>', "")
                        .parse()
                        .unwrap_or(1);

                    current = list.iter()
                        .nth(n_front - 1)
                        .ok_or(PathResolveError::NoFirstItem)?;

                    continue;
                }

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

pub fn resolve_path_mut(path: impl Into<String>, value: &mut Value) -> Result<&mut Value, PathResolveError> {
    let path = path.into();
    resolve_path_iter_mut(path.split('.'), value)
}

pub fn resolve_path_iter_mut<'a, 'b>(mut parts: impl Iterator<Item = &'b str>, value: &'a mut Value) -> Result<&'a mut Value, PathResolveError> {
    if !matches!(parts.next(), Some("$")) {
        return Err(PathResolveError::NoRoot);
    }

    let mut current = value;

    while let Some(part) = parts.next() {
        match current {
            Value::Null => return Err(PathResolveError::CannotMatchOnANullValue),
            Value::Bool(_) => return Err(PathResolveError::CannotMatchOnABoolean),
            Value::Number(_) => return Err(PathResolveError::CannotMatchOnANumber),
            Value::String(_) => return Err(PathResolveError::CannotMatchOnAString),
            Value::Array(list) => {
                if part.starts_with('<') {
                    let n_back: usize = part.replace('<', "")
                        .parse()
                        .unwrap_or(1);

                    current = list.iter_mut()
                        .nth_back(n_back - 1)
                        .ok_or(PathResolveError::NoLastItem)?;

                    continue;
                }

                if part.starts_with('>') {
                    let n_front: usize = part.replace('>', "")
                        .parse()
                        .unwrap_or(1);

                    current = list.iter_mut()
                        .nth(n_front - 1)
                        .ok_or(PathResolveError::NoFirstItem)?;

                    continue;
                }

                let index: usize = part.parse()
                    .map_err(|_| PathResolveError::NotAnIndex(part.to_string()))?;

                let Some(value) = list.get_mut(index) else {
                    return Err(PathResolveError::IndexNotFound(index));
                };

                current = value;
            }
            Value::Object(map) => {
                let Some(value) = map.get_mut(part) else {
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

    #[test]
    fn last_item_of_array_is_returned_correctly() {
        let value = json!([ 1, 2, 3 ]);

        let resolved_value = resolve_path("$.<", &value);

        assert_eq!(resolved_value, Ok(&json!(3)));
    }

    #[test]
    fn missing_last_index_of_array_returns_error() {
        let value = json!([]);

        let resolved_value = resolve_path("$.<", &value);

        assert_eq!(resolved_value, Err(PathResolveError::NoLastItem));
    }

    #[test]
    fn specific_item_back_of_array_is_returned_correctly() {
        let value = json!([ 1, 2, 3 ]);

        let resolved_value = resolve_path("$.<3", &value);

        assert_eq!(resolved_value, Ok(&json!(1)));
    }

    #[test]
    fn first_item_of_array_is_returned_correctly() {
        let value = json!([ 1, 2, 3 ]);

        let resolved_value = resolve_path("$.>", &value);

        assert_eq!(resolved_value, Ok(&json!(1)));
    }

    #[test]
    fn specific_item_front_of_array_is_returned_correctly() {
        let value = json!([ 1, 2, 3 ]);

        let resolved_value = resolve_path("$.>3", &value);

        assert_eq!(resolved_value, Ok(&json!(3)));
    }
}
