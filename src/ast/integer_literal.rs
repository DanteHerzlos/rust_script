use crate::token::Token;

use super::*;

pub struct IntegerLiteral {
    pub token: Token,
    pub value: i64,
}

impl Node for IntegerLiteral {
    fn get_type(&self) -> NodeType {
        return NodeType::IntegerLiteral
    }
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn to_string(&self) -> String {
        self.value.to_string()
    }
    fn as_node(&self) -> Rc<&dyn Node> {
        Rc::new(self)
    }
    fn try_into_int_literal(&self) -> Result<&IntegerLiteral, Error> {
        Ok(self)
    }
}

impl Expression for IntegerLiteral {
    fn exporession_node(&mut self) {}
}
