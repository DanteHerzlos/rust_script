use std::rc::Rc;

use crate::token::Token;

use super::*;

pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Option<Rc<dyn Expression>>,
}

impl Node for LetStatement {
    fn get_type(&self) -> NodeType {
        return NodeType::LetStatement
    }
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn to_string(&self) -> String {
        if self.value.is_some() {
            return format!(
                "{} {} = {};",
                self.token_literal(),
                self.name.to_string(),
                self.value.as_ref().unwrap().to_string()
            );
        } else {
            return format!("{} {} = ;", self.token_literal(), self.name.to_string(),);
        }
    }
    fn as_node(&self) -> Rc<&dyn Node> {
        Rc::new(self)
    }
    fn try_into_let_statement(&self) -> Result<&LetStatement, Error> {
        Ok(self)
    }
}

impl Statement for LetStatement {
    fn statement_node(&mut self) {}
}
