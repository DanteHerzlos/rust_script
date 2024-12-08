use std::rc::Rc;

use crate::token::Token;

use super::*;

pub struct PrefixExpression {
    pub token: Token,
    pub operator: String,
    pub right: Rc<dyn Expression>,
}

impl Node for PrefixExpression {
    fn get_type(&self) -> NodeType {
        return NodeType::PrefixExpression
    }
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn to_string(&self) -> String {
        format!("({}{})", self.operator, self.right.to_string())
    }
    fn as_node(&self) -> Rc<&dyn Node> {
        Rc::new(self)
    }
    fn try_into_prefix_expr(&self) -> Result<&PrefixExpression, Error> {
        Ok(self)
    }
}

impl Expression for PrefixExpression {
    fn exporession_node(&mut self) {}
}
