mod args;

use args::{Cli, Commands, BaseCommands};
use clap::Parser;
use nya::cli::{
  init, 
  base, 
  capsule,
  pack
};

use crate::args::CapsuleCommands;

#[tokio::main]
async fn main() {
  let cli = Cli::parse();

  match cli.command {
    Commands::Init { output } => { init::run(output) },
    Commands::Base { command } => match command {
      BaseCommands::Build { config }=> { base::build(config).await },
      BaseCommands::Destroy { config }=> { base::destroy(config).await }
    },
    Commands::Capsule { command } => match command {
      CapsuleCommands::New { config } => { capsule::new(config) },
      CapsuleCommands::Check { config } => { capsule::check(config) }
      CapsuleCommands::List { config } => {  }
    },
    Commands::Pack { command } => match command {
      args::PackCommands::New { capsule } => { pack::new(capsule) },
    }
  }
}