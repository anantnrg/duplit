use std::io::Write;

use clap::{Args, Parser, Subcommand, ValueEnum};
use colored::*;
use duplit::Duplit;

#[derive(Debug, Parser)]
#[command(name = "duplit")]
#[command(about = "CLI tool to create reproducible backups with Duplit.", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Init,
    InitForce,
    Create,
}

fn main() -> anyhow::Result<()> {
    let config = Duplit::fetch_config()?;
    let mut duplit = Duplit::new(config);
    let mut genconfig = duplit::GenConfig::new();

    let progress = |status: String| {
        print!("\r{}", status);
        std::io::stdout().flush().unwrap();
    };
    duplit.copy_configs(&mut genconfig, progress)?;
    let args = Cli::parse();

    match args.command {
        Commands::Init => match Duplit::init_config(false) {
            Ok(_) => {
                println!(
                    "{} {}",
                    "INFO:".green().bold(),
                    "Duplit config initialized successfully!"
                );
                Ok(())
            }
            Err(e) => Err(e),
        },
        Commands::InitForce => match Duplit::init_config(true) {
            Ok(_) => {
                println!(
                    "{} {}",
                    "INFO:".green().bold(),
                    "Duplit config initialized successfully!"
                );
                Ok(())
            }
            Err(e) => Err(e),
        },
        Commands::Create => match Duplit::get_pkgs() {
            Ok(pkgs) => Ok(()),
            Err(e) => Err(e),
        },
    }
}
