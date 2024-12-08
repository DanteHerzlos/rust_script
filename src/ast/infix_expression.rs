use std::rc::Rc;

use crate::token::Token;

use super::*;

pub struct InfixExpression {
    pub token: Token,
    pub right: Option<Rc<dyn Expression>>,
    pub operator: String,
    pub left: Option<Rc<dyn Expression>>,
}

impl Node for InfixExpression {
    fn get_type(&self) -> NodeType {
        return NodeType::InfixExpression
    }
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn to_string(&self) -> String {
        format!(
            "({} {} {})",
            self.left.as_ref().unwrap().to_string(),
            self.operator,
            self.right.as_ref().unwrap().to_string()
        )
    }
    fn as_node(&self) -> Rc<&dyn Node> {
        Rc::new(self)
    }
    fn try_into_infix_expr(&self) -> Result<&InfixExpression, Error> {
        Ok(self)
    }
}

impl Expression for InfixExpression {
    fn exporession_node(&mut self) {}
}
