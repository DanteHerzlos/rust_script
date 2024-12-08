#[cfg(test)]
mod ast_tests {
    use std::rc::Rc;

    use crate::{
        ast::{Identifier, LetStatement, Node, Program, Statement},
        token::{Token, TokenType},
    };

    #[test]
    fn to_string() {
        let statements = Vec::from([Rc::new(LetStatement {
            token: Token {
                token_type: TokenType::LET,
                literal: String::from("let"),
            },
            name: Identifier {
                token: Token {
                    token_type: TokenType::IDENT,
                    literal: String::from("myVar"),
                },
                value: String::from("myVar"),
            },
            value: Some(Rc::new(Identifier {
                token: Token {
                    token_type: TokenType::IDENT,
                    literal: String::from("anotherVar"),
                },
                value: String::from("anotherVar"),
            })),
        }) as Rc<dyn Statement>]);

        let program = Program { statements };

        assert_eq!(
            program.to_string(),
            String::from("let myVar = anotherVar;"),
            "program.to_string() wrong. got={}",
            program.to_string()
        )
    }
}
