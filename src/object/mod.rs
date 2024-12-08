use std::{cell::RefCell, collections::HashMap, rc::Rc};

use core::fmt;

pub use array::Array;
pub use boolean::Boolean;
pub use builtin::Builtin;
pub use error::Error;
pub use function::Function;
pub use hash::Hash;
pub use integer::Integer;
pub use null::NULL;
pub use return_value::ReturnValue;
pub use string::StringObj;

pub mod array;
pub mod boolean;
pub mod builtin;
pub mod error;
pub mod function;
pub mod hash;
pub mod integer;
pub mod null;
pub mod return_value;
pub mod string;

#[derive(PartialEq, Debug, Eq, Clone, Hash)]
pub enum ObjectType {
    FUNCTION,
    BUILTIN,
    INTEGER,
    BOOLEAN,
    STRING,
    RETURN,
    ERROR,
    ARRAY,
    HASH,
    NULL,
}

impl ObjectType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ObjectType::FUNCTION => "FUNCTION",
            ObjectType::BUILTIN => "BUILTIN",
            ObjectType::BOOLEAN => "BOOLEAN",
            ObjectType::INTEGER => "INTEGER",
            ObjectType::RETURN => "RETURN",
            ObjectType::STRING => "STRING",
            ObjectType::ERROR => "ERROR",
            ObjectType::ARRAY => "ARRAY",
            ObjectType::HASH => "HASH",
            ObjectType::NULL => "NULL",
        }
    }
}

impl fmt::Display for ObjectType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, PartialEq, Hash, Eq)]
pub struct HashKey {
    value: u64,
    object_type: ObjectType,
}

type ErrorType = String;

pub trait Object {
    fn get_type(&self) -> ObjectType;
    fn inspect(&self) -> String;
    fn hash_key(&self) -> HashKey {
        panic!("can't get hash key for {}", self.get_type())
    }
    fn try_hash_key(&self) -> Result<HashKey, ErrorType>{
        Err(format!("can't get hash key for {}", self.get_type()))
    }
    fn as_int(&self) -> &Integer {
        panic!("can't cast {} to Integer", self.get_type())
    }
    fn as_bool(&self) -> &Boolean {
        panic!("can't cast {} to Boolean", self.get_type())
    }
    fn as_object(&self) -> &dyn Object;
    fn try_into_str(&self) -> Result<&StringObj, ErrorType> {
        Err(format!("can't cast from {} to String", self.get_type()))
    }
    fn try_into_int(&self) -> Result<&Integer, ErrorType> {
        Err(format!("can't cast from {} to Integer", self.get_type()))
    }
    fn try_into_bool(&self) -> Result<&Boolean, ErrorType> {
        Err(format!("can't cast from {} to Boolean", self.get_type()))
    }
    fn try_into_null(&self) -> Result<&NULL, ErrorType> {
        Err(format!("can't cast from {} to NULL", self.get_type()))
    }
    fn try_into_error(&self) -> Result<&Error, ErrorType> {
        Err(format!("can't cast from {} to Error", self.get_type()))
    }
    fn try_into_return_value(&self) -> Result<&ReturnValue, ErrorType> {
        Err(format!(
            "can't cast from {} to ReturnValue",
            self.get_type()
        ))
    }
    fn try_into_function(&self) -> Result<&Function, ErrorType> {
        Err(format!("can't cast from {} to Function", self.get_type()))
    }
    fn try_into_builtin(&self) -> Result<&Builtin, ErrorType> {
        Err(format!("can't cast from {} to Builtin", self.get_type()))
    }
    fn try_into_array(&self) -> Result<&Array, ErrorType> {
        Err(format!("can't cast from {} to String", self.get_type()))
    }
    fn try_into_hash(&self) -> Result<&Hash, ErrorType> {
        Err(format!("can't cast from {} to Hash", self.get_type()))
    }
}

pub struct Environment {
    store: HashMap<String, Rc<dyn Object>>,
    outer: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            store: HashMap::new(),
            outer: None,
        }
    }

    pub fn new_enclosed_env(outer: Rc<RefCell<Environment>>) -> Rc<RefCell<Environment>> {
        let mut env = Environment::new();
        env.outer = Some(outer);
        return Rc::new(RefCell::new(env));
    }

    pub fn get(&self, name: String) -> Result<Rc<dyn Object>, ErrorType> {
        match self.store.get(&name) {
            Some(val) => Ok(val.clone()),
            None => match &self.outer {
                Some(val) => val.borrow().get(name),
                None => Err("Element not exist in env".to_string()),
            },
        }
    }

    pub fn set(&mut self, name: String, val: Rc<dyn Object>) {
        self.store.insert(name, val);
    }
}

const HVAL_64_PRIME: u64 = 0x00000100000001b3;

pub fn get_fnv_a_hash(str: String) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;

    for char in str.as_bytes() {
        hash = hash ^ *char as u64;
        hash = hash & HVAL_64_PRIME;
    }

    hash
}
