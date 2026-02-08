use clap::{Parser, Subcommand};

pub const BASE_CONFIG_DEFAULT_LOCATION: &str = "~/.nya/config.json";

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
  Init {
    #[arg(short, long, default_value = BASE_CONFIG_DEFAULT_LOCATION)]
    output: String,
  },

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
  Build {
    #[arg(short, long, default_value = BASE_CONFIG_DEFAULT_LOCATION)]
    config: String,
  },
  Destroy{
    #[arg(short, long, default_value = BASE_CONFIG_DEFAULT_LOCATION)]
    config: String,
  },
}