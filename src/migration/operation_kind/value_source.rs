use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;
use crate::migration::json_path::{JsonPath, JsonPathError};

#[derive(Debug, Error)]
pub enum ValueSourceError {
    #[error("{0}")]
    PathError(#[from] JsonPathError),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ValueSource {
    Path(JsonPath),
    Array(Vec<ValueSource>),
    Object(HashMap<String, ValueSource>),
}

impl ValueSource {
    pub fn resolve(&self, target: &Value) -> Result<Value, ValueSourceError> {
        match self {
            ValueSource::Path(path) => {
                let value = path.resolve(target)?;
                Ok(value.clone())
            }
            ValueSource::Array(sources) => {
                let values: Result<Vec<Value>, ValueSourceError> = sources
                    .iter()
                    .map(|source| source.resolve(target))
                    .collect();

                Ok(Value::Array(values?))
            }
            ValueSource::Object(map) => {
                let mut entries = Vec::with_capacity(map.len());

                for (key, source) in map.iter() {
                    entries.push((key.to_string(), source.resolve(target)?));
                }

                Ok(Value::from_iter(entries))
            }
        }
    }
}
