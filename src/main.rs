#[macro_use]
extern crate dotenv_codegen;

mod bluesky;
mod warning;
use bluesky::post_to_bluesky;
use warning::latest_warning;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let warning = latest_warning().await?;

    let stored_warning = std::fs::read_to_string("warning").expect("Error reading file");

    if warning.clone() != stored_warning.clone() {
        std::fs::write("warning", warning.clone()).expect("Error writing to file");
        if warning.clone() != "No current tornado warnings".to_string() {
            post_to_bluesky(warning.clone()).await?;
        }
    }

    Ok(())
}
