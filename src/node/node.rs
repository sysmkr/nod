pub struct Node {
    id: Option<u32>,
}

impl Node {
    pub fn new() -> Self { Self { id: None } }
    pub fn set_id(&mut self, id: u32) { self.id = Some(id); }
    pub fn get_id(&self) -> Option<u32> { self.id }
}

#[cfg(test)]
mod test_node_type {
    use super::*;

    #[test]
    fn test_new() {
        let new_instance = Node::new();
        assert_eq!(new_instance.get_id(), None);
    }
    
    #[test]
    fn test_set_id() {
        let mut new_instance = Node::new();
        new_instance.set_id(10);
        assert_eq!(new_instance.get_id(), Some(10));
    }
}
