pub mod model_client;
pub mod agent;

// Re-export main types for convenience
pub use model_client::{ModelClient, ModelResponse, LocalOllamaClient};
pub use agent::Agent; 