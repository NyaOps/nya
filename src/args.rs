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
  Init {
    #[arg(short, long)]
    output: Option<String>,
  },

  Base {
    #[command(subcommand)]
    command: BaseCommands,
  },

  Capsule {
    #[command(subcommand)]
    command: CapsuleCommands,
  },

  Pack {
    #[command(subcommand)]
    command: PackCommands,
  },
  
  Ship {
    #[arg(short, long)]
    config: Option<String>,

    #[arg(short, long)]
    location: Option<String>,
  },
}

#[derive(Subcommand, Debug)]
pub enum BaseCommands {
  Build {
    #[arg(short, long)]
    config: Option<String>,
  },
  Destroy{
    #[arg(short, long)]
    config: Option<String>,
  },
}

#[derive(Subcommand, Debug)]
pub enum CapsuleCommands {
  New {
    #[arg(short, long)]
    config: Option<String>,
  },
}

#[derive(Subcommand, Debug)]
pub enum PackCommands {
  New {
    #[arg(short, long)]
    capsule: Option<String>,
  },
}