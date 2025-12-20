use super::Node;

impl Node {
    pub fn store(&mut self, key: String, value: String) {
        self.get_storage().insert(key, value);
    }
}

#[cfg(test)]
mod store {
    use super::Node;

    #[test]
    fn test() {
        let mut new_instance = Node::new();
        let test_key = String::from("test.txt");
        let test_value = String::from("Hello.");
        new_instance.store(test_key.clone(), test_value.clone());
        assert_eq!(
            new_instance.get_storage().get(&test_key), 
            Some(&test_value)
        );
    }
}
