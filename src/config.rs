use serde::Deserialize;
use std::{env, fs, path::Path};

#[derive(Clone, Deserialize)]
pub struct Config {
    pub location: Option<String>,
    pub units: Option<String>,
    pub apikey: Option<String>,
}

impl Config {
    pub fn read() -> Option<Self> {
        env::var("HOME").ok().and_then(|home| {
            let path = Path::new(&home).join(".config/touchgrass.toml");
            fs::read_to_string(path)
                .ok()
                .and_then(|contents| toml::from_str::<Self>(&contents).ok())
        })
    }
}
