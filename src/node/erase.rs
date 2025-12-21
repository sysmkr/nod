use super::Node;

impl Node {
    pub fn erase(&mut self, key: &String) {
        self.get_storage().remove(key);
    }
}

#[cfg(test)]
mod test {
    use super::Node;

    #[test]
    fn erase() {
        let mut new_instance = Node::new();
        let key = String::from("test.txt");
        let value = String::from("Hello.");
        new_instance.store(key.clone(), value.clone());
        assert_eq!(new_instance.request(&key), Some(value.clone()).as_ref());
        new_instance.erase(&key);
        assert_eq!(new_instance.request(&key), None);
    }
}
