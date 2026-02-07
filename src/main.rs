mod args;

use args::{Cli, Commands, BaseCommands};
use clap::Parser;
use nya::cli::base;

#[tokio::main]
async fn main() {
  let cli = Cli::parse();

  match cli.command {
      Commands::Base { command } => match command {
        BaseCommands::Build { context }=> { base::build(context).await },
        BaseCommands::Destroy { context }=> { base::destroy(context).await }
      }
  }
}