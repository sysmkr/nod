use super::{ Node, StorageErr };

impl Node {
    pub fn erase(&mut self, key: &String) {
        self.get_storage().remove(key);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn erase() {
        let mut new_instance = Node::new();
        let key = String::from("test.txt");
        let value = String::from("Hello.");
        new_instance.store(&key, &value);
        assert_eq!(
            new_instance.request(&key),
            Ok(&value)
        );
        new_instance.erase(&key);
        assert_eq!(
            new_instance.request(&key),
            Err(StorageErr::NotFound)
        );
    }
}
