use std::{env, fs, path::PathBuf};
use colored::*;
use inquire::Text;
use serde_json::{ json, Value };
use crate::utils;

// pub struct Capsule {
//     name: String,
//     created_at: String,
//     updated_at: String,
//     packs: Vec<Pack>,
// }

enum CreateNewCapsuleResult {
    Success(PathBuf),
    Error(String),
}

pub fn new(output_path: Option<String>) {
  let file_path = utils::resolve_capsule(output_path.as_deref());
  match file_path {
    Some(path) => {
      println!("{}", "Cannot create a new Capsule, a nya.json file already exists!".red());
      println!("Location: {}", path.display());
      println!("You can input a different path by running {}", "nya init -o your_path_here".purple());
      println!("Otherwise, remove the existing file first.");
    },
    None => {
      let result = create_new_capsule_file(output_path);
      match result {
        CreateNewCapsuleResult::Success(path) => {
          println!("Created new Capsule file at: {}", path.display());
        },
        CreateNewCapsuleResult::Error(error) => {
          eprintln!("{}", error);
        }
      }
    }
  }
}

fn create_new_capsule_file(output_path: Option<String>) -> CreateNewCapsuleResult {
  let name = Text::new("What do you want to call this capsule?")
      .prompt();
  let name = match name {
        Ok(name) => name.to_lowercase().replace(" ", "-"),
        Err(e) => return CreateNewCapsuleResult::Error(format!("Failed to read capsule name: {}", e)),
    };
  
  let capsule = json!({
      "capsule": {
          "name": name,
          "created_at": chrono::Utc::now().to_rfc3339(),
          "updated_at": chrono::Utc::now().to_rfc3339(),
          "packs": []
      }
  });
    
  let file_path = get_capsule_path(output_path);
  

  if let Some(parent) = file_path.parent() {
      fs::create_dir_all(parent).expect("Failed to create directories");
  }

  fs::write(&file_path, capsule.to_string()).expect("Failed to write capsule file");

  CreateNewCapsuleResult::Success(file_path)

}

pub fn check(path: Option<String>){
  let exists = utils::resolve_capsule(path.as_deref()).is_some();
  if exists {
    let current_capsule = read_capsule_file(path);
    println!("{}", "A Capsule file exists in this directory.".green());
    if let Some(capsule) = current_capsule {
      println!("Capsule name: {}", capsule["capsule"]["name"]);
    }
  } else {
    println!("{}", "No Capsule file found in this directory.".yellow());
  }
}

fn get_capsule_path(path: Option<String>) -> PathBuf {
  let mut file_path =PathBuf::from(if let Some(input) = path {
    let expanded = shellexpand::tilde(&input).to_string();
    expanded.into()
  } else {
    env::current_dir().unwrap()
  });

  file_path.push(".nya");
  file_path.push("nya.json");
  file_path
}

fn read_capsule_file(path: Option<String>) -> Option<Value> {
  let file_path = get_capsule_path(path);
  let content = fs::read_to_string(file_path).ok()?;
  serde_json::from_str(&content).ok()
}