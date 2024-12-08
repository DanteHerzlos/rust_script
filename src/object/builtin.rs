use super::*;

type BuiltinFunction = fn(objects: Vec<Rc<dyn Object>>) -> Rc<dyn Object>;

pub struct Builtin {
    pub function: BuiltinFunction,
}

impl Object for Builtin {
    fn inspect(&self) -> String {
        "[builtin function]".to_string()
    }
    fn get_type(&self) -> ObjectType {
        ObjectType::BUILTIN
    }
    fn as_object(&self) -> &dyn Object {
        self
    }
    fn try_into_builtin(&self) -> Result<&Builtin, ErrorType> {
        Ok(self)
    }
}
