use std::rc::Rc;

use super::*;

pub struct Program {
    pub statements: Vec<Rc<dyn Statement>>,
}

impl Node for Program {
    fn get_type(&self) -> NodeType {
        return NodeType::Program;
    }
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            self.statements[0].token_literal()
        } else {
            String::from("")
        }
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
    fn try_into_program(&self) -> Result<&Program, Error> {
        Ok(self)
    }
}
