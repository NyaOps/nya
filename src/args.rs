use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "nya")]
#[command(version = "preview-1")]
#[command(about = "Nya is framework that lets you build your own platform, anywhere you want.", long_about = None)]
pub struct Cli {
  #[command(subcommand)]
  pub command: Commands
}

#[derive(Subcommand, Debug)]
pub enum Commands {
  Base {
    #[command(subcommand)]
    command: BaseCommands,
  },
  // Capsule {
  //   #[command(subcommand)]
  //   command: CapsuleCommands,
  // },
  // Pack {
  //   #[command(subcommand)]
  //   command: PackCommands,
  // },
}

#[derive(Subcommand, Debug)]
pub enum BaseCommands {
  Build,
  Destroy
}