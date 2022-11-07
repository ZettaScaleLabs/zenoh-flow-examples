use clap::Parser;
use opencv::{highgui, prelude::*};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use zenoh::prelude::sync::*;
use zenoh_buffers::traits::SplitBuffer;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    config: PathBuf,
}

#[derive(Serialize, Deserialize)]
struct Config {
    zenoh_config: zenoh::config::Config,
    key_expr: String,
}

impl Config {
    fn load(path: PathBuf) -> Result<Self> {
        let config: Config = serde_yaml::from_str(&std::fs::read_to_string(path)?)?;
        Ok(config)
    }
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Result<T, E = Error> = std::result::Result<T, E>;

#[async_std::main]
async fn main() -> Result<()> {
    let Args { config } = Args::parse();
    let config = Config::load(config)?;

    async_std::task::spawn_blocking(move || -> Result<_> {
        let session = zenoh::open(config.zenoh_config).res()?;
        let subscriber = session.declare_subscriber(&config.key_expr).res()?;
        while let Ok(data) = subscriber.recv() {
            let decoded = opencv::imgcodecs::imdecode(
                &opencv::types::VectorOfu8::from_slice(&data.payload.contiguous()),
                opencv::imgcodecs::IMREAD_COLOR,
            )?;
            if decoded.size().unwrap().width > 0 {
                highgui::imshow("Window", &decoded)?;
            }
            highgui::wait_key(10)?;
        }
        Ok(())
    })
    .await?;

    Ok(())
}
