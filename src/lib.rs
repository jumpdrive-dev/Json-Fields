pub mod errors;
pub mod field;
pub mod migration;
mod schema;
pub(crate) mod traits;

pub use serde::{Deserialize, Serialize};
pub use traits::validator::Validator;
pub use typetag::serde as validator_impl;

#[cfg(test)]
mod tests {
    use crate::field::string_field::StringField;
    use crate::field::Field;
    use crate::schema::Schema;
    use crate::Validator;
    use serde_json::json;

    #[test]
    fn main() {
        let field = Field::String(StringField::default());
        dbg!(&field);

        let schema: Schema = field.into();
        dbg!(&schema);

        let json = json!("Hello world");
        let validation_result = schema.validate(&json);

        assert!(validation_result.is_ok());
    }
}
