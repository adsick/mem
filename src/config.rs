use crate::{common::*, utils::expand_tilde};
use serde::{Deserialize, Serialize};
use std::{
    fs::read_to_string,
    io::Write,
    path::{Path, PathBuf},
};

// todo manual default impl
#[derive(Default, Serialize, Deserialize)]
pub struct Config {
    /// default path for documents, allows for running mem from any place
    #[serde(default)] //None if missing
    pub default_path: Option<PathBuf>,
}

impl Config {
    /// load the config using the default config path
    pub fn load() -> Result<Config> {
        let mut config_path = config_dir().unwrap(); // todo: add some fancy error handling
        config_path.push(Path::new("mem/config.toml"));

        dbg!(&config_path);

        Self::load_from(&config_path)
    }

    pub fn load_from(path: &Path) -> Result<Config> {
        let config = if let Ok(content) = read_to_string(&path) {
            let mut config: Config = toml::from_str(&content)?;
            println!("successfully loaded config from {path:?}");

            let dp = &mut config.default_path;

            if let Some(dp) = dp {
                println!("default path: {dp:?}");
                if let Some(new_path) = expand_tilde(&dp) {
                    *dp = new_path;
                }
            }

            // *dp = dp.as_ref().map_or(*dp, expand_tilde);

            config
        } else {
            std::fs::create_dir_all(path.parent().unwrap())?;
            std::fs::File::create(&path)?;
            Config::default()
        };
        Ok(config)
    }

    /// writes config to default location
    pub fn write(&self) -> Result<()> {
        let mut path = config_dir().unwrap();
        path.push(Path::new("mem/config.toml"));

        std::fs::create_dir_all(path.parent().unwrap())?;
        let mut file = std::fs::File::create(&path)?;

        file.write(toml::to_string_pretty(self)?.as_bytes())?;
        Ok(())
    }
}
