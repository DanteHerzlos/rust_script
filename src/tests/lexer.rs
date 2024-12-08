#[cfg(test)]
mod lexer_tests {
    use crate::{lexer::Lexer, token::{Token, TokenType}};


    fn new_token(token_type: TokenType, literal: &str) -> Token {
        Token {
            token_type,
            literal: String::from(literal),
        }
    }

    #[test]
    fn test_next_token() {
        let input = String::from(
            "
            let five = 5;
            let ten = 10;
            let add = fn(x, y) {
                x + y;
            };
            let result = add(five, ten);
            !-/*5;
            5 < 10 > 5;

            if (5 < 10) {
                return true;
            } else {
                return false;
            }

            10 == 10;
            10 != 9;
            \"foobar\"
            \"foo bar\"
            [1, 2];
            { \"foo\": \"bar\" }
            ",
        );

        let tests: Vec<Token> = Vec::from([
            // let five = 5;
            new_token(TokenType::LET, "let"),
            new_token(TokenType::IDENT, "five"),
            new_token(TokenType::ASSIGN, "="),
            new_token(TokenType::INT, "5"),
            new_token(TokenType::SEMICOLON, ";"),
            // let ten = 10;
            new_token(TokenType::LET, "let"),
            new_token(TokenType::IDENT, "ten"),
            new_token(TokenType::ASSIGN, "="),
            new_token(TokenType::INT, "10"),
            new_token(TokenType::SEMICOLON, ";"),
            // let add = fn(x, y) {
            new_token(TokenType::LET, "let"),
            new_token(TokenType::IDENT, "add"),
            new_token(TokenType::ASSIGN, "="),
            new_token(TokenType::FUNCTION, "fn"),
            new_token(TokenType::LPAREN, "("),
            new_token(TokenType::IDENT, "x"),
            new_token(TokenType::COMMA, ","),
            new_token(TokenType::IDENT, "y"),
            new_token(TokenType::RPAREN, ")"),
            new_token(TokenType::LBRACE, "{"),
            // x + y;
            new_token(TokenType::IDENT, "x"),
            new_token(TokenType::PLUS, "+"),
            new_token(TokenType::IDENT, "y"),
            new_token(TokenType::SEMICOLON, ";"),
            // };
            new_token(TokenType::RBRACE, "}"),
            new_token(TokenType::SEMICOLON, ";"),
            // let result = add(five, ten);
            new_token(TokenType::LET, "let"),
            new_token(TokenType::IDENT, "result"),
            new_token(TokenType::ASSIGN, "="),
            new_token(TokenType::IDENT, "add"),
            new_token(TokenType::LPAREN, "("),
            new_token(TokenType::IDENT, "five"),
            new_token(TokenType::COMMA, ","),
            new_token(TokenType::IDENT, "ten"),
            new_token(TokenType::RPAREN, ")"),
            new_token(TokenType::SEMICOLON, ";"),
            // !-/*5;
            new_token(TokenType::BANG, "!"),
            new_token(TokenType::MINUS, "-"),
            new_token(TokenType::SLASH, "/"),
            new_token(TokenType::ASTERISK, "*"),
            new_token(TokenType::INT, "5"),
            new_token(TokenType::SEMICOLON, ";"),
            // 5 < 10 > 5;
            new_token(TokenType::INT, "5"),
            new_token(TokenType::LT, "<"),
            new_token(TokenType::INT, "10"),
            new_token(TokenType::GT, ">"),
            new_token(TokenType::INT, "5"),
            new_token(TokenType::SEMICOLON, ";"),
            // if (5 < 10) {
            new_token(TokenType::IF, "if"),
            new_token(TokenType::LPAREN, "("),
            new_token(TokenType::INT, "5"),
            new_token(TokenType::LT, "<"),
            new_token(TokenType::INT, "10"),
            new_token(TokenType::RPAREN, ")"),
            new_token(TokenType::LBRACE, "{"),
            // return true;
            new_token(TokenType::RETURN, "return"),
            new_token(TokenType::TRUE, "true"),
            new_token(TokenType::SEMICOLON, ";"),
            // } else {
            new_token(TokenType::RBRACE, "}"),
            new_token(TokenType::ELSE, "else"),
            new_token(TokenType::LBRACE, "{"),
            // return false;
            new_token(TokenType::RETURN, "return"),
            new_token(TokenType::FALSE, "false"),
            new_token(TokenType::SEMICOLON, ";"),
            // }
            new_token(TokenType::RBRACE, "}"),
            // 10 == 10;
            new_token(TokenType::INT, "10"),
            new_token(TokenType::EQ, "=="),
            new_token(TokenType::INT, "10"),
            new_token(TokenType::SEMICOLON, ";"),
            // 10 != 9;
            new_token(TokenType::INT, "10"),
            new_token(TokenType::NotEQ, "!="),
            new_token(TokenType::INT, "9"),
            new_token(TokenType::SEMICOLON, ";"),
            // \"foobar\"
            new_token(TokenType::STRING, "foobar"),
            // \"foo bar\"
            new_token(TokenType::STRING, "foo bar"),
            // [1, 2];
            new_token(TokenType::LBRACKET, "["),
            new_token(TokenType::INT, "1"),
            new_token(TokenType::COMMA, ","),
            new_token(TokenType::INT, "2"),
            new_token(TokenType::RBRACKET, "]"),
            new_token(TokenType::SEMICOLON, ";"),
            // { \"foo\": \"bar\" }
            new_token(TokenType::LBRACE, "{"),
            new_token(TokenType::STRING, "foo"),
            new_token(TokenType::COLON, ":"),
            new_token(TokenType::STRING, "bar"),
            new_token(TokenType::RBRACE, "}"),
            // EOF
            new_token(TokenType::EOF, "\0"),
        ]);

        let mut lexer = Lexer::new(input);

        for (index, test_token) in tests.iter().enumerate() {
            let token = lexer.next_token();

            assert_eq!(
                token.token_type,
                test_token.token_type,
                "Failed test #{}: Should same token_type",
                index + 1
            );
            assert_eq!(
                token.literal,
                test_token.literal,
                "Failed test #{}: Should same literal",
                index + 1
            );
        }
    }
}

