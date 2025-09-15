mod buffer_reader;
mod decoder;
mod processor;
mod sc_file;
use std::io::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("> Starting SC File Parser");

    // TODO - Loop trough assets directory and search for *.sc files
    let current_dir = std::env::current_dir()?;
    let sc_file_path = current_dir.join("assets/background_clan_capital.sc");
    // let sc_file_path = current_dir.join("assets/background_clan_capital.sc");
    // let sc_file_path = current_dir.join("assets/buildings.sc");

    let mut sc_file = sc_file::ScFile::new(sc_file_path.to_string_lossy().into());
    sc_file.load().await?;

    let decoder = decoder::Decoder::new();

    // decoder.decode(&sc_file.buffer).await?;

    Ok(())
}
