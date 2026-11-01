use anyhow::Result;
use clap::Parser;
use shrinker::cli::{self, Cli};

fn main() -> Result<()> {
    let cli = Cli::parse();
    cli::run(cli)
}
