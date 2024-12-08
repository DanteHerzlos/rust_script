use std::rc::Rc;

use crate::token::Token;

use super::*;

pub struct FunctionLiteral {
    pub token: Token,
    pub parameters: Rc<Vec<Rc<Identifier>>>,
    pub body: Rc<BlockStatement>,
}

impl Node for FunctionLiteral {
    fn get_type(&self) -> NodeType {
        return NodeType::FunctionLiteral;
    }
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn to_string(&self) -> String {
        let mut params = Vec::new();

        for p in self.parameters.as_ref() {
            params.push(p.to_string());
        }

        format!(
            "{} ( {} ) {}",
            self.token_literal(),
            params.join(", "),
            self.body.to_string()
        )
    }
    fn as_node(&self) -> Rc<&dyn Node> {
        Rc::new(self)
    }
    fn try_into_fn_literal(&self) -> Result<&FunctionLiteral, Error> {
        Ok(self)
    }
}

impl Expression for FunctionLiteral {
    fn exporession_node(&mut self) {}
}
