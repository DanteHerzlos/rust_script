use crate::token::Token;

use super::*;

pub struct ArrayLiteral {
    pub token: Token,
    pub elements: Vec<Rc<dyn Expression>>,
}

impl Node for ArrayLiteral {
    fn get_type(&self) -> NodeType {
        return NodeType::ArrayLiteral;
    }
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn to_string(&self) -> String {
        let mut elements = vec![];

        for el in &self.elements {
            elements.push(el.to_string());
        }

        format!("[{}]", elements.join(", "))
    }
    fn as_node(&self) -> Rc<&dyn Node> {
        Rc::new(self)
    }
    fn try_into_array_literal(&self) -> Result<&ArrayLiteral, Error> {
        Ok(self)
    }
}

impl Expression for ArrayLiteral {
    fn exporession_node(&mut self) {}
}
