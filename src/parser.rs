use std::{collections::HashMap, rc::Rc, sync::LazyLock};

use crate::{
    ast::{
        ArrayLiteral, BlockStatement, Boolean, CallExpression, Expression, ExpressionStetement,
        FunctionLiteral, HashLiteral, Identifier, IfExpression, IndexExpression, InfixExpression,
        IntegerLiteral, LetStatement, PrefixExpression, Program, ReturnStatement, Statement,
        StringLiteral,
    },
    lexer::Lexer,
    token::{Token, TokenType},
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Precedence {
    LOWEST,
    EQUALS,
    LESSGREATER,
    SUM,
    PRODUCT,
    PREFIX,
    CALL,
    INDEX,
}

type PrefixParseFn = fn(&mut Parser) -> Option<Rc<dyn Expression>>;
type InfixParseFn = fn(&mut Parser, Option<Rc<dyn Expression>>) -> Option<Rc<dyn Expression>>;

pub struct Parser {
    lexer: Lexer,

    errors: Vec<String>,

    cur_token: Token,
    peek_token: Token,

    prefix_parse_fns: HashMap<String, PrefixParseFn>,
    infix_parse_fns: HashMap<String, InfixParseFn>,

    precedences: LazyLock<HashMap<TokenType, Precedence>>,
}

const PRECEDENCES: LazyLock<HashMap<TokenType, Precedence>> = LazyLock::new(|| {
    HashMap::from([
        (TokenType::EQ, Precedence::EQUALS),
        (TokenType::NotEQ, Precedence::EQUALS),
        (TokenType::LT, Precedence::LESSGREATER),
        (TokenType::GT, Precedence::LESSGREATER),
        (TokenType::PLUS, Precedence::SUM),
        (TokenType::MINUS, Precedence::SUM),
        (TokenType::SLASH, Precedence::PRODUCT),
        (TokenType::ASTERISK, Precedence::PRODUCT),
        (TokenType::LPAREN, Precedence::CALL),
        (TokenType::LBRACKET, Precedence::INDEX),
    ])
});

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let cur_token = lexer.next_token();
        let peek_token = lexer.next_token();


        let mut parser = Parser {
            lexer,
            cur_token,
            peek_token,
            errors: Vec::new(),
            prefix_parse_fns: HashMap::new(),
            infix_parse_fns: HashMap::new(),
            precedences: PRECEDENCES,
        };

        parser.register_prefix(TokenType::IDENT, Parser::parse_identifier);
        parser.register_prefix(TokenType::INT, Parser::parse_int_literal);
        parser.register_prefix(TokenType::STRING, Parser::parse_str_literal);
        parser.register_prefix(TokenType::BANG, Parser::parse_prefix_expression);
        parser.register_prefix(TokenType::MINUS, Parser::parse_prefix_expression);
        parser.register_prefix(TokenType::TRUE, Parser::parse_boolean);
        parser.register_prefix(TokenType::FALSE, Parser::parse_boolean);
        parser.register_prefix(TokenType::LPAREN, Parser::parse_grouped_expression);
        parser.register_prefix(TokenType::IF, Parser::parse_if_expression);
        parser.register_prefix(TokenType::FUNCTION, Parser::parse_function_literal);
        parser.register_prefix(TokenType::LBRACKET, Parser::parse_array_literal);
        parser.register_prefix(TokenType::LBRACE, Parser::parse_hash_literal);

        parser.register_infix(TokenType::LBRACKET, Parser::parse_index_expression);
        parser.register_infix(TokenType::PLUS, Parser::parse_infix_expression);
        parser.register_infix(TokenType::MINUS, Parser::parse_infix_expression);
        parser.register_infix(TokenType::SLASH, Parser::parse_infix_expression);
        parser.register_infix(TokenType::ASTERISK, Parser::parse_infix_expression);
        parser.register_infix(TokenType::EQ, Parser::parse_infix_expression);
        parser.register_infix(TokenType::NotEQ, Parser::parse_infix_expression);
        parser.register_infix(TokenType::LT, Parser::parse_infix_expression);
        parser.register_infix(TokenType::GT, Parser::parse_infix_expression);
        parser.register_infix(TokenType::LPAREN, Parser::parse_call_expression);

        parser
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program {
            statements: Vec::new(),
        };
        while self.cur_token.token_type != TokenType::EOF {
            let stmt = self.parse_statement();
            if stmt.is_some() {
                program.statements.push(stmt.unwrap());
            }
            self.next_token();
        }

        program
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn parse_statement(&mut self) -> Option<Rc<dyn Statement>> {
        match self.cur_token.token_type {
            TokenType::LET => {
                let stmt = self.parse_let_statement();
                if stmt.is_some() {
                    return Some(Rc::new(stmt.unwrap()));
                }
                return None;
            }
            TokenType::RETURN => {
                let stmt = self.parse_return_statement();
                if stmt.is_some() {
                    return Some(Rc::new(stmt.unwrap()));
                }
                return None;
            }
            _ => Some(Rc::new(self.parse_expression_statement())),
        }
    }

    fn parse_grouped_expression(&mut self) -> Option<Rc<dyn Expression>> {
        self.next_token();

        let expr = self.parse_expression(Precedence::LOWEST);

        if !self.expect_peek(TokenType::RPAREN) {
            return None;
        }

        expr
    }

    fn parse_call_expression(
        &mut self,
        function: Option<Rc<dyn Expression>>,
    ) -> Option<Rc<dyn Expression>> {
        let token = self.cur_token.clone();
        let arguments = self.parse_expression_list(TokenType::RPAREN).unwrap();
        Some(Rc::new(CallExpression {
            token,
            arguments,
            function: function?,
        }))
    }

    fn parse_expression_list(&mut self, end: TokenType) -> Option<Vec<Rc<dyn Expression>>> {
        let mut list = Vec::new();

        if self.peek_token_is(end.clone()) {
            self.next_token();
            return Some(list);
        }

        self.next_token();

        list.push(self.parse_expression(Precedence::LOWEST).unwrap());

        while self.peek_token_is(TokenType::COMMA) {
            self.next_token();
            self.next_token();
            list.push(self.parse_expression(Precedence::LOWEST).unwrap());
        }

        if !self.expect_peek(end) {
            return None;
        }

        Some(list)
    }

    fn parse_function_literal(&mut self) -> Option<Rc<dyn Expression>> {
        let token = self.cur_token.clone();

        if !self.expect_peek(TokenType::LPAREN) {
            return None;
        }

        let parameters = Rc::new(self.parse_function_parameters().unwrap());

        if !self.expect_peek(TokenType::LBRACE) {
            return None;
        }

        let body = Rc::new(self.parse_block_statement());

        Some(Rc::new(FunctionLiteral {
            token,
            parameters,
            body,
        }))
    }

    fn parse_function_parameters(&mut self) -> Option<Vec<Rc<Identifier>>> {
        let mut identifiers = Vec::new();

        if self.peek_token_is(TokenType::RPAREN) {
            self.next_token();
            return Some(identifiers);
        }

        self.next_token();

        let ident = Rc::new(Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        });

        identifiers.push(ident.into());

        while self.peek_token_is(TokenType::COMMA) {
            self.next_token();
            self.next_token();

            let ident = Rc::new(Identifier {
                token: self.cur_token.clone(),
                value: self.cur_token.literal.clone(),
            });

            identifiers.push(ident.into());
        }

        if !self.expect_peek(TokenType::RPAREN) {
            return None;
        }

        Some(identifiers)
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Option<Rc<dyn Expression>> {
        let prefix = self
            .prefix_parse_fns
            .get(self.cur_token.token_type.as_str());

        if prefix.is_none() {
            self.no_prefix_parse_fn_error(self.cur_token.token_type.clone());
            return None;
        }

        let mut left_expr = prefix.unwrap()(self);

        while !self.peek_token_is(TokenType::SEMICOLON)
            && (precedence as u8) < (self.peek_precedence() as u8)
        {
            let token_type = self.peek_token.token_type.as_str();
            if self.infix_parse_fns.get(token_type).is_none() {
                return left_expr;
            }

            self.next_token();

            left_expr = self.infix_parse_fns.get(token_type).unwrap()(self, left_expr);
        }

        left_expr
    }

    fn parse_if_expression(&mut self) -> Option<Rc<dyn Expression>> {
        let token = self.cur_token.clone();

        if !self.expect_peek(TokenType::LPAREN) {
            return None;
        }

        self.next_token();

        let condition = self.parse_expression(Precedence::LOWEST);

        if !self.expect_peek(TokenType::RPAREN) {
            return None;
        }

        if !self.expect_peek(TokenType::LBRACE) {
            return None;
        }

        let consequence = Some(self.parse_block_statement());
        let mut alternative = None;

        if self.peek_token_is(TokenType::ELSE) {
            self.next_token();

            if !self.expect_peek(TokenType::LBRACE) {
                return None;
            }

            alternative = Some(self.parse_block_statement());
        }

        Some(Rc::new(IfExpression {
            token,
            consequence,
            condition,
            alternative,
        }))
    }

    fn parse_block_statement(&mut self) -> BlockStatement {
        let token = self.cur_token.clone();
        let mut statements: Vec<Rc<dyn Statement>> = Vec::new();

        self.next_token();

        while !self.cur_token_is(TokenType::RBRACE) && !self.cur_token_is(TokenType::EOF) {
            let stmt = self.parse_statement();
            if stmt.is_some() {
                statements.push(stmt.unwrap());
            }
            self.next_token();
        }

        BlockStatement { token, statements }
    }

    fn parse_prefix_expression(&mut self) -> Option<Rc<dyn Expression>> {
        let token = self.cur_token.clone();
        let operator = self.cur_token.literal.clone();

        self.next_token();

        let right = self.parse_expression(Precedence::PREFIX).unwrap();

        Some(Rc::new(PrefixExpression {
            token,
            operator,
            right,
        }))
    }

    fn parse_infix_expression(
        &mut self,
        left: Option<Rc<dyn Expression>>,
    ) -> Option<Rc<dyn Expression>> {
        let token = self.cur_token.clone();
        let operator = self.cur_token.literal.clone();

        let precedence = self.cur_precedence();

        self.next_token();

        let right = self.parse_expression(precedence);

        Some(Rc::new(InfixExpression {
            token,
            operator,
            left,
            right,
        }))
    }

    fn parse_boolean(&mut self) -> Option<Rc<dyn Expression>> {
        Some(Rc::new(Boolean {
            token: self.cur_token.clone(),
            value: self.cur_token_is(TokenType::TRUE),
        }))
    }

    fn parse_identifier(&mut self) -> Option<Rc<dyn Expression>> {
        Some(Rc::new(Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        }))
    }

    fn parse_int_literal(&mut self) -> Option<Rc<dyn Expression>> {
        let value = match self.cur_token.literal.parse::<i64>() {
            Ok(val) => val,
            Err(e) => {
                self.errors.push(e.to_string());
                return None;
            }
        };

        Some(Rc::new(IntegerLiteral {
            token: self.cur_token.clone(),
            value,
        }))
    }

    fn parse_str_literal(&mut self) -> Option<Rc<dyn Expression>> {
        Some(Rc::new(StringLiteral {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        }))
    }

    fn parse_let_statement(&mut self) -> Option<LetStatement> {
        let token = self.cur_token.clone();

        if !self.expect_peek(TokenType::IDENT) {
            return None;
        }

        let name = Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.clone().literal,
        };

        if !self.expect_peek(TokenType::ASSIGN) {
            return None;
        }

        self.next_token();

        let value = self.parse_expression(Precedence::LOWEST);

        if !self.cur_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }

        Some(LetStatement { token, value, name })
    }

    fn parse_expression_statement(&mut self) -> ExpressionStetement {
        let stmt = ExpressionStetement {
            token: self.cur_token.clone(),
            expression: self.parse_expression(Precedence::LOWEST),
        };

        if self.peek_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }

        return stmt;
    }

    fn parse_return_statement(&mut self) -> Option<ReturnStatement> {
        let token = self.cur_token.clone();
        self.next_token();

        let return_value = self.parse_expression(Precedence::LOWEST);

        if self.peek_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }

        Some(ReturnStatement {
            token,
            return_value,
        })
    }

    fn cur_token_is(&mut self, token_type: TokenType) -> bool {
        self.cur_token.token_type == token_type
    }

    fn peek_token_is(&mut self, token_type: TokenType) -> bool {
        self.peek_token.token_type == token_type
    }

    fn expect_peek(&mut self, token_type: TokenType) -> bool {
        if self.peek_token_is(token_type.clone()) {
            self.next_token();
            return true;
        }

        self.peek_errors(token_type);
        false
    }

    pub fn get_errors(&self) -> Vec<String> {
        self.errors.clone()
    }

    pub fn peek_errors(&mut self, token: TokenType) {
        let msg = format!(
            "expected next token to be {}, got {} instead",
            token, self.peek_token.token_type
        );
        self.errors.push(msg);
    }

    pub fn register_prefix(&mut self, token_type: TokenType, prefix_parse_fn: PrefixParseFn) {
        self.prefix_parse_fns
            .insert(token_type.to_string(), prefix_parse_fn);
    }

    pub fn register_infix(&mut self, token_type: TokenType, infix_parse_fn: InfixParseFn) {
        self.infix_parse_fns
            .insert(token_type.to_string(), infix_parse_fn);
    }

    fn no_prefix_parse_fn_error(&mut self, token_type: TokenType) {
        let msg = format!("no prefix parse function for {} found", token_type);
        self.errors.push(msg);
    }

    fn peek_precedence(&self) -> Precedence {
        match self.precedences.get(&self.peek_token.token_type) {
            None => Precedence::LOWEST,
            Some(val) => val.clone(),
        }
    }

    fn cur_precedence(&self) -> Precedence {
        match self.precedences.get(&self.cur_token.token_type) {
            None => Precedence::LOWEST,
            Some(val) => val.clone(),
        }
    }

    fn parse_array_literal(&mut self) -> Option<Rc<dyn Expression>> {
        Some(Rc::new(ArrayLiteral {
            token: self.cur_token.clone(),
            elements: self.parse_expression_list(TokenType::RBRACKET).unwrap(),
        }))
    }

    fn parse_index_expression(
        &mut self,
        left: Option<Rc<dyn Expression>>,
    ) -> Option<Rc<dyn Expression>> {
        let token = self.cur_token.clone();

        self.next_token();

        let index = self.parse_expression(Precedence::LOWEST).unwrap();

        if !self.expect_peek(TokenType::RBRACKET) {
            return None;
        }

        Some(Rc::new(IndexExpression {
            token,
            index,
            left: left?,
        }))
    }

    fn parse_hash_literal(&mut self) -> Option<Rc<dyn Expression>> {
        let token = self.cur_token.clone();
        let mut pairs: HashMap<Rc<dyn Expression>, Rc<dyn Expression>> = HashMap::new();

        while !self.peek_token_is(TokenType::RBRACE) {
            self.next_token();
            let key = self.parse_expression(Precedence::LOWEST);

            if !self.expect_peek(TokenType::COLON) {
                return None;
            }

            self.next_token();

            let value = self.parse_expression(Precedence::LOWEST);

            pairs.insert(key.unwrap(), value.unwrap());

            if !self.peek_token_is(TokenType::RBRACE) && !self.expect_peek(TokenType::COMMA) {
                return None;
            }
        }

        if !self.expect_peek(TokenType::RBRACE) {
            return None;
        }

        Some(Rc::new(HashLiteral { token, pairs }))
    }
}
