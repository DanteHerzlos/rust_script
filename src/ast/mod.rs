mod program;

mod block_statement;
mod expression_statement;
mod let_statement;
mod return_statement;

mod array_literal;
mod boolean;
mod call_expression;
mod function_literal;
mod hash_literal;
mod identifier;
mod if_expression;
mod index_expression;
mod infix_expression;
mod integer_literal;
mod prefix_expression;
mod string_literal;

use core::fmt;
use std::{hash::Hash, rc::Rc};

pub use program::Program;

pub use block_statement::BlockStatement;
pub use call_expression::CallExpression;
pub use expression_statement::ExpressionStetement;
pub use function_literal::FunctionLiteral;
pub use let_statement::LetStatement;
pub use return_statement::ReturnStatement;

pub use array_literal::ArrayLiteral;
pub use boolean::Boolean;
pub use hash_literal::HashLiteral;
pub use identifier::Identifier;
pub use if_expression::IfExpression;
pub use index_expression::IndexExpression;
pub use infix_expression::InfixExpression;
pub use integer_literal::IntegerLiteral;
pub use prefix_expression::PrefixExpression;
pub use string_literal::StringLiteral;

type Error = String;

#[derive(PartialEq, Debug, Eq, Clone)]
pub enum NodeType {
    ExpressionStetement,
    IndexExpression,
    PrefixExpression,
    InfixExpression,
    FunctionLiteral,
    ReturnStatement,
    IntegerLiteral,
    BlockStatement,
    CallExpression,
    StringLiteral,
    ArrayLiteral,
    IfExpression,
    LetStatement,
    HashLiteral,
    Identifier,
    Boolean,
    Program,
}

impl NodeType {
    pub fn as_str(&self) -> &'static str {
        match self {
            NodeType::ExpressionStetement => "ExpressionStetement",
            NodeType::PrefixExpression => "PrefixExpression",
            NodeType::InfixExpression => "InfixExpression",
            NodeType::FunctionLiteral => "FunctionLiteral",
            NodeType::IndexExpression => "IndexExpression",
            NodeType::ReturnStatement => "ReturnStatement",
            NodeType::IntegerLiteral => "IntegerLiteral",
            NodeType::BlockStatement => "BlockStatement",
            NodeType::CallExpression => "CallExpression",
            NodeType::StringLiteral => "StringLiteral",
            NodeType::ArrayLiteral => "ArrayLiteral",
            NodeType::IfExpression => "IfExpression",
            NodeType::LetStatement => "LetStatement",
            NodeType::HashLiteral => "HashLiteral",
            NodeType::Identifier => "Identifier",
            NodeType::Boolean => "Boolean",
            NodeType::Program => "Program",
        }
    }
}

impl fmt::Display for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

pub trait Node {
    fn get_type(&self) -> NodeType;
    fn token_literal(&self) -> String;
    fn to_string(&self) -> String;
    fn as_node(&self) -> Rc<&dyn Node>;
    fn try_into_if_expr(&self) -> Result<&IfExpression, Error> {
        Err(format!(
            "can't cast from {} to IfExpression",
            self.get_type()
        ))
    }
    fn try_into_prefix_expr(&self) -> Result<&PrefixExpression, Error> {
        Err(format!(
            "can't cast from {} to PrefixExpression",
            self.get_type()
        ))
    }
    fn try_into_infix_expr(&self) -> Result<&InfixExpression, Error> {
        Err(format!(
            "can't cast from {} to InfixExpression",
            self.get_type()
        ))
    }
    fn try_into_expr_stmt(&self) -> Result<&ExpressionStetement, Error> {
        Err(format!(
            "can't cast from {} to ExpressionStetement",
            self.get_type()
        ))
    }
    fn try_into_int_literal(&self) -> Result<&IntegerLiteral, Error> {
        Err(format!(
            "can't cast from {} to IntegerLiteral",
            self.get_type()
        ))
    }
    fn try_into_program(&self) -> Result<&Program, Error> {
        Err(format!("can't cast from {} to Program", self.get_type()))
    }
    fn try_into_boolean(&self) -> Result<&Boolean, Error> {
        Err(format!("can't cast from {} to Boolean", self.get_type()))
    }
    fn try_into_return_stmt(&self) -> Result<&ReturnStatement, Error> {
        Err(format!(
            "can't cast from {} to ReturnStatement",
            self.get_type()
        ))
    }
    fn try_into_let_statement(&self) -> Result<&LetStatement, Error> {
        Err(format!(
            "can't cast from {} to LetStatement",
            self.get_type()
        ))
    }
    fn try_into_identifier(&self) -> Result<&Identifier, Error> {
        Err(format!("can't cast from {} to Identifier", self.get_type()))
    }
    fn try_into_fn_literal(&self) -> Result<&FunctionLiteral, Error> {
        Err(format!(
            "can't cast from {} to FunctionLiteral",
            self.get_type()
        ))
    }
    fn try_into_call_expr(&self) -> Result<&CallExpression, Error> {
        Err(format!(
            "can't cast from {} to CallExpression",
            self.get_type()
        ))
    }
    fn try_into_block_stmt(&self) -> Result<&BlockStatement, Error> {
        Err(format!(
            "can't cast from {} to BlockStatement",
            self.get_type()
        ))
    }
    fn try_into_str_literal(&self) -> Result<&StringLiteral, Error> {
        Err(format!(
            "can't cast from {} to StringLiteral",
            self.get_type()
        ))
    }
    fn try_into_array_literal(&self) -> Result<&ArrayLiteral, Error> {
        Err(format!(
            "can't cast from {} to ArrayLiteral",
            self.get_type()
        ))
    }
    fn try_into_index_expr(&self) -> Result<&IndexExpression, Error> {
        Err(format!(
            "can't cast from {} to IndexExpression",
            self.get_type()
        ))
    }
    fn try_into_hash_literal(&self) -> Result<&HashLiteral, Error> {
        Err(format!(
            "can't cast from {} to HashLiteral",
            self.get_type()
        ))
    }
}

pub trait Statement: Node {
    fn statement_node(&mut self) {}
}

pub trait Expression: Node {
    fn exporession_node(&mut self) {}
}

impl Hash for dyn Expression {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let ptr = self as *const dyn Expression as *const ();
        ptr.hash(state)
    }
}

impl PartialEq for dyn Expression {
    fn eq(&self, other: &Self) -> bool {
        let ptr1 = self as *const dyn Expression as *const ();
        let ptr2 = other as *const dyn Expression as *const ();
        ptr1 == ptr2
    }
}

impl Eq for dyn Expression {}
