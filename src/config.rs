use crate::common::*;
use serde::{Deserialize, Serialize};
use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
};

#[derive(Serialize, Deserialize)]
pub struct Config {
    /// default path for documents, allows for running mem from any place
    pub default_path: Option<PathBuf>,
}

impl Config {
    /// load the config using the default config path
    pub fn load() -> Result<Config> {
        let mut dir = config_dir().unwrap(); // todo: add some fancy error handling
        dir.push(Path::new("mem/"));

        dbg!(&dir);
        
        let content = read_to_string(dir)?;
        let config = toml::from_str(&content)?;
        Ok(config)
    }
    pub fn load_from(path: &Path) -> Result<Config> {
        let content = read_to_string(path)?;
        let config = toml::from_str(&content)?;
        Ok(config)
    }
    
}
