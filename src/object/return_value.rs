use super::*;

pub struct ReturnValue {
    pub value: Rc<dyn Object>,
}

impl Object for ReturnValue {
    fn inspect(&self) -> String {
        self.value.inspect()
    }
    fn get_type(&self) -> ObjectType {
        ObjectType::RETURN
    }
    fn try_into_return_value(&self) -> Result<&ReturnValue, ErrorType> {
        Ok(self)
    }
    fn as_object(&self) -> &dyn Object {
        self
    }
}
