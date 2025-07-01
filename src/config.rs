use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub udp_bind: String,
    pub rest_bind: String,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Self> {
        let settings = config::Config::builder()
            .add_source(config::File::with_name(path))
            .build()?;
        settings.try_deserialize().map_err(Into::into)
    }
}
