use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::{
    io::{BufRead, BufReader, Read, Write},
    path::PathBuf,
};

pub struct Duplit {
    pub config: Config,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Options {
    pub repository: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Configs {
    pub include: Vec<String>,
    pub exclude: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub options: Options,
    #[serde(rename = "configs")]
    pub configs: Configs,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Packages {
    pacman: Vec<String>,
    aur: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigLocations {
    name: String,
    out: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenConfig {
    pub packages: Packages,
    pub configs: Vec<ConfigLocations>,
}

impl Config {
    pub fn default() -> Config {
        Config {
            options: Options {
                repository: "https://github.com/<YOUR REPOSITORY URL>".to_string(),
            },
            configs: Configs {
                include: vec!["$HOME/.config".to_string()],
                exclude: vec!["$HOME/.config/pavucontrol.ini".to_string()],
            },
        }
    }
}

impl GenConfig {
    pub fn new() -> Self {
        GenConfig {
            configs: Vec::new(),
            packages: Packages {
                aur: Vec::new(),
                pacman: Vec::new(),
            },
        }
    }
}

impl Duplit {
    pub fn new(config: Config) -> Self {
        Duplit { config }
    }
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

    pub fn fetch_config() -> anyhow::Result<Config> {
        let config_path = Duplit::config_path()?.join("config.toml");
        let mut raw_data = String::new();
        std::fs::File::open(config_path)?.read_to_string(&mut raw_data)?;

        let toml: Config = toml::from_str(raw_data.as_str())?;

        Ok(toml)
    }

    pub fn get_pacman_pkgs() -> anyhow::Result<Vec<String>> {
        let mut pkgs_vec = Vec::new();
        let raw_output = std::process::Command::new("pacman")
            .arg("-Qqen")
            .output()
            .unwrap();
        if raw_output.status.success() {
            let stdout = raw_output.stdout;
            let reader = BufReader::new(stdout.as_slice());
            for line in reader.lines() {
                pkgs_vec.push(line?.to_string());
            }
        }
        Ok(pkgs_vec)
    }

    pub fn get_aur_pkgs() -> anyhow::Result<Vec<String>> {
        let mut pkgs_vec = Vec::new();
        let raw_output = std::process::Command::new("pacman")
            .arg("-Qqem")
            .output()
            .unwrap();
        if raw_output.status.success() {
            let stdout = raw_output.stdout;
            let reader = BufReader::new(stdout.as_slice());
            for line in reader.lines() {
                pkgs_vec.push(line?.to_string());
            }
        }
        Ok(pkgs_vec)
    }

    pub fn get_pkgs() -> anyhow::Result<Packages> {
        let pacman_pkgs = Duplit::get_pacman_pkgs()?;
        let aur_pkgs = Duplit::get_aur_pkgs()?;

        Ok(Packages {
            pacman: pacman_pkgs,
            aur: aur_pkgs,
        })
    }

    pub fn expand_path(path: &String) -> PathBuf {
        if path.contains("~") {
            return PathBuf::from(shellexpand::tilde(&path).to_string());
        } else if path.contains("$") {
            return PathBuf::from(shellexpand::env(&path).unwrap().to_string());
        } else {
            return PathBuf::from(path);
        }
    }

    pub fn copy_configs<F>(&mut self, gen_config: &mut GenConfig, status: F) -> anyhow::Result<()>
    where
        F: Fn(String),
    {
        let config_path = Duplit::config_path()?;
        let dest_path = config_path.join("configs");

        if !dest_path.exists() {
            std::fs::create_dir(&dest_path)?;
        }

        for path in &self.config.configs.include {
            let full_path = Duplit::expand_path(path);

            if let Ok(metadata) = std::fs::metadata(&full_path) {
                if metadata.is_file() {
                    if let Some(file_name) = &full_path.file_name() {
                        let options = fs_extra::file::CopyOptions::new();
                        let progress_handle = |process_info: fs_extra::file::TransitProcess| {
                            let percent =
                                (process_info.copied_bytes * 100) / process_info.total_bytes;
                            status(format!(
                                "Copying file \"{}\" to \"{}\": {}/{} ({}%) ",
                                full_path.to_str().unwrap(),
                                config_path
                                    .join(file_name.to_str().unwrap())
                                    .to_str()
                                    .unwrap(),
                                process_info.copied_bytes,
                                process_info.total_bytes,
                                percent
                            ))
                        };
                        gen_config.configs.push(ConfigLocations {
                            name: String::from(file_name.to_str().unwrap()),
                            out: String::from(full_path.to_str().unwrap()),
                        });
                        fs_extra::file::copy_with_progress(
                            &full_path,
                            dest_path.clone().join(file_name.to_str().unwrap()),
                            &options,
                            progress_handle,
                        )?;
                    }
                } else if metadata.is_dir() {
                    if let Some(dir_name) = full_path
                        .file_name()
                        .unwrap_or(std::ffi::OsStr::new(""))
                        .to_str()
                    {
                        let dest_dir = dest_path.clone().join(dir_name);
                        let options = fs_extra::dir::CopyOptions::new();
                        let progress_handle = |process_info: fs_extra::dir::TransitProcess| {
                            let percent =
                                (process_info.copied_bytes * 100) / process_info.total_bytes;
                            status(format!(
                                "Copying folder \"{}\" to \"{}\": {}/{} ({}%) ",
                                full_path.to_str().unwrap(),
                                config_path.join(dir_name).to_str().unwrap(),
                                process_info.copied_bytes,
                                process_info.total_bytes,
                                percent
                            ));
                            fs_extra::dir::TransitProcessResult::ContinueOrAbort
                        };
                        gen_config.configs.push(ConfigLocations {
                            name: String::from(dir_name),
                            out: String::from(full_path.to_str().unwrap()),
                        });
                        if !dest_dir.exists() {
                            std::fs::create_dir(&dest_dir)?;
                        }

                        let mut exclude_paths = Vec::new();

                        for exclude_path in &self.config.configs.exclude {
                            if exclude_path.contains(full_path.to_str().unwrap()) {
                                let path = exclude_path.replace(full_path.to_str().unwrap(), "");
                                exclude_paths.push(path);
                            }
                        }

                        println!("{:?}", exclude_paths);

                        fs_extra::dir::copy_with_progress(
                            &full_path,
                            dest_dir.clone(),
                            &options,
                            progress_handle,
                        )?;
                    }
                }
            }
        }

        Ok(())
    }
}
