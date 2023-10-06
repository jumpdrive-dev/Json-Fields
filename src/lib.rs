pub mod errors;
pub mod field;
mod schema;
pub(crate) mod traits;
pub mod migration;

pub use traits::validator::Validator;
pub use typetag::serde as validator_impl;
pub use serde::{Serialize, Deserialize};

#[cfg(test)]
mod tests {
    use crate::field::string_field::StringField;
    use crate::field::Field;
    use crate::schema::Schema;
    use serde_json::json;
    use crate::Validator;

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
