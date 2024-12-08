use crate::token::{lookup_ident, Token, TokenType};

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        l.read_char();
        return l;
    }

    fn read_string(&mut self) -> String {
        let position = self.position + 1;
        self.read_char();
        while self.ch != '\"' && self.ch != '\0' {
            self.read_char();
        }
        return self.input[position..self.position].to_string();
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while is_digit(self.ch) {
            self.read_char();
        }
        self.input[position..self.position].to_string()
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while is_letter(self.ch) {
            self.read_char();
        }
        self.input[position..self.position].to_string()
    }

    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }

    fn peek_char(&mut self) -> char {
        if self.read_position >= self.input.len() {
            '\0'
        } else {
            self.input.chars().nth(self.read_position).unwrap()
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.ch {
            '+' => new_char_token(TokenType::PLUS, self.ch),
            '-' => new_char_token(TokenType::MINUS, self.ch),
            '/' => new_char_token(TokenType::SLASH, self.ch),
            '*' => new_char_token(TokenType::ASTERISK, self.ch),
            '<' => new_char_token(TokenType::LT, self.ch),
            '>' => new_char_token(TokenType::GT, self.ch),
            ';' => new_char_token(TokenType::SEMICOLON, self.ch),
            ',' => new_char_token(TokenType::COMMA, self.ch),
            ':' => new_char_token(TokenType::COLON, self.ch),
            '(' => new_char_token(TokenType::LPAREN, self.ch),
            ')' => new_char_token(TokenType::RPAREN, self.ch),
            '{' => new_char_token(TokenType::LBRACE, self.ch),
            '}' => new_char_token(TokenType::RBRACE, self.ch),
            '[' => new_char_token(TokenType::LBRACKET, self.ch),
            ']' => new_char_token(TokenType::RBRACKET, self.ch),
            '\0' => new_char_token(TokenType::EOF, self.ch),
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token {
                        token_type: TokenType::EQ,
                        literal: "==".to_string(),
                    }
                } else {
                    new_char_token(TokenType::ASSIGN, self.ch)
                }
            }
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token {
                        token_type: TokenType::NotEQ,
                        literal: "!=".to_string(),
                    }
                } else {
                    new_char_token(TokenType::BANG, self.ch)
                }
            }
            '\"' => Token {
                token_type: TokenType::STRING,
                literal: self.read_string(),
            },
            _ => {
                if is_letter(self.ch) {
                    let literal = self.read_identifier();
                    let token_type = lookup_ident(&literal);
                    return Token {
                        token_type,
                        literal,
                    };
                } else if is_digit(self.ch) {
                    return Token {
                        token_type: TokenType::INT,
                        literal: self.read_number(),
                    };
                } else {
                    new_char_token(TokenType::ILLEGAL, self.ch)
                }
            }
        };

        self.read_char();

        token
    }
}

fn new_char_token(token_type: TokenType, ch: char) -> Token {
    Token {
        token_type,
        literal: ch.to_string(),
    }
}

fn is_letter(ch: char) -> bool {
    'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
}

fn is_digit(ch: char) -> bool {
    '0' <= ch && ch <= '9'
}
