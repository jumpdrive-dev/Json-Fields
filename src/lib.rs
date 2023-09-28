pub mod field;
mod schema;
pub(crate) mod traits;
pub mod errors;

pub use traits::validator::Validator;

#[cfg(test)]
mod tests {
    use serde_json::json;
    use crate::field::Field;
    use crate::field::string_field::StringField;
    use crate::schema::Schema;

    #[test]
    fn main() {
        let field = Field::String(StringField::default());
        dbg!(&field);

        let schema: Schema = field.into();
        dbg!(&schema);

        let json = json!("Hello world");


        // dbg!(&field);
        //
        // let schema: Schema = field.into();
        // dbg!(&schema);
    }
}
