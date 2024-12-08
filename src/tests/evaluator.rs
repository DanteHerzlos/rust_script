#[cfg(test)]
mod evaluator_tests {
    use std::{
        any::{type_name_of_val, Any},
        cell::RefCell,
        collections::HashMap,
        rc::Rc,
    };

    use crate::{
        ast::Node,
        evaluator::eval,
        lexer::Lexer,
        object::{self, Environment, Object, ObjectType, NULL},
        parser::Parser,
    };

    #[test]
    fn test_eval_int_expression() {
        struct TestStruct {
            input: String,
            expected: i64,
        }

        let tests = Vec::from([
            TestStruct {
                input: "5".to_string(),
                expected: 5,
            },
            TestStruct {
                input: "10".to_string(),
                expected: 10,
            },
            TestStruct {
                input: "-5".to_string(),
                expected: -5,
            },
            TestStruct {
                input: "-10".to_string(),
                expected: -10,
            },
            TestStruct {
                input: "5 + 5 + 5 + 5 - 10".to_string(),
                expected: 10,
            },
            TestStruct {
                input: "2 * 2 * 2 * 2 * 2".to_string(),
                expected: 32,
            },
            TestStruct {
                input: "-50 + 100 + -50".to_string(),
                expected: 0,
            },
            TestStruct {
                input: "5 * 2 + 10".to_string(),
                expected: 20,
            },
            TestStruct {
                input: "5 + 2 * 10".to_string(),
                expected: 25,
            },
            TestStruct {
                input: "20 + 2 * -10".to_string(),
                expected: 0,
            },
            TestStruct {
                input: "50 / 2 * 2 + 10".to_string(),
                expected: 60,
            },
            TestStruct {
                input: "2 * (5 + 10)".to_string(),
                expected: 30,
            },
            TestStruct {
                input: "3 * 3 * 3 + 10".to_string(),
                expected: 37,
            },
            TestStruct {
                input: "3 * (3 * 3) + 10".to_string(),
                expected: 37,
            },
            TestStruct {
                input: "(5 + 10 * 2 + 15 / 3) * 2 + -10".to_string(),
                expected: 50,
            },
        ]);

        for test in tests {
            let evaluated = test_eval(test.input);
            test_int_object(evaluated, test.expected)
        }
    }

    #[test]
    fn test_eval_bool_expression() {
        struct TestStruct {
            input: String,
            expected: bool,
        }

        let tests = Vec::from([
            TestStruct {
                input: "true".to_string(),
                expected: true,
            },
            TestStruct {
                input: "false".to_string(),
                expected: false,
            },
            TestStruct {
                input: "1 < 2".to_string(),
                expected: true,
            },
            TestStruct {
                input: "1 > 2".to_string(),
                expected: false,
            },
            TestStruct {
                input: "1 < 1".to_string(),
                expected: false,
            },
            TestStruct {
                input: "1 > 1".to_string(),
                expected: false,
            },
            TestStruct {
                input: "1 == 1".to_string(),
                expected: true,
            },
            TestStruct {
                input: "1 != 1".to_string(),
                expected: false,
            },
            TestStruct {
                input: "1 == 2".to_string(),
                expected: false,
            },
            TestStruct {
                input: "1 != 2".to_string(),
                expected: true,
            },
            TestStruct {
                input: "true == true".to_string(),
                expected: true,
            },
            TestStruct {
                input: "false == false".to_string(),
                expected: true,
            },
            TestStruct {
                input: "true == false".to_string(),
                expected: false,
            },
            TestStruct {
                input: "true != false".to_string(),
                expected: true,
            },
            TestStruct {
                input: "false != true".to_string(),
                expected: true,
            },
            TestStruct {
                input: "(1 < 2) == true".to_string(),
                expected: true,
            },
            TestStruct {
                input: "(1 < 2) == false".to_string(),
                expected: false,
            },
            TestStruct {
                input: "(1 > 2) == true".to_string(),
                expected: false,
            },
            TestStruct {
                input: "(1 > 2) == false".to_string(),
                expected: true,
            },
        ]);

        for test in tests {
            let evaluated = test_eval(test.input);
            test_bool_object(evaluated, test.expected)
        }
    }

    #[test]
    fn test_bang_operator() {
        struct TestStruct {
            input: String,
            expected: bool,
        }

        let tests = Vec::from([
            TestStruct {
                input: "!true".to_string(),
                expected: false,
            },
            TestStruct {
                input: "!false".to_string(),
                expected: true,
            },
            TestStruct {
                input: "!5".to_string(),
                expected: false,
            },
            TestStruct {
                input: "!!true".to_string(),
                expected: true,
            },
            TestStruct {
                input: "!!false".to_string(),
                expected: false,
            },
            TestStruct {
                input: "!!5".to_string(),
                expected: true,
            },
        ]);

        for test in tests {
            let evaluated = test_eval(test.input);
            test_bool_object(evaluated, test.expected)
        }
    }

    #[test]
    fn test_if_else_expressions() {
        struct TestStruct {
            input: String,
            expected: Box<dyn Any>,
        }

        let tests = Vec::from([
            TestStruct {
                input: "if (true) { 10 }".to_string(),
                expected: Box::new(10),
            },
            TestStruct {
                input: "if (false) { 10 }".to_string(),
                expected: Box::new("null"),
            },
            TestStruct {
                input: "if (1) { 10 }".to_string(),
                expected: Box::new(10),
            },
            TestStruct {
                input: "if (1 < 2) { 10 }".to_string(),
                expected: Box::new(10),
            },
            TestStruct {
                input: "if (1 > 2) { 10 }".to_string(),
                expected: Box::new("null"),
            },
            TestStruct {
                input: "if (1 > 2) { 10 } else {20}".to_string(),
                expected: Box::new(20),
            },
            TestStruct {
                input: "if (1 < 2) { 10 } else {20}".to_string(),
                expected: Box::new(10),
            },
        ]);

        for test in tests {
            let evaluated = test_eval(test.input);
            if test.expected.downcast_ref::<i64>().is_some() {
                test_int_object(evaluated, *test.expected.downcast_ref::<i64>().unwrap())
            } else {
                test_null_object(evaluated)
            }
        }
    }

    #[test]
    fn test_return_statement() {
        struct TestStruct {
            input: String,
            expected: i64,
        }

        let tests = Vec::from([
            TestStruct {
                input: "return 10;".to_string(),
                expected: 10,
            },
            TestStruct {
                input: "return 10; 9;".to_string(),
                expected: 10,
            },
            TestStruct {
                input: "return 2 * 5; 9;".to_string(),
                expected: 10,
            },
            TestStruct {
                input: "9; return 2 * 5; 9;".to_string(),
                expected: 10,
            },
            TestStruct {
                input: "
                    if (10 > 1) {
                        if (10 > 1) {
                            return 10;
                        }
                        return 1;
                    }"
                .to_string(),
                expected: 10,
            },
        ]);

        for test in tests {
            let evaluated = test_eval(test.input);
            test_int_object(evaluated, test.expected)
        }
    }

    #[test]
    fn test_error_handling() {
        struct TestStruct {
            input: String,
            expected_message: String,
        }

        let tests = Vec::from([
            TestStruct {
                input: "5 + true;".to_string(),
                expected_message: "type mismatch: INTEGER + BOOLEAN".to_string(),
            },
            TestStruct {
                input: "5 + true; 5;".to_string(),
                expected_message: "type mismatch: INTEGER + BOOLEAN".to_string(),
            },
            TestStruct {
                input: "-true".to_string(),
                expected_message: "unknown operator: -BOOLEAN".to_string(),
            },
            TestStruct {
                input: "true + false".to_string(),
                expected_message: "unknown operator: BOOLEAN + BOOLEAN".to_string(),
            },
            TestStruct {
                input: "5; true + false; 5".to_string(),
                expected_message: "unknown operator: BOOLEAN + BOOLEAN".to_string(),
            },
            TestStruct {
                input: "if (10 > 1) { true + false; }".to_string(),
                expected_message: "unknown operator: BOOLEAN + BOOLEAN".to_string(),
            },
            TestStruct {
                input: "
                    if (10 > 1) {
                        if (10 > 1) {
                            return true + false;
                        }
                        return 1;
                    }"
                .to_string(),
                expected_message: "unknown operator: BOOLEAN + BOOLEAN".to_string(),
            },
            TestStruct {
                input: "foobar".to_string(),
                expected_message: "identifier not found: foobar".to_string(),
            },
            TestStruct {
                input: "\"Hello\" - \"World\"".to_string(),
                expected_message: "unknown operator: STRING - STRING".to_string(),
            },
            TestStruct {
                input: "{\"name\": \"Monkey\"}[fn(x) { x }];".to_string(),
                expected_message: "unusable as hash key: FUNCTION".to_string(),
            },
        ]);

        for test in tests {
            let evaluated = test_eval(test.input);

            if evaluated.try_into_error().is_err() {
                assert!(false, "no error object returned.");
                continue;
            }

            let msg = evaluated.try_into_error().unwrap().message.clone();
            assert_eq!(
                test.expected_message, msg,
                "wrong error message. expected: {}, got={}",
                test.expected_message, msg,
            );
        }
    }

    #[test]
    fn test_let_statements() {
        struct TestStruct {
            input: String,
            expected: i64,
        }

        let tests = Vec::from([
            TestStruct {
                input: "let a = 5; a;".to_string(),
                expected: 5,
            },
            TestStruct {
                input: "let a = 5 * 5; a;".to_string(),
                expected: 25,
            },
            TestStruct {
                input: "let a = 5; let b = 5; b;".to_string(),
                expected: 5,
            },
            TestStruct {
                input: "let a = 5; let b = a; b;".to_string(),
                expected: 5,
            },
            TestStruct {
                input: "let a = 5; let b = a; let c = a + b + 5; c;".to_string(),
                expected: 15,
            },
        ]);

        for test in tests {
            test_int_object(test_eval(test.input), test.expected);
        }
    }

    #[test]
    fn test_function_object() {
        struct TestStruct {
            input: String,
            expected: i64,
        }

        let tests = Vec::from([
            TestStruct {
                input: "let identity = fn(x) { x; }; identity(5);".to_string(),
                expected: 5,
            },
            TestStruct {
                input: "let identity = fn(x) { return x; }; identity(5);".to_string(),
                expected: 5,
            },
            TestStruct {
                input: "let double = fn(x) { x * 2; }; double(5);".to_string(),
                expected: 10,
            },
            TestStruct {
                input: "let add = fn(x, y) { x + y; }; add(5, 5);".to_string(),
                expected: 10,
            },
            TestStruct {
                input: "let add = fn(x, y) { x + y; }; add(5 + 5, add(5, 5));".to_string(),
                expected: 20,
            },
            TestStruct {
                input: "fn(x) { x; }(5)".to_string(),
                expected: 5,
            },
        ]);

        for test in tests {
            test_int_object(test_eval(test.input), test.expected);
        }
    }

    #[test]
    fn test_function_app() {
        let input = "fn(x) { x + 2; };".to_string();
        let evaluated = test_eval(input);

        let function = match evaluated.try_into_function() {
            Ok(val) => val.try_into_function().unwrap(),
            Err(_) => {
                assert!(
                    false,
                    "object is not Function. got={}",
                    evaluated.get_type()
                );
                return;
            }
        };

        assert_eq!(
            function.parameters.len(),
            1,
            "function has wrong parameters. Parameters={}",
            function.parameters.len()
        );

        assert_eq!(
            function.parameters[0].to_string(),
            "x".to_string(),
            "parameter is not 'x'. got={}",
            function.parameters[0].to_string()
        );

        assert_eq!(
            function.body.to_string(),
            "(x + 2)".to_string(),
            "body is not '(x + 2)'. got={}",
            function.body.to_string()
        );
    }

    #[test]
    fn test_closures() {
        let input = "
                let newAdder = fn(x) {
                fn(y) { x + y };
                };
                let addTwo = newAdder(2);
                addTwo(2);"
            .to_string();

        test_int_object(test_eval(input), 4);
    }

    #[test]
    fn test_string_literal() {
        let input = "\"Hello World!\"".to_string();
        let evaluated = test_eval(input);

        match evaluated.try_into_str() {
            Err(_) => assert!(false, "object is not String. got={}", evaluated.get_type()),
            Ok(val) => assert_eq!(
                val.value, "Hello World!",
                "String has wrong value. got={}",
                val.value
            ),
        }
    }

    #[test]
    fn test_string_concatenation() {
        let input = "\"Hello\" + \" \" + \"World!\"".to_string();
        let evaluated = test_eval(input);

        match evaluated.try_into_str() {
            Err(_) => assert!(false, "object is not String. got={}", evaluated.get_type()),
            Ok(val) => assert_eq!(
                val.value, "Hello World!",
                "String has wrong value. got={}",
                val.value
            ),
        }
    }

    #[test]
    fn test_builtin_functions() {
        struct TestStruct {
            input: String,
            expected: Box<dyn Any>,
        }

        let tests = Vec::from([
            TestStruct {
                input: "len(\"\")".to_string(),
                expected: Box::new(0),
            },
            TestStruct {
                input: "len(\"four\")".to_string(),
                expected: Box::new(4),
            },
            TestStruct {
                input: "len(\"hello world\")".to_string(),
                expected: Box::new(11),
            },
            TestStruct {
                input: "len(1)".to_string(),
                expected: Box::new("argument to `len` not supported, got INTEGER".to_string()),
            },
            TestStruct {
                input: "len(\"one\", \"two\")".to_string(),
                expected: Box::new("wrong number of arguments. got=2, want=1".to_string()),
            },
        ]);

        for test in tests {
            let evaluated = test_eval(test.input);
            if test.expected.downcast_ref::<i64>().is_some() {
                test_int_object(evaluated, *test.expected.downcast_ref::<i64>().unwrap())
            } else if test.expected.downcast_ref::<String>().is_some() {
                let expected = test.expected.downcast_ref::<String>().unwrap();
                match evaluated.try_into_error() {
                    Ok(val) => assert_eq!(
                        expected.clone(),
                        val.message,
                        "wrong error message. expected={}, got={}",
                        expected,
                        val.message
                    ),
                    Err(_) => assert!(false, "object is not Error. got={}", evaluated.get_type()),
                }
            }
        }
    }

    #[test]
    fn test_array_literals() {
        let input = "[1, 2 * 2, 3 + 3]".to_string();
        let evaluated = test_eval(input);

        match evaluated.try_into_array() {
            Err(_) => assert!(false, "object is not Array. got={}", evaluated.get_type()),
            Ok(val) => {
                assert_eq!(
                    val.elements.len(),
                    3,
                    "array has wrong num of elements. got={}",
                    val.elements.len()
                );

                test_int_object(val.elements[0].clone(), 1);
                test_int_object(val.elements[1].clone(), 4);
                test_int_object(val.elements[2].clone(), 6);
            }
        }
    }

    #[test]
    fn test_array_index_expressions() {
        struct TestStruct {
            input: String,
            expected: Box<dyn Any>,
        }

        let tests = Vec::from([
            TestStruct {
                input: "[1, 2, 3][0]".to_string(),
                expected: Box::new(1),
            },
            TestStruct {
                input: "[1, 2, 3][1]".to_string(),
                expected: Box::new(2),
            },
            TestStruct {
                input: "[1, 2, 3][2]".to_string(),
                expected: Box::new(3),
            },
            TestStruct {
                input: "let i = 0; [1][i];".to_string(),
                expected: Box::new(1),
            },
            TestStruct {
                input: "[1, 2, 3][1 + 1];".to_string(),
                expected: Box::new(3),
            },
            TestStruct {
                input: "let myArray = [1, 2, 3]; myArray[2];".to_string(),
                expected: Box::new(3),
            },
            TestStruct {
                input: "let myArray = [1, 2, 3]; myArray[0] + myArray[1] + myArray[2];".to_string(),
                expected: Box::new(6),
            },
            TestStruct {
                input: "let myArray = [1, 2, 3]; let i = myArray[0]; myArray[i];".to_string(),
                expected: Box::new(2),
            },
            TestStruct {
                input: "[1, 2, 3][3]".to_string(),
                expected: Box::new(NULL {}),
            },
            TestStruct {
                input: "[1, 2, 3][3]".to_string(),
                expected: Box::new(NULL {}),
            },
        ]);

        for test in tests {
            let evaluated = test_eval(test.input);
            if test.expected.downcast_ref::<i64>().is_some() {
                test_int_object(evaluated, *test.expected.downcast_ref::<i64>().unwrap())
            } else {
                test_null_object(evaluated);
            }
        }
    }

    #[test]
    fn test_hash_literals() {
        let input = "
            let two = \"two\";
            {
                \"one\": 10 - 9,
                two: 1 + 1,
                \"thr\" + \"ee\": 6 / 2,
                4: 4,
                true: 5,
                false: 6
            }"
        .to_string();
        let evaluated = test_eval(input);

        assert_eq!(
            evaluated.get_type(),
            ObjectType::HASH,
            "object is not Hash. got={}",
            evaluated.get_type()
        );

        let hash = evaluated.try_into_hash().unwrap();
        let expected = HashMap::from([
            (
                object::StringObj {
                    value: "one".to_string(),
                }
                .hash_key(),
                1,
            ),
            (
                object::StringObj {
                    value: "two".to_string(),
                }
                .hash_key(),
                2,
            ),
            (
                object::StringObj {
                    value: "three".to_string(),
                }
                .hash_key(),
                3,
            ),
            (object::Integer { value: 4 }.hash_key(), 4),
            (object::Boolean { value: true }.hash_key(), 5),
            (object::Boolean { value: false }.hash_key(), 6),
        ]);

        assert_eq!(
            hash.pairs.len(),
            expected.len(),
            "hash has wrong num of pairs. got={}",
            hash.pairs.len()
        );

        for (expected_key, expected_value) in expected {
            match hash.pairs.get(&expected_key) {
                None => assert!(false, "no pair for given key in pairs"),
                Some(pair) => test_int_object(pair.value.clone(), expected_value),
            }
        }
    }

    #[test]
    fn test_hash_index_expressions() {
        struct TestStruct {
            input: String,
            expected: Box<dyn Any>,
        }

        let tests = Vec::from([
            TestStruct {
                input: "{\"foo\": 5}[\"foo\"]".to_string(),
                expected: Box::new(5),
            },
            TestStruct {
                input: "{\"foo\": 5}[\"bar\"]".to_string(),
                expected: Box::new(NULL {}),
            },
            TestStruct {
                input: "let key = \"foo\"; {\"foo\": 5}[key]".to_string(),
                expected: Box::new(5),
            },
            TestStruct {
                input: "{}[\"foo\"]".to_string(),
                expected: Box::new(NULL {}),
            },
            TestStruct {
                input: "{5: 5}[5]".to_string(),
                expected: Box::new(5),
            },
            TestStruct {
                input: "{true: 5}[true]".to_string(),
                expected: Box::new(5),
            },
            TestStruct {
                input: "{false: 5}[false]".to_string(),
                expected: Box::new(5),
            },
        ]);

        for test in tests {
            let evaluated = test_eval(test.input);
            if test.expected.downcast_ref::<i64>().is_some() {
                test_int_object(evaluated, *test.expected.downcast_ref::<i64>().unwrap())
            } else {
                test_null_object(evaluated);
            }
        }
    }

    fn test_eval(input: String) -> Rc<dyn Object> {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let env = Rc::new(RefCell::new(Environment::new()));

        eval(program.as_node(), env).unwrap()
    }

    fn test_int_object(obj: Rc<dyn Object>, expected: i64) {
        let int_obj = obj.as_int();
        let int_obj_type_name = type_name_of_val(&int_obj).split("::").last().unwrap();

        assert_eq!(
            "Integer", int_obj_type_name,
            "object is not Integer. got={}",
            int_obj_type_name
        );

        assert_eq!(
            expected, int_obj.value,
            "object has wrong value. got={}, want={}",
            expected, int_obj.value
        )
    }

    fn test_bool_object(obj: Rc<dyn Object>, expected: bool) {
        let bool_obj = obj.as_bool();
        let bool_obj_type_name = type_name_of_val(&bool_obj).split("::").last().unwrap();

        assert_eq!(
            "Boolean", bool_obj_type_name,
            "object is not Boolean. got={}",
            bool_obj_type_name
        );

        assert_eq!(
            expected, bool_obj.value,
            "object has wrong value. got={}, want={}",
            expected, bool_obj.value
        )
    }

    fn test_null_object(obj: Rc<dyn Object>) {
        if obj.try_into_null().is_err() {
            eprintln!("object is not NULL");
        }
    }
}
