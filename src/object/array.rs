use super::*;

pub struct Array {
    pub elements: Vec<Rc<dyn Object>>
}

impl Object for Array {
    fn inspect(&self) -> String {
        let mut elements = vec![];

        for el in &self.elements {
            elements.push(el.inspect())
        }

        format!("[{}]", elements.join(", "))
    }
    fn get_type(&self) -> ObjectType {
        ObjectType::ARRAY
    }
    fn as_object(&self) -> &dyn Object {
        self
    }
    fn try_into_array(&self) -> Result<&Array, ErrorType> {
        Ok(self)
    }
}

