use std::{error::Error, path::PathBuf};

use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct DiscordBuildInfo {
    pub(crate) release_channel: String,
    pub(crate) version: String,
}

/// Hacky Discord updater 😎
#[derive(Parser, Debug)]
#[command(author, about)]
struct Args {
    version: String,
    #[arg(short, long, default_value = "/opt/discord/resources/build_info.json")]
    path: PathBuf,
}
fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let build_info_json = match std::fs::read_to_string(&args.path) {
        Ok(info) => info,
        Err(err) => {
            return Err(format!(
                "Error while reading {file}: {err:?}",
                file = args.path.display()
            )
            .into())
        }
    };

    let mut build_info: DiscordBuildInfo = serde_json::from_str(&build_info_json).unwrap();

    build_info.version = args.version;

    sudo::escalate_if_needed()?;

    std::fs::write(&args.path, serde_json::to_string_pretty(&build_info)?)?;

    Ok(())
}
