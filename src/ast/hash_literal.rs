use std::collections::HashMap;

use crate::token::Token;

use super::*;

pub struct HashLiteral {
    pub token: Token,
    pub pairs: HashMap<Rc<dyn Expression>, Rc<dyn Expression>>,
}

impl Node for HashLiteral {
    fn get_type(&self) -> NodeType {
        return NodeType::HashLiteral;
    }
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn to_string(&self) -> String {
        let mut pairs = vec![];

        for (key, value) in &self.pairs {
            pairs.push(format!("{}:{}", key.to_string(), value.to_string()));
        }

        format!("{{{}}}", pairs.join(", "))
    }
    fn as_node(&self) -> Rc<&dyn Node> {
        Rc::new(self)
    }
    fn try_into_hash_literal(&self) -> Result<&HashLiteral, Error> {
        Ok(self)
    }
}

impl Expression for HashLiteral {
    fn exporession_node(&mut self) {}
}
