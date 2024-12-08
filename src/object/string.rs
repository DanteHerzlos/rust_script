use super::*;

pub struct StringObj {
    pub value: String,
}

impl Object for StringObj {
    fn inspect(&self) -> String {
        self.value.clone()
    }
    fn get_type(&self) -> ObjectType {
        ObjectType::STRING
    }
    fn hash_key(&self) -> HashKey {
        HashKey {
            value: get_fnv_a_hash(self.value.clone()),
            object_type: self.get_type(),
        }
    }
    fn try_hash_key(&self) -> Result<HashKey, ErrorType>{
        Ok(self.hash_key())
    }
    fn as_object(&self) -> &dyn Object {
        self
    }
    fn try_into_str(&self) -> Result<&StringObj, ErrorType> {
        Ok(self)
    }
}
