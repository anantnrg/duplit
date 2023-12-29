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
}

fn main() {}
