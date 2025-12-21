use super::*;

impl Node {
    pub fn store(&mut self, key: &String, value: &String) -> Result<(), StorageErr> {
        if let Ok(_) = self.request(&key) {
            return Err(StorageErr::AlreadyExist);
        }
        self.get_storage().insert(key.clone(), value.clone());
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn store() -> Result<(), StorageErr> {
        let mut instance = Node::new();
        let key = String::from("test.txt");
        let value = String::from("Hello.");
        instance.store(&key, &value)?;
        assert_eq!(
            instance.get_storage().get(&key),
            Some(&value)
        );
        assert_eq!(
            instance.store(&key, &value),
            Err(StorageErr::AlreadyExist)
        );
        Ok(())
    }
}
