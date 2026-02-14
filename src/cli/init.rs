use std::fs;
use crate::{ embedded, utils::ConfigStatus };
use colored::*;
use crate::utils;

pub fn run(output_path: Option<String>) {
  let file_path = utils::resolve_base_config(output_path.as_deref());
  match file_path {
    ConfigStatus::Exists(path) => {
      println!("{}", "Cannot initialize Base Config, file already exists!".red());
      println!("Location: {}", path.display());
      println!("You can input a different path by running {}", "nya init -o your_path_here".purple());
      println!("Otherwise, remove the existing file first.");
    },
    ConfigStatus::Missing(path) => {
      if let Some(parent) = path.parent() {
          if let Err(e) = fs::create_dir_all(parent) {
              println!("Failed to create directories for {}: {}", path.display(), e);
              return;
          }
      }

      if let Err(e) = fs::write(&path, embedded::BASE_CONFIG_TEMPLATE) {
          println!("Failed to create config file at {}: {}", path.display(), e);
          return;
      }
        
      println!("{}", "Created Nya base config template".green());
      println!("Location: {}", path.display());
      println!();
      println!("{}", "Next steps:".cyan());
      println!("1. Edit the config file and fill in your infrastructure details");
      println!("2. Run: {}", "nya base build".bright_purple());
    }
  }
}