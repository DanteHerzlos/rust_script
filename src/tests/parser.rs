#[cfg(test)]
mod parser_tests {
    use std::{any::Any, collections::HashMap, rc::Rc};

    use crate::{
        ast::{Expression, LetStatement, Node, NodeType},
        lexer::Lexer,
        parser::Parser,
    };

    struct InfixTest {
        input: String,
        left_value: Box<dyn Any>,
        operator: String,
        right_value: Box<dyn Any>,
    }

    #[test]
    fn test_let_statements() {
        struct TestStruct {
            input: String,
            expected_identifier: String,
            expected_value: Box<dyn Any>,
        }

        let tests = Vec::from([
            TestStruct {
                input: "let x = 5;".to_string(),
                expected_identifier: "x".to_string(),
                expected_value: Box::new(5),
            },
            TestStruct {
                input: "let y = 10;".to_string(),
                expected_identifier: "y".to_string(),
                expected_value: Box::new(10),
            },
            TestStruct {
                input: "let foobar = y;".to_string(),
                expected_identifier: "foobar".to_string(),
                expected_value: Box::new("y"),
            },
        ]);

        for test in tests {
            let lexer = Lexer::new(test.input);
            let mut parser = Parser::new(lexer);

            let mut program = parser.parse_program();
            check_parse_errors(parser);

            let statements_len = program.statements.len();
            assert_eq!(
                statements_len, 1,
                "program.statements does not contain 1 statement. got={}",
                statements_len,
            );

            let stmt = &mut program.statements[0];
            test_let_statement(
                stmt.try_into_let_statement().unwrap(),
                test.expected_identifier.clone(),
            );

            let value = stmt
                .try_into_let_statement()
                .unwrap()
                .value
                .as_ref()
                .unwrap()
                .clone();

            test_literal_expression(value, test.expected_value);
        }
    }

    fn test_let_statement(s: &LetStatement, name: String) {
        assert_eq!(
            s.token_literal(),
            "let",
            "s.token_literal not 'let'. got={}",
            s.token_literal()
        );

        assert_eq!(
            name, s.name.value,
            "let_stmt.name.value not {}. got={}",
            name, s.name.value
        );

        assert_eq!(
            name,
            s.name.token_literal(),
            "let_stmt.name not {}. got={}",
            name,
            s.name.token_literal()
        );
    }

    fn check_parse_errors(parser: Parser) {
        let errors = parser.get_errors();

        if errors.len() == 0 {
            return;
        }

        eprintln!("parser has {} errors", errors.len());

        for msg in errors {
            eprintln!("parser error: {}", msg);
        }

        assert!(false)
    }

    #[test]
    fn test_identifier_expression() {
        let input = String::from("foobar;");

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        check_parse_errors(parser);

        let statements_len = program.statements.len();

        assert_eq!(
            statements_len, 1,
            "program has not enough statements. got={}",
            statements_len,
        );

        assert_eq!(
            NodeType::ExpressionStetement,
            program.statements[0].get_type(),
            "program.statements[0] is not ast::ExpressionStetement. got={}",
            program.statements[0].get_type()
        );

        let stmt = program.statements[0]
            .try_into_expr_stmt()
            .unwrap()
            .expression
            .as_ref()
            .unwrap();

        assert_eq!(
            NodeType::Identifier,
            stmt.get_type(),
            "statements[0].expression is not ast::Identifier. got={}",
            stmt.get_type(),
        );
        let ident = stmt.try_into_identifier().unwrap();

        assert_eq!(
            "foobar", ident.value,
            "ident.value not {}. got={}",
            "foobar", ident.value
        );

        assert_eq!(
            "foobar",
            ident.token_literal(),
            "ident.token_literal not {}. got={}",
            "foobar",
            ident.token_literal()
        );
    }

    #[test]
    fn test_integer_literal_expression() {
        let input = String::from("5;");

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        check_parse_errors(parser);

        let statements_len = program.statements.len();

        assert_eq!(
            statements_len, 1,
            "program has not enough statements. got={}",
            statements_len,
        );

        assert_eq!(
            NodeType::ExpressionStetement,
            program.statements[0].get_type(),
            "program.statements[0] is not ast::ExpressionStetement. got={}",
            program.statements[0].get_type()
        );

        let stmt = program.statements[0].try_into_expr_stmt().unwrap();
        let expr = stmt.expression.as_ref().unwrap();

        assert_eq!(
            expr.get_type(),
            NodeType::IntegerLiteral,
            "program.statements[0] is not ast::IntegerLiteral. got={}",
            expr.get_type()
        );

        let literal = expr.try_into_int_literal().unwrap();

        assert_eq!(
            5, literal.value,
            "ident.value not {}. got={}",
            5, literal.value
        );

        assert_eq!(
            "5",
            literal.token_literal(),
            "ident.token_literal not {}. got={}",
            "5",
            literal.token_literal()
        );
    }

    #[test]
    fn test_parsing_prefix_expressions() {
        struct PrefixTest {
            input: String,
            operator: String,
            value: Box<dyn Any>,
        }

        let prefix_tests = Vec::from([
            PrefixTest {
                input: String::from("!5;"),
                operator: String::from("!"),
                value: Box::new(5),
            },
            PrefixTest {
                input: String::from("-15;"),
                operator: String::from("-"),
                value: Box::new(15),
            },
            PrefixTest {
                input: String::from("!true;"),
                operator: String::from("!"),
                value: Box::new(true),
            },
            PrefixTest {
                input: String::from("!false;"),
                operator: String::from("!"),
                value: Box::new(false),
            },
        ]);

        for test in prefix_tests {
            let lexer = Lexer::new(test.input);
            let mut parser = Parser::new(lexer);
            let program = parser.parse_program();
            check_parse_errors(parser);

            let statements_len = program.statements.len();

            assert_eq!(
                statements_len, 1,
                "program has not enough statements. got={}",
                statements_len,
            );

            assert_eq!(
                NodeType::ExpressionStetement,
                program.statements[0].get_type(),
                "program.statements[0] is not ast::ExpressionStetement. got={}",
                program.statements[0].get_type()
            );

            let stmt = program.statements[0].try_into_expr_stmt().unwrap();

            let expr = stmt.expression.as_ref().unwrap();
            assert_eq!(
                NodeType::PrefixExpression,
                expr.get_type(),
                "stmt is not ast::PrefixExpression. got={}",
                expr.get_type()
            );

            let prefix_expr = expr.try_into_prefix_expr().unwrap();
            assert_eq!(
                test.operator, prefix_expr.operator,
                "expr.operator is not {}. got={}",
                test.operator, prefix_expr.operator,
            );

            test_literal_expression(prefix_expr.right.clone(), test.value);
        }
    }

    fn test_int_literal(expr: Rc<dyn Expression>, value: i64) {
        assert_eq!(
            NodeType::IntegerLiteral,
            expr.get_type(),
            "stmt is not ast::IntegerLiteral. got={}",
            expr.get_type()
        );

        let int_literal = expr.try_into_int_literal().unwrap();
        assert_eq!(
            value, int_literal.value,
            "int_literal.value not {}. got={}",
            value, int_literal.value
        );

        assert_eq!(
            value.to_string(),
            int_literal.token_literal(),
            "int_literal.token_literal not {}. got={}",
            value,
            int_literal.token_literal()
        )
    }

    #[test]
    fn test_parsing_infix_expressions() {
        let infix_tests = Vec::from([
            InfixTest {
                input: String::from("5 + 5;"),
                left_value: Box::new(5),
                operator: String::from("+"),
                right_value: Box::new(5),
            },
            InfixTest {
                input: String::from("5 - 5;"),
                left_value: Box::new(5),
                operator: String::from("-"),
                right_value: Box::new(5),
            },
            InfixTest {
                input: String::from("5 * 5;"),
                left_value: Box::new(5),
                operator: String::from("*"),
                right_value: Box::new(5),
            },
            InfixTest {
                input: String::from("5 / 5;"),
                left_value: Box::new(5),
                operator: String::from("/"),
                right_value: Box::new(5),
            },
            InfixTest {
                input: String::from("5 < 5;"),
                left_value: Box::new(5),
                operator: String::from("<"),
                right_value: Box::new(5),
            },
            InfixTest {
                input: String::from("5 > 5;"),
                left_value: Box::new(5),
                operator: String::from(">"),
                right_value: Box::new(5),
            },
            InfixTest {
                input: String::from("5 == 5;"),
                left_value: Box::new(5),
                operator: String::from("=="),
                right_value: Box::new(5),
            },
            InfixTest {
                input: String::from("5 != 5;"),
                left_value: Box::new(5),
                operator: String::from("!="),
                right_value: Box::new(5),
            },
            InfixTest {
                input: String::from("true == true;"),
                left_value: Box::new(true),
                operator: String::from("=="),
                right_value: Box::new(true),
            },
            InfixTest {
                input: String::from("true != false;"),
                left_value: Box::new(true),
                operator: String::from("!="),
                right_value: Box::new(false),
            },
            InfixTest {
                input: String::from("true == true;"),
                left_value: Box::new(true),
                operator: String::from("=="),
                right_value: Box::new(true),
            },
        ]);

        for test in infix_tests {
            let lexer = Lexer::new(test.input);
            let mut parser = Parser::new(lexer);
            let program = parser.parse_program();
            check_parse_errors(parser);

            let statements_len = program.statements.len();

            assert_eq!(
                statements_len, 1,
                "program has not enough statements. got={}",
                statements_len,
            );

            assert_eq!(
                NodeType::ExpressionStetement,
                program.statements[0].get_type(),
                "program.statements[0] is not ast::ExpressionStetement. got={}",
                program.statements[0].get_type()
            );

            let stmt = program.statements[0].try_into_expr_stmt().unwrap();

            test_infix_expression(
                stmt.expression.as_ref().unwrap().clone(),
                test.left_value,
                test.operator,
                test.right_value,
            );
        }
    }

    #[test]
    fn test_if_expression() {
        let test = InfixTest {
            input: "if (x < y) { x }".to_string(),
            left_value: Box::new("x".to_string()),
            operator: "<".to_string(),
            right_value: Box::new("y".to_string()),
        };

        let lexer = Lexer::new(test.input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        check_parse_errors(parser);

        let statements_len = program.statements.len();
        assert_eq!(
            statements_len, 1,
            "program has not enough statements. got={}",
            statements_len,
        );

        assert_eq!(
            NodeType::ExpressionStetement,
            program.statements[0].get_type(),
            "program.statements[0] is not ast::ExpressionStetement. got={}",
            program.statements[0].get_type()
        );
        let stmt = program.statements[0].try_into_expr_stmt().unwrap();

        let expr = stmt.expression.as_ref().unwrap();
        assert_eq!(
            NodeType::IfExpression,
            expr.get_type(),
            "stmt.expression is not ast::IfExpression. got={}",
            expr.get_type()
        );
        let if_expr = stmt
            .expression
            .as_ref()
            .unwrap()
            .try_into_if_expr()
            .unwrap();

        test_infix_expression(
            if_expr.condition.as_ref().unwrap().clone(),
            test.left_value,
            test.operator,
            test.right_value,
        );

        let conseq_len = if_expr.consequence.as_ref().unwrap().statements.len();
        assert_eq!(
            conseq_len, 1,
            "consequence is not 1 statement. got={}",
            conseq_len
        );

        let conseq = if_expr.consequence.as_ref().unwrap();
        assert_eq!(
            NodeType::ExpressionStetement,
            conseq.statements[0].get_type(),
            "consequence.statements[0] is not ast::ExpressionStetement. got={}",
            conseq.statements[0].get_type()
        );
        let consequence = conseq.statements[0].try_into_expr_stmt().unwrap();

        let conseq_expr = consequence.expression.as_ref().unwrap().clone();
        test_identifier(conseq_expr, "x".to_string());

        if if_expr.alternative.is_some() {
            eprintln!("if_expr.alternative.statements was not None.",)
        }
    }

    #[test]
    fn test_if_else_expression() {
        let test = InfixTest {
            input: "if (x < y) { x } else { y }".to_string(),
            left_value: Box::new("x".to_string()),
            operator: "<".to_string(),
            right_value: Box::new("y".to_string()),
        };

        let lexer = Lexer::new(test.input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        check_parse_errors(parser);

        let statements_len = program.statements.len();
        assert_eq!(
            statements_len, 1,
            "program has not enough statements. got={}",
            statements_len,
        );

        assert_eq!(
            NodeType::ExpressionStetement,
            program.statements[0].get_type(),
            "program.statements[0] is not ast::ExpressionStetement. got={}",
            program.statements[0].get_type(),
        );
        let stmt = program.statements[0].try_into_expr_stmt().unwrap();

        let expr = stmt.expression.as_ref().unwrap();
        assert_eq!(
            NodeType::IfExpression,
            expr.get_type(),
            "stmt.expression is not ast::IfExpression. got={}",
            expr.get_type()
        );
        let if_expr = expr.try_into_if_expr().unwrap();

        test_infix_expression(
            if_expr.condition.as_ref().unwrap().clone(),
            test.left_value,
            test.operator,
            test.right_value,
        );

        let conseq_len = if_expr.consequence.as_ref().unwrap().statements.len();
        assert_eq!(
            conseq_len, 1,
            "consequence is not 1 statement. got={}",
            conseq_len
        );
        let consequence = if_expr.consequence.as_ref().unwrap();
        assert_eq!(
            NodeType::ExpressionStetement,
            consequence.statements[0].get_type(),
            "consequence.statements[0] is not ast::ExpressionStetement. got={}",
            consequence.statements[0].get_type(),
        );

        let conseq_expr = consequence.statements[0]
            .try_into_expr_stmt()
            .unwrap()
            .expression
            .as_ref()
            .unwrap()
            .clone();

        test_identifier(conseq_expr, "x".to_string());

        let alternative = if_expr.alternative.as_ref().unwrap();
        assert_eq!(
            NodeType::ExpressionStetement,
            alternative.statements[0].get_type(),
            "alternative.statements[0] is not ast::ExpressionStetement. got={}",
            alternative.statements[0].get_type()
        );

        let alt_expr = alternative.statements[0]
            .try_into_expr_stmt()
            .unwrap()
            .expression
            .as_ref()
            .unwrap()
            .clone();
        test_identifier(alt_expr, "y".to_string());
    }

    fn test_infix_expression(
        expr: Rc<dyn Expression>,
        left: Box<dyn Any>,
        operator: String,
        right: Box<dyn Any>,
    ) {
        let op_expr = expr.try_into_infix_expr().unwrap();

        test_literal_expression(op_expr.left.as_ref().unwrap().clone(), left);

        assert_eq!(
            operator, op_expr.operator,
            "expr.operator is not {}. got={}",
            operator, op_expr.operator
        );

        test_literal_expression(op_expr.right.as_ref().unwrap().clone(), right);
    }

    #[test]
    fn test_operator_precedence_parsing() {
        struct TestStruct {
            input: String,
            expected: String,
        }

        let tests = Vec::from([
            TestStruct {
                input: String::from("true"),
                expected: String::from("true"),
            },
            TestStruct {
                input: String::from("false"),
                expected: String::from("false"),
            },
            TestStruct {
                input: String::from("3 > 5 == false"),
                expected: String::from("((3 > 5) == false)"),
            },
            TestStruct {
                input: String::from("3 < 5 == true"),
                expected: String::from("((3 < 5) == true)"),
            },
            TestStruct {
                input: String::from("-a * b"),
                expected: String::from("((-a) * b)"),
            },
            TestStruct {
                input: String::from("!-a"),
                expected: String::from("(!(-a))"),
            },
            TestStruct {
                input: String::from("a + b + c"),
                expected: String::from("((a + b) + c)"),
            },
            TestStruct {
                input: String::from("a + b - c"),
                expected: String::from("((a + b) - c)"),
            },
            TestStruct {
                input: String::from("a * b * c"),
                expected: String::from("((a * b) * c)"),
            },
            TestStruct {
                input: String::from("a * b / c"),
                expected: String::from("((a * b) / c)"),
            },
            TestStruct {
                input: String::from("a + b / c"),
                expected: String::from("(a + (b / c))"),
            },
            TestStruct {
                input: String::from("a + b * c + d / e - f"),
                expected: String::from("(((a + (b * c)) + (d / e)) - f)"),
            },
            TestStruct {
                input: String::from("3 + 4; -5 * 5"),
                expected: String::from("(3 + 4)((-5) * 5)"),
            },
            TestStruct {
                input: String::from("5 > 4 == 3 < 4"),
                expected: String::from("((5 > 4) == (3 < 4))"),
            },
            TestStruct {
                input: String::from("5 < 4 != 3 > 4"),
                expected: String::from("((5 < 4) != (3 > 4))"),
            },
            TestStruct {
                input: String::from("3 + 4 * 5 == 3 * 1 + 4 * 5"),
                expected: String::from("((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))"),
            },
            TestStruct {
                input: String::from("3 + 4 * 5 == 3 * 1 + 4 * 5"),
                expected: String::from("((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))"),
            },
            TestStruct {
                input: String::from("1 + (2 + 3) + 4"),
                expected: String::from("((1 + (2 + 3)) + 4)"),
            },
            TestStruct {
                input: String::from("(5 + 5) * 2"),
                expected: String::from("((5 + 5) * 2)"),
            },
            TestStruct {
                input: String::from("2 / (5 + 5)"),
                expected: String::from("(2 / (5 + 5))"),
            },
            TestStruct {
                input: String::from("-(5 + 5)"),
                expected: String::from("(-(5 + 5))"),
            },
            TestStruct {
                input: String::from("!(true == true)"),
                expected: String::from("(!(true == true))"),
            },
            TestStruct {
                input: String::from("a + add(b * c) + d"),
                expected: String::from("((a + add((b * c))) + d)"),
            },
            TestStruct {
                input: String::from("add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))"),
                expected: String::from("add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))"),
            },
            TestStruct {
                input: String::from("add(a + b + c * d / f + g)"),
                expected: String::from("add((((a + b) + ((c * d) / f)) + g))"),
            },
            TestStruct {
                input: String::from("a * [1, 2, 3, 4][b * c] * d"),
                expected: String::from("((a * ([1, 2, 3, 4][(b * c)])) * d)"),
            },
            TestStruct {
                input: String::from("add(a * b[2], b[1], 2 * [1, 2][1])"),
                expected: String::from("add((a * (b[2])), (b[1]), (2 * ([1, 2][1])))"),
            },
        ]);

        for test in tests {
            let lexer = Lexer::new(test.input);
            let mut parser = Parser::new(lexer);
            let program = parser.parse_program();
            check_parse_errors(parser);

            let actual = program.to_string();

            assert_eq!(
                test.expected, actual,
                "expected={}, got={}",
                test.expected, actual
            );
        }
    }

    fn test_identifier(expr: Rc<dyn Expression>, value: String) {
        let ident = expr.try_into_identifier().unwrap();

        assert_eq!(
            value, ident.value,
            "ident.value not {}. got={}",
            value, ident.value
        );

        assert_eq!(
            value,
            ident.token_literal(),
            "ident.token_literal not {}. got={}",
            value,
            ident.token_literal()
        );
    }

    fn test_boolean_literal(expr: Rc<dyn Expression>, value: bool) {
        let ident = expr.try_into_boolean().unwrap();

        assert_eq!(
            value, ident.value,
            "ident.value not {}. got={}",
            value, ident.value
        );

        assert_eq!(
            value.to_string(),
            ident.token_literal(),
            "ident.token_literal not {}. got={}",
            value,
            ident.token_literal()
        );
    }

    fn test_literal_expression(expr: Rc<dyn Expression>, expected: Box<dyn Any>) {
        if expected.downcast_ref::<i64>().is_some() {
            test_int_literal(expr, *expected.downcast_ref::<i64>().unwrap())
        } else if expected.downcast_ref::<String>().is_some() {
            test_identifier(expr, expected.downcast_ref::<String>().unwrap().to_string())
        } else if expected.downcast_ref::<bool>().is_some() {
            test_boolean_literal(expr, *expected.downcast_ref::<bool>().unwrap())
        }
    }

    #[test]
    fn test_function_parameter_parsing() {
        struct TestStruct {
            input: String,
            expected_params: Vec<String>,
        }
        let tests = Vec::from([
            TestStruct {
                input: "fn() {};".to_string(),
                expected_params: Vec::from([]),
            },
            TestStruct {
                input: "fn(x) {};".to_string(),
                expected_params: Vec::from(["x".to_string()]),
            },
            TestStruct {
                input: "fn(x, y, z) {};".to_string(),
                expected_params: Vec::from(["x".to_string(), "y".to_string(), "z".to_string()]),
            },
        ]);

        for test in tests {
            let lexer = Lexer::new(test.input);
            let mut parser = Parser::new(lexer);
            let program = parser.parse_program();
            check_parse_errors(parser);

            let stmt = program.statements[0].try_into_expr_stmt().unwrap();
            let function = stmt
                .expression
                .as_ref()
                .unwrap()
                .try_into_fn_literal()
                .unwrap();

            assert_eq!(
                test.expected_params.len(),
                function.parameters.len(),
                "length parameters wrong. want {}, got={}",
                test.expected_params.len(),
                function.parameters.len()
            );

            let mut index = 0;
            for ident in test.expected_params {
                test_literal_expression(function.parameters[index].clone(), Box::new(ident));
                index += 1;
            }
        }
    }

    #[test]
    fn test_call_expression_parsing() {
        let input = String::from("add(1, 2 * 3, 4 + 5);");

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        check_parse_errors(parser);

        let statements_len = program.statements.len();
        assert_eq!(
            statements_len, 1,
            "program has not enough statements. got={}",
            statements_len,
        );

        assert_eq!(
            NodeType::ExpressionStetement,
            program.statements[0].get_type(),
            "program.statements[0] is not ast::ExpressionStetement. got={}",
            program.statements[0].get_type()
        );
        let stmt = program.statements[0]
            .try_into_expr_stmt()
            .unwrap()
            .expression
            .as_ref()
            .unwrap();

        assert_eq!(
            NodeType::CallExpression,
            stmt.get_type(),
            "stmt.expression is not ast::CallExpression. got={}",
            stmt.get_type(),
        );

        let call_expr = stmt.try_into_call_expr().unwrap();

        test_identifier(call_expr.function.clone(), String::from("add"));

        let args_len = call_expr.arguments.len();
        assert_eq!(args_len, 3, "wrong length of arguments. got={}", args_len,);

        test_literal_expression(call_expr.arguments[0].clone(), Box::new(1));
        test_infix_expression(
            call_expr.arguments[1].clone(),
            Box::new(2),
            String::from("*"),
            Box::new(3),
        );
        test_infix_expression(
            call_expr.arguments[2].clone(),
            Box::new(4),
            String::from("+"),
            Box::new(5),
        );
    }

    #[test]
    fn test_function_literal_parsing() {
        let test = InfixTest {
            input: "fn(x, y) { x + y; }".to_string(),
            left_value: Box::new("x".to_string()),
            operator: "+".to_string(),
            right_value: Box::new("y".to_string()),
        };

        let lexer = Lexer::new(test.input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        check_parse_errors(parser);

        let statements_len = program.statements.len();
        assert_eq!(
            statements_len, 1,
            "program has not enough statements. got={}",
            statements_len,
        );

        assert_eq!(
            NodeType::ExpressionStetement,
            program.statements[0].get_type(),
            "program.statements[0] is not ast::ExpressionStetement. got={}",
            program.statements[0].get_type()
        );
        let stmt = program.statements[0]
            .try_into_expr_stmt()
            .unwrap()
            .expression
            .as_ref()
            .unwrap();

        assert_eq!(
            NodeType::FunctionLiteral,
            stmt.get_type(),
            "stmt.expression is not ast::FunctionLiteral. got={}",
            stmt.get_type()
        );
        let func_literal = stmt.try_into_fn_literal().unwrap();

        let param_len = func_literal.parameters.len();
        assert_eq!(
            param_len, 2,
            "function literal parameters wrong. want 2, got={}",
            param_len,
        );

        test_literal_expression(
            func_literal.parameters[0].clone(),
            Box::new("x".to_string()),
        );
        test_literal_expression(
            func_literal.parameters[1].clone(),
            Box::new("y".to_string()),
        );

        let body_stmts_len = func_literal.body.statements.len();
        assert_eq!(
            body_stmts_len, 1,
            "function.body.statements has not 1 statements, got={}",
            body_stmts_len,
        );

        assert_eq!(
            NodeType::ExpressionStetement,
            func_literal.body.statements[0].get_type(),
            "function.body.stmt.expression is not ast::ExpressionStetement. got={}",
            func_literal.body.statements[0].get_type()
        );
        let body_stmt_expr = func_literal.body.statements[0]
            .try_into_expr_stmt()
            .unwrap();

        test_infix_expression(
            body_stmt_expr.expression.as_ref().unwrap().clone(),
            test.left_value,
            test.operator,
            test.right_value,
        );
    }

    #[test]
    fn test_return_statements() {
        let input = "
                    return 5;
                    return 10;
                    return 993322;
        "
        .to_string();

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        check_parse_errors(parser);

        let statements_len = program.statements.len();
        assert_eq!(
            statements_len, 3,
            "program has not enough statements. got={}",
            statements_len,
        );

        for stmt in program.statements {
            assert_eq!(
                NodeType::ReturnStatement,
                stmt.get_type(),
                "program.statement is not ast::ReturnStatement. got={}",
                stmt.get_type()
            );
            let return_stmt = stmt.try_into_return_stmt().unwrap();

            assert_eq!(
                "return",
                return_stmt.token_literal(),
                "return_stmt.token_literal not 'return', got={}",
                return_stmt.token_literal()
            );
        }
    }
    #[test]
    fn test_string_literal_expression() {
        let input = "\"hello world\";".to_string();

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        check_parse_errors(parser);

        let stmt = program.statements[0].try_into_expr_stmt().unwrap();
        let expr = stmt.expression.as_ref().unwrap();
        assert_eq!(
            expr.get_type(),
            NodeType::StringLiteral,
            "exp not StringLiteral. got={}",
            expr.get_type(),
        );
        let str_literal = expr.try_into_str_literal().unwrap();

        assert_eq!(
            "hello world".to_string(),
            str_literal.value,
            "literal.value not {} got={}, \"hello world\"",
            "hello world".to_string(),
            str_literal.value,
        );
    }

    #[test]
    fn test_parsing_array_literal() {
        let input = "[1, 2 * 2, 3 + 3]".to_string();

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        check_parse_errors(parser);

        let stmt = program.statements[0].try_into_expr_stmt().unwrap();
        let expr = stmt.expression.as_ref().unwrap();
        assert_eq!(
            expr.get_type(),
            NodeType::ArrayLiteral,
            "exp not ArrayLiteral. got={}",
            expr.get_type(),
        );
        let arr_literal = expr.try_into_array_literal().unwrap();

        assert_eq!(
            arr_literal.elements.len(),
            3,
            "array.elements.len not 3. got={}",
            arr_literal.elements.len(),
        );

        test_int_literal(arr_literal.elements[0].clone(), 1);
        test_infix_expression(
            arr_literal.elements[1].clone(),
            Box::new(2),
            "*".to_string(),
            Box::new(2),
        );
        test_infix_expression(
            arr_literal.elements[2].clone(),
            Box::new(3),
            "+".to_string(),
            Box::new(3),
        );
    }

    #[test]
    fn test_parsing_index_expressions() {
        let input = "myArray[1 + 1]".to_string();

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        check_parse_errors(parser);

        let stmt = program.statements[0].try_into_expr_stmt().unwrap();
        let expr = stmt.expression.as_ref().unwrap();
        assert_eq!(
            expr.get_type(),
            NodeType::IndexExpression,
            "exp not IndexExpression. got={}",
            expr.get_type(),
        );
        let index_expr = expr.try_into_index_expr().unwrap();

        test_identifier(index_expr.left.clone(), "myArray".to_string());
        test_infix_expression(
            index_expr.index.clone(),
            Box::new(1),
            "+".to_string(),
            Box::new(1),
        );
    }

    #[test]
    fn test_parsing_hash_literals_string_keys() {
        let input = "{\"one\": 1, \"two\": 2, \"three\": 3}".to_string();

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        check_parse_errors(parser);

        let stmt = program.statements[0].try_into_expr_stmt().unwrap();
        let expr = stmt.expression.as_ref().unwrap();
        assert_eq!(
            expr.get_type(),
            NodeType::HashLiteral,
            "exp not HashLiteral. got={}",
            expr.get_type(),
        );
        let hash_literal = expr.try_into_hash_literal().unwrap();

        assert_eq!(
            hash_literal.pairs.len(),
            3,
            "hash.pairs has wrong length. got={}",
            hash_literal.pairs.len(),
        );

        let expected = HashMap::from([
            ("one".to_string(), 1),
            ("two".to_string(), 2),
            ("three".to_string(), 3),
        ]);

        for (key, value) in hash_literal.pairs.clone() {
            match key.try_into_str_literal() {
                Err(_) => assert!(false, "key is not StringLiteral. got={}", key.get_type()),
                Ok(str_literal) => {
                    let expected_value = expected.get(&str_literal.to_string());
                    match expected_value {
                        Some(val) => test_int_literal(value, *val),
                        None => assert!(false, "wrong key. got={}", str_literal.to_string()),
                    }
                }
            }
        }
    }

    #[test]
    fn test_parsing_empty_hash_literal() {
        let input = "{}".to_string();

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        check_parse_errors(parser);

        let stmt = program.statements[0].try_into_expr_stmt().unwrap();
        let expr = stmt.expression.as_ref().unwrap();
        assert_eq!(
            expr.get_type(),
            NodeType::HashLiteral,
            "exp not HashLiteral. got={}",
            expr.get_type(),
        );

        let hash = expr.try_into_hash_literal().unwrap();

        assert_eq!(
            hash.pairs.len(),
            0,
            "hash.pairs has wrong length. got={}",
            hash.pairs.len(),
        );
    }

    #[test]
    fn test_parsing_hash_literal_with_expressions() {
        struct TestFn {
            callback: fn(Rc<dyn Expression>),
        }

        let input = "{\"one\": 0 + 1, \"two\": 10 - 8, \"three\": 15 / 5}".to_string();

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        check_parse_errors(parser);

        let stmt = program.statements[0].try_into_expr_stmt().unwrap();
        let expr = stmt.expression.as_ref().unwrap();
        assert_eq!(
            expr.get_type(),
            NodeType::HashLiteral,
            "exp not HashLiteral. got={}",
            expr.get_type(),
        );

        let hash = expr.try_into_hash_literal().unwrap();

        assert_eq!(
            hash.pairs.len(),
            3,
            "hash.pairs has wrong length. got={}",
            hash.pairs.len(),
        );

        let tests: HashMap<String, TestFn> = HashMap::from([
            (
                "one".to_string(),
                TestFn {
                    callback: |expr: Rc<dyn Expression>| {
                        test_infix_expression(expr, Box::new(0), "+".to_string(), Box::new(1))
                    },
                },
            ),
            (
                "two".to_string(),
                TestFn {
                    callback: |expr: Rc<dyn Expression>| {
                        test_infix_expression(expr, Box::new(10), "-".to_string(), Box::new(8))
                    },
                },
            ),
            (
                "three".to_string(),
                TestFn {
                    callback: |expr: Rc<dyn Expression>| {
                        test_infix_expression(expr, Box::new(15), "/".to_string(), Box::new(5))
                    },
                },
            ),
        ]);

        for (key, value) in hash.pairs.clone() {
            match key.try_into_str_literal() {
                Err(_) => assert!(false, "key is not StringLiteral. got={}", key.get_type()),
                Ok(str_literal) => {
                    let test_fn = tests.get(&str_literal.to_string());
                    match test_fn {
                        None => assert!(
                            false,
                            "No test function for key {}",
                            str_literal.to_string()
                        ),
                        Some(function) => (function.callback)(value),
                    }
                }
            }
        }
    }
}
