use dotenv::dotenv;

mod tts_manager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    tts_manager::speak("Hello from main!").await?;

    Ok(())
}
