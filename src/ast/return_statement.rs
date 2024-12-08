use std::rc::Rc;

use crate::token::Token;

use super::*;

pub struct ReturnStatement {
    pub token: Token,
    pub return_value: Option<Rc<dyn Expression>>,
}

impl Node for ReturnStatement {
    fn get_type(&self) -> NodeType {
        return NodeType::ReturnStatement;
    }
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn to_string(&self) -> String {
        if self.return_value.is_some() {
            let return_value = self.return_value.as_ref().unwrap().to_string();
            return format!("{} {};", self.token_literal(), return_value);
        } else {
            return format!("{} ;", self.token_literal());
        }
    }
    fn as_node(&self) -> Rc<&dyn Node> {
        Rc::new(self)
    }
    fn try_into_return_stmt(&self) -> Result<&ReturnStatement, Error> {
        Ok(self)
    }
}

impl Statement for ReturnStatement {
    fn statement_node(&mut self) {}
}
