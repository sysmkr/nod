use super::Node;

impl Node {
    pub fn request(&mut self, key: &String) -> Option<&String> {
        self.get_storage().get(key)
    }
}

#[cfg(test)]
mod request {
    use super::Node;
    
    #[test]
    fn test() {
        let mut new_instance = Node::new();
        let key = String::from("test.txt");
        let value = String::from("Hello.");
        new_instance.store(key.clone(), value.clone());
        assert_eq!(
            new_instance.request(&key),
            Some(String::from("Hello.")).as_ref()
        );
    }
}
