use super::{ Node, StorageErr };

impl Node {
    pub fn erase(&mut self, key: &String) -> Result<(), StorageErr> {
        if let None = self.get_storage().remove(key) {
            return Err(StorageErr::NotFound);
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn erase() -> Result<(), StorageErr> {
        let mut new_instance = Node::new();
        let key = String::from("test.txt");
        let value = String::from("Hello.");
        new_instance.store(&key, &value)?;
        assert_eq!(
            new_instance.request(&key),
            Ok(&value)
        );
        new_instance.erase(&key)?;
        assert_eq!(
            new_instance.request(&key),
            Err(StorageErr::NotFound)
        );
        Ok(())
    }
}
