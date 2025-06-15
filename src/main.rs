mod cli;
mod configs;
mod pw_sound_controll;

use clap::Parser;
use cli::Cli;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    cli.match_commands()
}
