use anyhow::Result;
use async_trait::async_trait;

#[derive(Debug, Clone, PartialEq)]
pub struct Haiku {
    pub content: String,
}

#[async_trait]
pub trait HaikuGenerator {
    async fn generate(&self) -> Result<Haiku>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::automock;

    #[automock]
    #[async_trait]
    pub trait HaikuGenerator {
        async fn generate(&self) -> Result<Haiku>;
    }

    #[tokio::test]
    async fn test_haiku_generator_produces_content() {
        let mock_content = "Autumn moonlight falls\nA worm digs silently through\nThe chestnut flower";
        let mut mock_generator = MockHaikuGenerator::new();
        
        mock_generator
            .expect_generate()
            .times(1)
            .return_once(move || {
                Ok(Haiku {
                    content: mock_content.to_string(),
                })
            });

        let haiku = mock_generator.generate().await.unwrap();
        assert_eq!(haiku.content, mock_content);
    }
} 