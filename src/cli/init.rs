use std::fs;
use std::path::Path;
use crate::embedded;
use colored::*;

pub fn run(output_path: String) -> Result<(), Box<dyn std::error::Error>> {
  let expanded_path = shellexpand::tilde(&output_path).to_string();
  let path = Path::new(&expanded_path);

  let file_path = if path.is_dir() {
      path.join("config.json")
  } else {
      path.to_path_buf()
  };

  if file_path.exists() {
      println!("{}", "Cannot initialize Base Config, file already exists!".red());
      println!("Location: {}", file_path.display());
      println!("You can input a different path by running {}", "nya init -o your_path_here".purple());
      println!("Otherwise, remove the existing file first.");
      return Ok(());
  }

  if let Some(parent) = file_path.parent() {
      fs::create_dir_all(parent)?;
  }

  fs::write(&file_path, embedded::BASE_CONFIG_TEMPLATE)?;
    
  println!("{}", "Created Nya base config template".green());
  println!("Location: {}", file_path.display());
  println!();
  println!("{}", "Next steps:".cyan());
  println!("1. Edit the config file and fill in your infrastructure details");
  println!("2. Run: {}", "nya base build".bright_purple());

  Ok(())
}