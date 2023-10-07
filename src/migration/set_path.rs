use std::mem;
use serde_json::Value;
use thiserror::Error;
use crate::migration::json_path::{JsonPath, JsonPathError};
use crate::migration::resolve_path::{PathResolveError, resolve_path_iter, resolve_path_iter_mut, resolve_path_mut};

#[derive(Debug, Error)]
#[cfg_attr(test, derive(PartialEq))]
pub enum SetPathError {
    #[error("Failed to resolve path")]
    PathError(#[from] JsonPathError),

    #[error("Cannot set the index of an array that does not exist. Use push '$.<' instead")]
    CannotReplaceMissingIndex,

    #[error("Empty path")]
    EmptyPath,

    #[error("The provided value at is an array, so expected an index, but found '{0}'")]
    NotAnIndex(String),

    #[error("Expected either an object or an array, got a null value")]
    CannotSetOnANullValue,

    #[error("Expected either an object or an array, got a boolean")]
    CannotSetOnABoolean,

    #[error("Expected either an object or an array, got a number")]
    CannotSetOnANumber,

    #[error("Expected either an object or an array, got a string")]
    CannotSetOnAString,
}

pub trait SetPath {
    fn set_path(&mut self, path: &JsonPath, value: Value) -> Result<(), SetPathError>;
}

impl SetPath for Value {
    fn set_path(&mut self, path: &JsonPath, value: Value) -> Result<(), SetPathError> {
        // let mut parts: Vec<&str> = path.split('.').collect();

        let last = path.clone_last();
        let parent = path.parent();

        let Some(parent) = parent else {
            let _ = mem::replace(self, value);
            return Ok(());
        };

        let resolved = parent.resolve_mut(self)?;
        let last = last.expect("Should never be None here");

        match resolved {
            Value::Null => return Err(SetPathError::CannotSetOnANullValue),
            Value::Bool(_) => return Err(SetPathError::CannotSetOnABoolean),
            Value::Number(_) => return Err(SetPathError::CannotSetOnANumber),
            Value::String(_) => return Err(SetPathError::CannotSetOnAString),
            Value::Array(array) => {
                if last.starts_with('<') {
                    array.push(value);
                    return Ok(());
                }

                if last.starts_with('>') {
                    let mut new_vec = Vec::new();
                    new_vec.push(value);

                    new_vec.extend(array.iter().map(|v| v.clone()));
                    let _ = mem::replace(array, new_vec);

                    return Ok(());
                }

                let index: usize = last
                    .parse()
                    .map_err(|_| SetPathError::NotAnIndex(last.to_string()))?;

                if array.get(index).is_none() {
                    return Err(SetPathError::CannotReplaceMissingIndex);
                }

                array[index] = value;
            }
            Value::Object(map) => {
                map.insert(last.to_string(), value);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use serde_json::json;
    use crate::migration::json_path::JsonPath;
    use crate::migration::set_path::{SetPath, SetPathError};

    #[test]
    fn root_can_be_replaced_by_value() {
        let mut target = json!("String");

        let result = target.set_path(&JsonPath::from([]), json!(10));

        assert_eq!(result, Ok(()));
        assert_eq!(target, json!(10));
    }

    #[test]
    fn value_is_set_on_object_root_correctly() {
        let mut target = json!({});

        let result = target.set_path(&JsonPath::from(["a"]), json!(10));

        assert_eq!(result, Ok(()));
        assert_eq!(target, json!({ "a": 10 }));
    }

    #[test]
    fn value_is_set_on_nested_object_correctly() {
        let mut target = json!({
            "a": {}
        });

        let result = target.set_path(&JsonPath::from(["a", "b"]), json!(10));

        assert_eq!(result, Ok(()));
        assert_eq!(target, json!({ "a": { "b": 10 } }));
    }

    #[test]
    fn value_is_replaced_on_array_root_correctly() {
        let mut target = json!([
            5,
        ]);

        let result = target.set_path(&JsonPath::from(["0"]), json!(10));

        assert_eq!(result, Ok(()));
        assert_eq!(target, json!([10]));
    }

    #[test]
    fn value_is_replaced_on_nested_array_correctly() {
        let mut target = json!([
            [
                2,
                5
            ],
        ]);

        let result = target.set_path(&JsonPath::from(["0", "1"]), json!(10));

        assert_eq!(result, Ok(()));
        assert_eq!(target, json!([[2, 10]]));
    }

    #[test]
    fn cannot_replace_value_on_array_that_does_not_exist() {
        let mut target = json!([
            5,
        ]);

        let result = target.set_path(&JsonPath::from(["1"]), json!(10));

        assert_eq!(result, Err(SetPathError::CannotReplaceMissingIndex));
        assert_eq!(target, json!([5]));
    }

    #[test]
    fn value_is_pushed_to_back_onto_array_correctly() {
        let mut target = json!([5]);

        let result = target.set_path(&JsonPath::from(["<"]), json!(10));

        assert_eq!(result, Ok(()));
        assert_eq!(target, json!([5, 10]));
    }

    #[test]
    fn value_is_pushed_to_front_onto_array_correctly() {
        let mut target = json!([5]);

        let result = target.set_path(&JsonPath::from([">"]), json!(10));

        assert_eq!(result, Ok(()));
        assert_eq!(target, json!([10, 5]));
    }
}
