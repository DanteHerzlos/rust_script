use crate::token::Token;

use super::*;

pub struct StringLiteral {
    pub token: Token,
    pub value: String,
}

impl Node for StringLiteral {
    fn get_type(&self) -> NodeType {
        return NodeType::StringLiteral
    }
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn to_string(&self) -> String {
        self.value.clone()
    }
    fn as_node(&self) -> Rc<&dyn Node> {
        Rc::new(self)
    }
    fn try_into_str_literal(&self) -> Result<&StringLiteral, Error> {
        Ok(self)
    }
}

impl Expression for StringLiteral {
    fn exporession_node(&mut self) {}
}

