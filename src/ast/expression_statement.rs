use std::rc::Rc;

use crate::token::Token;

use super::*;

pub struct ExpressionStetement {
    pub token: Token,
    pub expression: Option<Rc<dyn Expression>>,
}

impl Node for ExpressionStetement {
    fn get_type(&self) -> NodeType {
        return NodeType::ExpressionStetement
    }
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn to_string(&self) -> String {
        if self.expression.is_none() {
            return String::new();
        }

        self.expression.as_ref().unwrap().to_string()
    }
    fn as_node(&self) -> Rc<&dyn Node> {
        Rc::new(self)
    }
    fn try_into_expr_stmt(&self) -> Result<&ExpressionStetement, Error> {
        Ok(self)
    }
}

impl Statement for ExpressionStetement {
    fn statement_node(&mut self) {}
}
