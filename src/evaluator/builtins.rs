use super::*;

pub const BUILTINS: LazyLock<HashMap<String, Rc<Builtin>>> = LazyLock::new(|| {
    HashMap::from([
        (
            "len".to_string(),
            Rc::new(object::Builtin {
                function: len_builtin_fn,
            }),
        ),
        (
            "first".to_string(),
            Rc::new(object::Builtin {
                function: first_builtin_fn,
            }),
        ),
        (
            "last".to_string(),
            Rc::new(object::Builtin {
                function: last_builtin_fn,
            }),
        ),
        (
            "rest".to_string(),
            Rc::new(object::Builtin {
                function: rest_builtin_fn,
            }),
        ),
        (
            "push".to_string(),
            Rc::new(object::Builtin {
                function: push_builtin_fn,
            }),
        ),
        (
            "puts".to_string(),
            Rc::new(object::Builtin {
                function: puts_builtin_fn,
            }),
        ),
    ])
});

fn len_builtin_fn(objects: Vec<Rc<dyn Object>>) -> Rc<dyn Object> {
    if objects.len() != 1 {
        return Rc::new(new_error(format!(
            "wrong number of arguments. got={}, want=1",
            objects.len()
        )));
    }

    match objects[0].get_type() {
        ObjectType::STRING => Rc::new(Integer {
            value: objects[0].try_into_str().unwrap().value.len() as i64,
        }),
        ObjectType::ARRAY => Rc::new(Integer {
            value: objects[0].try_into_array().unwrap().elements.len() as i64,
        }),
        _ => {
            return Rc::new(new_error(format!(
                "argument to `len` not supported, got {}",
                objects[0].get_type()
            )))
        }
    }
}

fn first_builtin_fn(objects: Vec<Rc<dyn Object>>) -> Rc<dyn Object> {
    if objects.len() != 1 {
        return Rc::new(new_error(format!(
            "wrong number of arguments. got={}, want=1",
            objects.len()
        )));
    }

    if objects[0].get_type() != ObjectType::ARRAY {
        return Rc::new(new_error(format!(
            "argument to `first` must be ARRAY, got={}",
            objects[0].get_type()
        )));
    }

    let arr = objects[0].try_into_array().unwrap();
    if arr.elements.len() > 0 {
        return arr.elements[0].clone();
    }

    return Rc::new(NULL);
}

fn last_builtin_fn(objects: Vec<Rc<dyn Object>>) -> Rc<dyn Object> {
    if objects.len() != 1 {
        return Rc::new(new_error(format!(
            "wrong number of arguments. got={}, want=1",
            objects.len()
        )));
    }

    if objects[0].get_type() != ObjectType::ARRAY {
        return Rc::new(new_error(format!(
            "argument to `first` must be ARRAY, got={}",
            objects[0].get_type()
        )));
    }

    let arr = objects[0].try_into_array().unwrap();
    if arr.elements.len() > 0 {
        return arr.elements.last().unwrap().clone();
    }

    return Rc::new(NULL);
}

fn rest_builtin_fn(objects: Vec<Rc<dyn Object>>) -> Rc<dyn Object> {
    if objects.len() != 1 {
        return Rc::new(new_error(format!(
            "wrong number of arguments. got={}, want=1",
            objects.len()
        )));
    }

    if objects[0].get_type() != ObjectType::ARRAY {
        return Rc::new(new_error(format!(
            "argument to `first` must be ARRAY, got={}",
            objects[0].get_type()
        )));
    }

    let arr = objects[0].try_into_array().unwrap();
    let length = arr.elements.len();
    if length > 0 {
        let new_elements = arr.elements[1..length].to_vec();
        return Rc::new(object::Array {
            elements: new_elements,
        });
    }

    return Rc::new(NULL);
}

fn push_builtin_fn(objects: Vec<Rc<dyn Object>>) -> Rc<dyn Object> {
    if objects.len() != 2 {
        return Rc::new(new_error(format!(
            "wrong number of arguments. got={}, want=2",
            objects.len()
        )));
    }

    if objects[0].get_type() != ObjectType::ARRAY {
        return Rc::new(new_error(format!(
            "argument to `first` must be ARRAY, got={}",
            objects[0].get_type()
        )));
    }

    let arr = objects[0].try_into_array().unwrap();
    let mut new_elements = arr.elements.to_vec();

    new_elements.push(objects[1].clone());

    Rc::new(object::Array {
        elements: new_elements,
    })
}

fn puts_builtin_fn(objects: Vec<Rc<dyn Object>>) -> Rc<dyn Object> {
    for obj in objects {
        println!("{}", obj.inspect())
    }
    Rc::new(object::NULL {})
}
