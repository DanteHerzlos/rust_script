use crate::token::Token;

use super::*;

pub struct IndexExpression {
    pub token: Token,
    pub left: Rc<dyn Expression>,
    pub index: Rc<dyn Expression>,
}

impl Node for IndexExpression {
    fn get_type(&self) -> NodeType {
        return NodeType::IndexExpression;
    }
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn to_string(&self) -> String {
        format!("({}[{}])", self.left.to_string(), self.index.to_string())
    }
    fn as_node(&self) -> Rc<&dyn Node> {
        Rc::new(self)
    }
    fn try_into_index_expr(&self) -> Result<&IndexExpression, Error> {
        Ok(self)
    }
}

impl Expression for IndexExpression {
    fn exporession_node(&mut self) {}
}
