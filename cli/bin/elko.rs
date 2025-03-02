//! Zink's package manager
#![deny(missing_docs)]

use clap::{Parser, Subcommand};
use color_eyre::Result;
use zinkup::{App, Build, New};

/// elko commands
#[derive(Debug, Subcommand)]
enum Command {
    New(New),
    Build(Build),
}

/// Zink's package manager
#[derive(Debug, Parser)]
#[command(name = "elko", version)]
pub struct Elko {
    #[command(subcommand)]
    command: Command,
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[clap(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

impl App for Elko {
    fn verbose(&self) -> u8 {
        self.verbose
    }

    fn run(&self) -> anyhow::Result<()> {
        match &self.command {
            Command::Build(build) => build.run(),
            Command::New(new) => new.run(),
        }
    }
}

/// The main function.
fn main() -> Result<()> {
    Elko::start()
}
