use colored::Colorize;

use crate::{core::runtime::Nya, utils::{ConfigStatus, resolve_base_config, resolve_capsule}};

pub async fn run(config: Option<String>, capsule: Option<String>) {
  let config_result = resolve_base_config(config.as_deref());
  let nya_base_config_path = match config_result {
    ConfigStatus::Exists(path) => path,
    ConfigStatus::Missing(path) => {
      println!("No config found at {}. Please create a config file to proceed.", path.display());
      return;
    }
  };

  let capsule_option = resolve_capsule(capsule.as_deref());
  let nya_capsule_path = match capsule_option {
    Some(path) => path,
    None => {
      println!("{}", "No capsule was found.".red());
      return;
    }
  };
  
  let nya_base_config_string = nya_base_config_path.display().to_string();
  let nya_capsule_string = nya_capsule_path.display().to_string();
  let context_file_path: Vec<&str> = vec![&nya_base_config_string, &nya_capsule_string];
  // TODO: these strings are still needed for the ship operation, 
  // we need to either be able to set them or the operation needs to derive the values
  Nya::run("capsule:ship", context_file_path).await;

}