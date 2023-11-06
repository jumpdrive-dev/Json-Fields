use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "$", rename_all = "camelCase")]
pub enum MigrationOp {

}
