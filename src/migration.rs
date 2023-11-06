pub mod migration_op;

use std::collections::HashMap;
use json_search::json_path::JsonPath;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum Migration {
    Ref(JsonPath),
    Array(Vec<Migration>),
    Object(HashMap<String, Migration>),
}

#[cfg(test)]
mod tests {

}
