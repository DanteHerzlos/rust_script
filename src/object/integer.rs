use super::*;

pub struct Integer {
    pub value: i64,
}

impl Object for Integer {
    fn inspect(&self) -> String {
        self.value.to_string()
    }
    fn get_type(&self) -> ObjectType {
        ObjectType::INTEGER
    }

    fn hash_key(&self) -> HashKey {
        HashKey {
            value: self.value as u64,
            object_type: self.get_type(),
        }
    }
    fn try_hash_key(&self) -> Result<HashKey, ErrorType>{
        Ok(self.hash_key())
    }
    fn as_int(&self) -> &Integer {
        self
    }
    fn as_bool(&self) -> &Boolean {
        if self.value == 0 {
            &Boolean { value: false }
        } else {
            &Boolean { value: true }
        }
    }
    fn as_object(&self) -> &dyn Object {
        self
    }
    fn try_into_int(&self) -> Result<&Integer, ErrorType> {
        Ok(self)
    }
}
