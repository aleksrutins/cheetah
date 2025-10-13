use std::sync::Mutex;

use config::{Config, ConfigError, Environment, File, FileFormat};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::hooks::Hook;

lazy_static! {
    pub static ref SETTINGS: Mutex<Settings> = Mutex::new(Settings::new().unwrap());
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub always_hydrate: bool,
    pub hooks: Vec<Hook>,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        Config::builder()
            .add_source(File::from_str(
                include_str!("config.default.toml"),
                FileFormat::Toml,
            ))
            .add_source(File::with_name("cheetah.toml").required(false))
            .add_source(Environment::with_prefix("CHEETAH"))
            .build()?
            .try_deserialize()
    }
}
