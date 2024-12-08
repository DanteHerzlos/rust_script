use super::*;

pub struct NULL {}

impl Object for NULL {
    fn inspect(&self) -> String {
        "null".to_string()
    }
    fn get_type(&self) -> ObjectType {
        ObjectType::NULL
    }
    fn as_int(&self) -> &Integer {
        &Integer { value: 0 }
    }
    fn as_bool(&self) -> &Boolean {
        &Boolean { value: false }
    }
    fn as_object(&self) -> &dyn Object {
        self
    }
    fn try_into_null(&self) -> Result<&NULL, ErrorType> {
        Ok(self)
    }
}
