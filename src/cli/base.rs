use crate::core::runtime::Nya;
use crate::utils::{resolve_base_config, ConfigStatus};

pub async fn build(config: Option<String>) {
  let valid_path = resolve_base_config(config.as_deref());
  let path = match valid_path {
    ConfigStatus::Exists(path) => path,
    ConfigStatus::Missing(path) => {
      println!("No config found at {}. Please create a config file to proceed.", path.display());
      return;
    }
  };
  Nya::run("base:build", vec![&path.display().to_string()]).await;
}

pub async fn destroy(config: Option<String>) {
  let valid_path = resolve_base_config(config.as_deref());
  let path = match valid_path {
    ConfigStatus::Exists(path) => path,
    ConfigStatus::Missing(path) => {
      println!("No config found at {}. Please create a config file to proceed.", path.display());
      return;
    }
  };
  Nya::run("base:destroy", vec![&path.display().to_string()]).await;
}