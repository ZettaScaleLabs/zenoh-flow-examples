use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub key_expr: String,
    pub zenoh_config: zenoh::config::Config,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            key_expr: "zf/data/pub_sink".to_string(),
            zenoh_config: zenoh::config::peer(),
        }
    }
}
