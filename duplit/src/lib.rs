use std::io::Write;

use anyhow::anyhow;
use serde::{Deserialize, Serialize};

pub struct Duplit;

#[derive(Debug, Serialize, Deserialize)]
struct Options {
    repository: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Configs {
    include: Vec<String>,
    exclude: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    options: Options,
    #[serde(rename = "configs")]
    config_files: Configs,
}

impl Config {
    pub fn default() -> Config {
        Config {
            options: Options {
                repository: "https://github.com/<YOUR REPOSITORY URL>".to_string(),
            },
            config_files: Configs {
                include: vec!["$HOME/.config".to_string()],
                exclude: vec!["$HOME/.config/pavucontrol.ini".to_string()],
            },
        }
    }
}

impl Duplit {
    pub fn config_path() -> anyhow::Result<std::path::PathBuf> {
        if let Some(home_dir) = std::env::var_os("HOME") {
            let home_path: std::path::PathBuf = home_dir.into();
            let config_path = home_path.join(".duplit");

            return Ok(config_path);
        } else {
            Err(anyhow!(
                "Home directory not found or $HOME variable is not set"
            ))
        }
    }

    pub fn init_config(force: bool) -> anyhow::Result<()> {
        let config_path = Duplit::config_path()?;
        if !force {
            if !config_path.exists() {
                std::fs::create_dir(&config_path)?;
                let default_config = Config::default();
                let default_string = toml::to_string(&default_config)?;

                let mut file = std::fs::File::create(config_path.join("config.toml"))?;
                Ok(file.write_all(default_string.as_bytes())?)
            } else {
                Err(anyhow!("Duplit config folder already exists!"))
            }
        } else {
            if config_path.exists() {
                std::fs::remove_dir_all(&config_path)?;
                std::fs::create_dir(&config_path)?;
                let default_config = Config::default();
                let default_string = toml::to_string(&default_config)?;

                let mut file = std::fs::File::create(config_path.join("config.toml"))?;
                Ok(file.write_all(default_string.as_bytes())?)
            } else {
                std::fs::create_dir(&config_path)?;
                let default_config = Config::default();
                let default_string = toml::to_string(&default_config)?;

                let mut file = std::fs::File::create(config_path.join("config.toml"))?;
                Ok(file.write_all(default_string.as_bytes())?)
            }
        }
    }
}
