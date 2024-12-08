use std::rc::Rc;

use crate::token::Token;

use super::*;

pub struct CallExpression {
    pub token: Token,
    pub function: Rc<dyn Expression>,
    pub arguments: Vec<Rc<dyn Expression>>,
}

impl Node for CallExpression {
    fn get_type(&self) -> NodeType {
        return NodeType::CallExpression
    }
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn to_string(&self) -> String {
        let mut args = Vec::new();

        for a in &self.arguments {
            args.push(a.to_string());
        }

        format!("{}({})", self.function.to_string(), args.join(", "),)
    }
    fn as_node(&self) -> Rc<&dyn Node> {
        Rc::new(self)
    }
    fn try_into_call_expr(&self) -> Result<&CallExpression, Error> {
        Ok(self)
    }
}

impl Expression for CallExpression {
    fn exporession_node(&mut self) {}
}
