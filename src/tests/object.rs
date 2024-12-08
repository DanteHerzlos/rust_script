#[cfg(test)]
mod object_tests {
    use crate::object::*;

    #[test]
    fn test_string_hash_key() {
        let hello1 = StringObj {
            value: "Hello World".to_string(),
        };
        let hello2 = StringObj {
            value: "Hello World".to_string(),
        };
        let diff1 = StringObj {
            value: "My name is johnny".to_string(),
        };
        let diff2 = StringObj {
            value: "My name is johnny".to_string(),
        };

        assert_eq!(
            hello1.hash_key(),
            hello2.hash_key(),
            "strings with same content have different hash keys"
        );
        assert_eq!(
            diff1.hash_key(),
            diff2.hash_key(),
            "strings with same content have different hash keys"
        );

        assert_ne!(
            hello1.hash_key(),
            diff1.hash_key(),
            "strings with different content have same hash keys"
        );
    }
}
