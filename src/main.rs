mod args;

use args::{Cli, Commands, BaseCommands};
use clap::Parser;
use nya::cli::{init, base};

#[tokio::main]
async fn main() {
  let cli = Cli::parse();

  match cli.command {
    Commands::Init { output } => { init::run(output).unwrap() },
    Commands::Base { command } => match command {
      BaseCommands::Build { config }=> { base::build(config).await },
      BaseCommands::Destroy { config }=> { base::destroy(config).await }
    }
  }
}