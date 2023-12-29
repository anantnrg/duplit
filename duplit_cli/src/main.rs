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
}

fn main() -> anyhow::Result<()> {
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
    }
}
