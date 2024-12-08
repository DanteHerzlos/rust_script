use std::{cell::RefCell, rc::Rc};

use crate::ast::{BlockStatement, Identifier, Node};

use super::*;

pub struct Function {
    pub parameters: Rc<Vec<Rc<Identifier>>>,
    pub body: Rc<BlockStatement>,
    pub env: Rc<RefCell<Environment>>,
}

impl Object for Function {
    fn inspect(&self) -> String {
        let mut params = Vec::new();

        for p in self.parameters.as_ref() {
            params.push(p.to_string());
        }

        format!(
            "fn({}) {{\n {} \n}}",
            params.join(", "),
            self.body.as_ref().to_string()
        )
    }
    fn get_type(&self) -> ObjectType {
        ObjectType::FUNCTION
    }
    fn as_object(&self) -> &dyn Object {
        self
    }
    fn try_into_function(&self) -> Result<&Function, ErrorType> {
        Ok(self)
    }
}
