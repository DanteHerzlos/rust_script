use std::rc::Rc;

use crate::token::Token;

use super::*;

pub struct BlockStatement {
    pub token: Token,
    pub statements: Vec<Rc<dyn Statement>>,
}

impl Node for BlockStatement {
    fn get_type(&self) -> NodeType {
        return NodeType::BlockStatement;
    }
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn to_string(&self) -> String {
        let mut out = String::new();
        for stmt in &self.statements {
            out.push_str(&stmt.to_string());
        }
        out
    }
    fn as_node(&self) -> Rc<&dyn Node> {
        Rc::new(self)
    }
    fn try_into_block_stmt(&self) -> Result<&BlockStatement, Error> {
        Ok(self)
    }
}

impl Statement for BlockStatement {
    fn statement_node(&mut self) {}
}
