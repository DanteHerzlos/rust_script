mod builtins;

use builtins::BUILTINS;

use std::{cell::RefCell, collections::HashMap, rc::Rc, sync::LazyLock, vec};

use crate::{
    ast::{self, Identifier, Node},
    object::{self, Builtin, Integer, Object, ObjectType, StringObj},
};

const NULL: object::NULL = object::NULL {};
const TRUE: object::Boolean = object::Boolean { value: true };
const FALSE: object::Boolean = object::Boolean { value: false };

pub fn eval(node: Rc<&dyn Node>, env: Rc<RefCell<object::Environment>>) -> Option<Rc<dyn Object>> {
    match node.get_type() {
        ast::NodeType::BlockStatement => {
            return eval_block_statement(node.try_into_block_stmt().unwrap(), env);
        }

        ast::NodeType::IfExpression => {
            return Some(eval_if_expression(node.try_into_if_expr().unwrap(), env));
        }

        ast::NodeType::PrefixExpression => {
            let prefix_expr = node.try_into_prefix_expr().unwrap();
            let right = eval(prefix_expr.right.as_node(), env);
            if is_error(&right) {
                return right;
            }
            return Some(eval_prefix_expression(&prefix_expr.operator, right?));
        }

        ast::NodeType::InfixExpression => {
            let infix_expr = node.try_into_infix_expr().unwrap();
            let left = eval(infix_expr.left.as_ref().unwrap().as_node(), env.clone());
            if is_error(&left) {
                return left;
            }
            let right = eval(infix_expr.right.as_ref().unwrap().as_node(), env);
            if is_error(&right) {
                return right;
            }
            return Some(eval_infix_expression(&infix_expr.operator, left?, right?));
        }

        ast::NodeType::ExpressionStetement => {
            let expr_stmt = node.try_into_expr_stmt().unwrap();
            return eval(expr_stmt.expression.as_ref().unwrap().as_node(), env);
        }

        ast::NodeType::IntegerLiteral => {
            let value = node.try_into_int_literal().unwrap().value;
            return Some(Rc::new(Integer { value }));
        }

        ast::NodeType::StringLiteral => {
            let value = node.try_into_str_literal().unwrap().value.clone();
            return Some(Rc::new(object::StringObj { value }));
        }

        ast::NodeType::Program => {
            return eval_program(node.try_into_program().unwrap(), env);
        }

        ast::NodeType::Boolean => {
            let bool_node = node.try_into_boolean().unwrap();
            return Some(Rc::new(native_bool_to_boolean_object(bool_node.value)));
        }

        ast::NodeType::ReturnStatement => {
            let return_val = &node.try_into_return_stmt().unwrap().return_value;
            let value = eval(return_val.as_ref()?.as_node(), env);
            if is_error(&value) {
                return value;
            }

            return Some(Rc::new(object::ReturnValue { value: value? }));
        }
        ast::NodeType::LetStatement => {
            let let_stmt = &node.try_into_let_statement().unwrap();
            let value = eval(let_stmt.value.as_ref().unwrap().as_node(), env.clone());
            if is_error(&value) {
                return value;
            }
            env.borrow_mut()
                .set(let_stmt.name.value.clone(), value.unwrap().into());
            return None;
        }

        ast::NodeType::Identifier => {
            return Some(eval_identifier(node.try_into_identifier().unwrap(), env).into());
        }

        ast::NodeType::FunctionLiteral => {
            let fn_literal = node.try_into_fn_literal().unwrap();

            return Some(Rc::new(object::Function {
                parameters: fn_literal.parameters.clone(),
                body: fn_literal.body.clone(),
                env,
            }));
        }

        ast::NodeType::CallExpression => {
            let call_expr = node.try_into_call_expr().unwrap();
            let function = eval(call_expr.function.clone().as_node(), env.clone());

            if is_error(&function) {
                return function;
            }

            let args = eval_expression(&call_expr.arguments, env);
            if args.len() == 1 && is_error(&Some(args[0].clone())) {
                return Some(args[0].clone());
            }

            return Some(apply_function(function?, args));
        }

        ast::NodeType::ArrayLiteral => {
            let elements = eval_expression(&node.try_into_array_literal().unwrap().elements, env);

            if elements.len() == 1 && is_error(&Some(elements[0].clone())) {
                return Some(elements[0].clone());
            }

            return Some(Rc::new(object::Array { elements }));
        }

        ast::NodeType::IndexExpression => {
            let index_expr = node.try_into_index_expr().unwrap();
            let left = eval(index_expr.left.as_node(), env.clone());

            if is_error(&left) {
                return left;
            }

            let index = eval(index_expr.index.as_node(), env);

            if is_error(&index) {
                return index;
            }

            return Some(eval_index_expression(left.unwrap(), index.unwrap()));
        }

        ast::NodeType::HashLiteral => {
            return eval_hash_literal(node.try_into_hash_literal().unwrap(), env)
        }
    };
}

fn eval_index_expression(left: Rc<dyn Object>, index: Rc<dyn Object>) -> Rc<dyn Object> {
    if left.get_type() == ObjectType::ARRAY && index.get_type() == ObjectType::INTEGER {
        return eval_array_index_expression(left, index);
    } else if left.get_type() == ObjectType::HASH {
        return eval_hash_index_expression(left, index);
    } else {
        return Rc::new(new_error(format!(
            "index operator not suported: {}",
            left.get_type()
        )));
    }
}

fn eval_array_index_expression(array: Rc<dyn Object>, index: Rc<dyn Object>) -> Rc<dyn Object> {
    let array_obj = array.try_into_array().unwrap();
    let idx = index.try_into_int().unwrap().value;
    let max = (array_obj.elements.len() - 1) as i64;

    if idx < 0 || idx > max {
        return Rc::new(NULL);
    }

    return array_obj.elements[idx as usize].clone();
}

fn eval_hash_index_expression(hash: Rc<dyn Object>, index: Rc<dyn Object>) -> Rc<dyn Object> {
    let hash_obj = hash.try_into_hash().unwrap();
    let hash_key = match index.try_hash_key() {
        Ok(val) => val,
        Err(_) => {
            return Rc::new(new_error(format!(
                "unusable as hash key: {}",
                index.get_type()
            )))
        }
    };

    match hash_obj.pairs.get(&hash_key) {
        None => return Rc::new(NULL),
        Some(pair) => return pair.value.clone(),
    };
}

fn apply_function(function: Rc<dyn Object>, args: Vec<Rc<dyn Object>>) -> Rc<dyn Object> {
    match function.get_type() {
        ObjectType::FUNCTION => {
            let val = function.try_into_function().unwrap();
            let ext_env = extend_function_env(val, args);
            let evaluated = eval(val.body.as_node(), ext_env).unwrap();
            return unwrap_return_value(evaluated);
        }
        ObjectType::BUILTIN => {
            let val = function.try_into_builtin().unwrap();
            return (val.function)(args);
        }
        _ => Rc::new(new_error(format!("not a function {}", function.get_type()))),
    }
}

fn extend_function_env(
    function: &object::Function,
    args: Vec<Rc<dyn Object>>,
) -> Rc<RefCell<object::Environment>> {
    let env = object::Environment::new_enclosed_env(function.env.clone());

    for (param_idx, param) in function.parameters.as_ref().iter().enumerate() {
        env.borrow_mut()
            .set(param.value.to_string(), args[param_idx].clone())
    }

    return env;
}

fn unwrap_return_value(obj: Rc<dyn Object>) -> Rc<dyn Object> {
    match obj.try_into_return_value() {
        Ok(val) => val.value.clone(),
        Err(_) => obj,
    }
}

fn eval_expression(
    exprs: &Vec<Rc<dyn ast::Expression>>,
    env: Rc<RefCell<object::Environment>>,
) -> Vec<Rc<dyn Object>> {
    let mut result = vec![];

    for expr in exprs {
        let evaluated = eval(expr.as_node().clone(), env.clone());

        if is_error(&evaluated) {
            return vec![evaluated.unwrap().into()];
        }

        result.push(evaluated.unwrap().into());
    }

    return result;
}

fn eval_identifier(node: &Identifier, env: Rc<RefCell<object::Environment>>) -> Rc<dyn Object> {
    match env.borrow_mut().get(node.value.clone()) {
        Ok(val) => val.clone(),
        Err(_) => match BUILTINS.get(&node.value.clone()) {
            Some(builtin) => builtin.clone(),
            None => Rc::new(new_error(format!("identifier not found: {}", node.value))),
        },
    }
}

fn native_bool_to_boolean_object(input: bool) -> object::Boolean {
    if input {
        return TRUE;
    } else {
        return FALSE;
    }
}

fn eval_program(
    program: &ast::Program,
    env: Rc<RefCell<object::Environment>>,
) -> Option<Rc<dyn Object>> {
    let mut result = None;

    for stmt in &program.statements {
        result = eval(stmt.as_node(), env.clone());

        if result.is_none() {
            continue;
        }

        match result.as_ref().unwrap().get_type() {
            object::ObjectType::RETURN => {
                return Some(
                    result
                        .unwrap()
                        .try_into_return_value()
                        .unwrap()
                        .value
                        .clone()
                        .into(),
                )
            }
            object::ObjectType::ERROR => return result,
            _ => continue,
        }
    }

    result
}

fn eval_prefix_expression(operator: &str, right: Rc<dyn Object>) -> Rc<dyn Object> {
    match operator {
        "!" => eval_bang_operator_expression(right),
        "-" => eval_minus_prefix_operator_expression(right),
        _ => Rc::new(new_error(format!(
            "unknown operator: {}{}",
            operator,
            right.get_type()
        ))),
    }
}

fn eval_bang_operator_expression(right: Rc<dyn Object>) -> Rc<dyn Object> {
    if right.try_into_bool().is_ok() {
        if right.try_into_bool().unwrap().value {
            return Rc::new(FALSE);
        } else {
            return Rc::new(TRUE);
        }
    }

    if right.try_into_null().is_ok() {
        return Rc::new(TRUE);
    }

    return Rc::new(FALSE);
}

fn eval_minus_prefix_operator_expression(right: Rc<dyn Object>) -> Rc<dyn Object> {
    if right.get_type() != object::ObjectType::INTEGER {
        return Rc::new(new_error(format!(
            "unknown operator: -{}",
            right.get_type()
        )));
    }

    let value = right.try_into_int().unwrap().value;

    return Rc::new(object::Integer { value: -value });
}

fn eval_infix_expression(
    operator: &str,
    left: Rc<dyn Object>,
    right: Rc<dyn Object>,
) -> Rc<dyn Object> {
    if right.try_into_int().is_ok() && left.try_into_int().is_ok() {
        return eval_int_infix_expression(operator, left.as_int(), right.as_int());
    }

    if operator == "==" {
        return Rc::new(native_bool_to_boolean_object(
            left.as_bool().value == right.as_bool().value,
        ));
    }

    if operator == "!=" {
        return Rc::new(native_bool_to_boolean_object(
            left.as_bool().value != right.as_bool().value,
        ));
    }

    if left.get_type() == ObjectType::STRING && right.get_type() == ObjectType::STRING {
        return eval_string_infix_expression(
            operator,
            left.try_into_str().unwrap(),
            right.try_into_str().unwrap(),
        );
    }

    if left.get_type() != right.get_type() {
        return Rc::new(new_error(format!(
            "type mismatch: {} {} {}",
            left.get_type(),
            operator,
            right.get_type()
        )));
    }

    return Rc::new(new_error(format!(
        "unknown operator: {} {} {}",
        left.get_type(),
        operator,
        right.get_type()
    )));
}

fn eval_string_infix_expression(
    operator: &str,
    left: &object::StringObj,
    right: &object::StringObj,
) -> Rc<dyn Object> {
    match operator {
        "+" => Rc::new(StringObj {
            value: format!("{}{}", left.value, right.value),
        }),
        _ => Rc::new(new_error(format!(
            "unknown operator: {} {} {}",
            left.get_type(),
            operator,
            right.get_type()
        ))),
    }
}

fn eval_int_infix_expression(
    operator: &str,
    left: &object::Integer,
    right: &object::Integer,
) -> Rc<dyn Object> {
    match operator {
        "+" => Rc::new(Integer {
            value: left.value + right.value,
        }),
        "-" => Rc::new(Integer {
            value: left.value - right.value,
        }),
        "*" => Rc::new(Integer {
            value: left.value * right.value,
        }),
        "/" => Rc::new(Integer {
            value: left.value / right.value,
        }),
        "<" => Rc::new(native_bool_to_boolean_object(left.value < right.value)),
        ">" => Rc::new(native_bool_to_boolean_object(left.value > right.value)),
        "==" => Rc::new(native_bool_to_boolean_object(left.value == right.value)),
        "!=" => Rc::new(native_bool_to_boolean_object(left.value != right.value)),
        _ => Rc::new(new_error(format!(
            "unknown operator: {} {} {}",
            left.get_type(),
            operator,
            right.get_type()
        ))),
    }
}

fn eval_if_expression(
    if_expr: &ast::IfExpression,
    env: Rc<RefCell<object::Environment>>,
) -> Rc<dyn Object> {
    let condition = eval(if_expr.condition.as_ref().unwrap().as_node(), env.clone());

    if is_error(&condition) {
        return condition.unwrap();
    }

    if is_truthy(condition.unwrap()) {
        let block_stmt = if_expr.consequence.as_ref().unwrap();
        return eval(block_stmt.as_node(), env).unwrap();
    } else if if_expr.alternative.is_some() {
        let alt = if_expr.alternative.as_ref().unwrap();
        return eval(alt.as_node(), env).unwrap();
    } else {
        return Rc::new(NULL);
    }
}

fn is_truthy(obj: Rc<dyn Object>) -> bool {
    return obj.as_bool().value;
}

fn eval_block_statement(
    block: &ast::BlockStatement,
    env: Rc<RefCell<object::Environment>>,
) -> Option<Rc<dyn Object>> {
    let mut result = None;

    for stmt in &block.statements {
        result = eval(stmt.as_node(), env.clone());

        if result.is_some() {
            let rt = result.as_ref().unwrap().get_type();
            if rt == object::ObjectType::RETURN || rt == object::ObjectType::ERROR {
                return result;
            }
        }
    }

    result
}

fn eval_hash_literal(
    hash: &ast::HashLiteral,
    env: Rc<RefCell<object::Environment>>,
) -> Option<Rc<dyn Object>> {
    let mut pairs = HashMap::new();

    for (key_node, key_value) in &hash.pairs {
        let key = eval(key_node.as_node(), env.clone());
        if is_error(&key) {
            return key;
        }

        if key.as_ref().unwrap().try_hash_key().is_err() {
            return Some(Rc::new(new_error(format!(
                "unusable as hash key: {}",
                key_node.get_type()
            ))));
        }

        let value = eval(key_value.as_node(), env.clone());
        if is_error(&value) {
            return value;
        }

        let hash_key = key.as_ref().unwrap().hash_key();

        pairs.insert(
            hash_key,
            object::hash::HashPair {
                key: key.unwrap(),
                value: value.unwrap(),
            },
        );
    }

    Some(Rc::new(object::Hash { pairs }))
}

fn new_error(message: String) -> object::Error {
    return object::Error { message };
}

fn is_error(obj: &Option<Rc<dyn Object>>) -> bool {
    if obj
        .as_ref()
        .is_some_and(|val| val.get_type() == object::ObjectType::ERROR)
    {
        return true;
    }
    return false;
}
