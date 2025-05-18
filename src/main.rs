pub mod agent;
pub mod model_client;

#[cfg(test)]
mod tests {}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    Ok(())
}
