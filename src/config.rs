use config::{Config, ConfigError, Environment, File, FileFormat};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    always_hydrate: bool,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        Config::builder()
            .add_source(File::from_str(include_str!("config.default.toml"), FileFormat::Toml))
            .add_source(File::with_name("cheetah.toml").required(false))
            .add_source(Environment::with_prefix("CHEETAH"))
            .build()?
            .try_deserialize()
    }
}
