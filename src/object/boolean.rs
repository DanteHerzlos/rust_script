use super::*;

pub struct Boolean {
    pub value: bool,
}

impl Object for Boolean {
    fn inspect(&self) -> String {
        self.value.to_string()
    }
    fn hash_key(&self) -> HashKey {
        let value = if self.value { 1 } else { 0 };

        HashKey {
            value,
            object_type: self.get_type(),
        }
    }
    fn try_hash_key(&self) -> Result<HashKey, ErrorType>{
        Ok(self.hash_key())
    }
    fn get_type(&self) -> ObjectType {
        ObjectType::BOOLEAN
    }
    fn as_int(&self) -> &Integer {
        match self.value {
            true => &Integer { value: 1 },
            false => &Integer { value: 0 },
        }
    }
    fn as_bool(&self) -> &Boolean {
        self
    }
    fn as_object(&self) -> &dyn Object {
        self
    }
    fn try_into_bool(&self) -> Result<&Boolean, ErrorType> {
        Ok(self)
    }
}
