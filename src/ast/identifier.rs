use crate::token::Token;

use super::*;

pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Node for Identifier {
    fn get_type(&self) -> NodeType {
        return NodeType::Identifier
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
    fn try_into_identifier(&self) -> Result<&Identifier, Error> {
        Ok(self)
    }
}

impl Expression for Identifier {
    fn exporession_node(&mut self) {}
}
