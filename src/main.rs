#[macro_use]
extern crate dotenv_codegen;

mod bluesky;
mod warning;
use bluesky::post_to_bluesky;
use warning::latest_warning;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let warning = latest_warning().await?;

    // TODO: add check for last warning
    if warning != "No current tornado warnings".to_string() {
        post_to_bluesky(warning).await?;
    }

    Ok(())
}
