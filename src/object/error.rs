use super::*;

pub struct Error {
    pub message: String,
}

impl Object for Error {
    fn inspect(&self) -> String {
        return format!("ERROR: {}", self.message);
    }
    fn get_type(&self) -> ObjectType {
        ObjectType::ERROR
    }
    fn as_object(&self) -> &dyn Object {
        self
    }
    fn try_into_error(&self) -> Result<&Error, ErrorType> {
        Ok(self)
    }
}
