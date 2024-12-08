use crate::token::Token;

use super::*;

pub struct Boolean {
    pub token: Token,
    pub value: bool,
}

impl Node for Boolean {
    fn get_type(&self) -> NodeType {
        return NodeType::Boolean;
    }
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn to_string(&self) -> String {
        self.token.literal.clone()
    }
    fn as_node(&self) -> Rc<&dyn Node> {
        Rc::new(self)
    }
    fn try_into_boolean(&self) -> Result<&Boolean, Error> {
        Ok(self)
    }
}

impl Expression for Boolean {
    fn exporession_node(&mut self) {}
}
