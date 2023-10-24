pub mod advanced_string_type;

use serde::{Deserialize, Serialize};
use crate::schema_type::advanced_type::advanced_string_type::AdvancedStringType;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "$", rename_all = "camelCase")]
#[cfg_attr(test, derive(PartialEq))]
pub enum AdvancedType {
    String(AdvancedStringType),
}
