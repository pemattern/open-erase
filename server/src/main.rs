use server::bootstrap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    bootstrap().await?;
    Ok(())
}
