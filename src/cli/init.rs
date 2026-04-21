use std::fs;
use std::path::PathBuf;
use crate::utils::ConfigStatus;
use colored::*;
use crate::utils;

pub fn run(user_input: Option<PathBuf>) {
  let path_result = utils::verify_base_config(user_input);
  match path_result {
    ConfigStatus::Exists(path) => {
      println!("{}", "Cannot initialize Base Config, file already exists!".red());
      println!("Location: {}", path.display());
      println!("You can input a different path by running {}", "nya init -o your_path_here".purple());
      println!("Otherwise, remove the existing file first.");
    },
    ConfigStatus::Missing(result) => {
      if let Some(parent) = result.0.parent() {
          if let Err(e) = fs::create_dir_all(parent) {
              println!("Failed to create directories for {}: {}", result.0.display(), e);
              return;
          }
      }

      if let Err(e) = fs::write(&result.0, include_str!("../../src/ops/init/initial_config.json")) {
          println!("Failed to create config file at {}: {}", result.0.display(), e);
          return;
      }
        
      println!("{}", "Created Nya base config template".green());
      println!("Location: {}", result.0.display());
      println!();
      println!("{}", "Next steps:".cyan());
      println!("1. Edit the config file and fill in your infrastructure details");
      println!("2. Run: {}", "nya base build".bright_purple());
    }
  }
}