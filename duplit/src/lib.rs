use anyhow::anyhow;
use colored::*;

pub struct Duplit;

impl Duplit {
    pub fn config_path() -> anyhow::Result<std::path::PathBuf> {
        if let Some(home_dir) = std::env::var_os("HOME") {
            let home_path: std::path::PathBuf = home_dir.into();
            let config_path = home_path.join(".duplit");

            println!("{:?}", config_path);

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
                Ok(std::fs::create_dir(&config_path)?)
            } else {
                Err(anyhow!("Duplit config folder already exists!"))
            }
        } else {
            if config_path.exists() {
                std::fs::remove_dir_all(&config_path)?;
                Ok(std::fs::create_dir(&config_path)?)
            } else {
                Ok(std::fs::create_dir(&config_path)?)
            }
        }
    }
}
