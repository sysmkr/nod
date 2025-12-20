use std::collections::HashMap;

pub struct Node {
    id: Option<u32>,
    storage: HashMap<String, String>,
}

impl Node {
    pub fn new() -> Self {
        Self { 
            id: None,
            storage: HashMap::new(),
        }
    }

    pub fn set_id(&mut self, id: u32) {
        self.id = Some(id);
    }

    pub fn get_id(&self) -> Option<u32> {
        self.id
    }

    pub fn get_storage(&mut self) -> &mut HashMap<String, String> {
        &mut self.storage
    }
}

#[cfg(test)]
mod structure {
    use super::*;

    #[test]
    fn new() {
        let new_instance = Node::new();
        assert_eq!(new_instance.get_id(), None);
    }
    
    #[test]
    fn set_id() {
        let mut new_instance = Node::new();
        new_instance.set_id(10);
        assert_eq!(new_instance.get_id(), Some(10));
    }
}
