use super::Node;
use std::time::Duration;

impl Node {
    pub async fn backup(&self) { 
        tokio::time::sleep(Duration::from_secs(2)).await;
    }
}

#[cfg(test)]
mod test {
    use super::Node;

    #[tokio::test]
    async fn backup() {
        let instance = Node::new();
        instance.backup().await;
    }
}
