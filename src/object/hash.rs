use super::*;

pub struct HashPair {
    pub key: Rc<dyn Object>,
    pub value: Rc<dyn Object>,
}

pub struct Hash {
    pub pairs: HashMap<HashKey, HashPair>,
}

impl Object for Hash {
    fn inspect(&self) -> String {
        let mut parirs = vec![];

        for (_, pair) in &self.pairs {
            parirs.push(format!("{}: {}", pair.key.inspect(), pair.value.inspect()))
        }

        format!("{{{}}}", parirs.join(", "))
    }
    fn get_type(&self) -> ObjectType {
        ObjectType::HASH
    }
    fn as_object(&self) -> &dyn Object {
        self
    }
    fn try_into_hash(&self) -> Result<&Hash, ErrorType> {
        Ok(self)
    }
}
