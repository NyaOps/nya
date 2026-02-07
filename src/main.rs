mod args;

use args::{Cli, Commands, BaseCommands};
use clap::Parser;
use nya::cli::base::build;

#[tokio::main]
async fn main() {
  let cli = Cli::parse();

  match cli.command {
      Commands::Base { command } => match command {
        BaseCommands::Build { context }=> { build::build(context).await },
        BaseCommands::Destroy { context }=> { build::build(context).await }
      }
  }
}