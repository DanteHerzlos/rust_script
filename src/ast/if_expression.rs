use std::rc::Rc;

use crate::token::Token;

use super::*;

pub struct IfExpression {
    pub token: Token,
    pub condition: Option<Rc<dyn Expression>>,
    pub consequence: Option<BlockStatement>,
    pub alternative: Option<BlockStatement>,
}

impl Node for IfExpression {
    fn get_type(&self) -> NodeType {
        return NodeType::IfExpression;
    }
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn to_string(&self) -> String {
        if self.alternative.is_some() {
            return format!("else {}", self.alternative.as_ref().unwrap().to_string(),);
        }

        format!(
            "if {} {}",
            self.condition.as_ref().unwrap().to_string(),
            self.consequence.as_ref().unwrap().to_string()
        )
    }
    fn as_node(&self) -> Rc<&dyn Node> {
        Rc::new(self)
    }
    fn try_into_if_expr(&self) -> Result<&IfExpression, Error> {
        Ok(self)
    }
}

impl Expression for IfExpression {
    fn exporession_node(&mut self) {}
}
