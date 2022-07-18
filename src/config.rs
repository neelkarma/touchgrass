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
        if let Ok(home) = env::var("HOME") {
            let path = Path::new(&home).join(".config/touchgrass.toml");
            if let Ok(contents) = fs::read_to_string(path) {
                if let Ok(parsed) = toml::from_str::<Self>(&contents) {
                    Some(parsed)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}
