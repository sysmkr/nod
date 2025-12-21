use super::*;


impl Node {
    pub fn request(&mut self, key: &String) -> Result<&String, StorageErr> {
        match self.get_storage().get(key) {
            Some(value) => Ok(value),
            None => Err(StorageErr::NotFound),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn request() -> Result<(), StorageErr> {
        let mut instance = Node::new();
        let key = String::from("test.txt");
        let value = String::from("Hello.");
        assert_eq!(
            instance.request(&key),
            Err(StorageErr::NotFound)
        );
        instance.store(key.clone(), value.clone())?;
        assert_eq!(
            instance.request(&key),
            Ok(&value)
        );
        Ok(())
    }
}
