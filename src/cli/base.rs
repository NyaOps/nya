use crate::{
  core::{
    core_services::nya_core::get_core_services, payload::Payload, service::Service
  }, 
  runtime::nya::Nya,
  utils::{self, ConfigStatus}
};

pub async fn build(config: Option<String>) {
  let valid_path = utils::resolve_base_config(config.as_deref());
  let path = match valid_path {
    ConfigStatus::Exists(path) => path,
    ConfigStatus::Missing(path) => {
      println!("No config found at {}. Please create a config file to proceed.", path.display());
      return;
    }
  };
  let services: Vec<Box<dyn Service>> = get_core_services();
  let nya = Nya::build("base:build", vec![&path.display().to_string()], services);
  nya.run(Payload::empty()).await;
}

pub async fn destroy(config: Option<String>) {
  let valid_path = utils::resolve_base_config(config.as_deref());
  let path = match valid_path {
    ConfigStatus::Exists(path) => path,
    ConfigStatus::Missing(path) => {
      println!("No config found at {}. Please create a config file to proceed.", path.display());
      return;
    }
  };
  let services: Vec<Box<dyn Service>> = get_core_services();
  let nya = Nya::build("base:destroy", vec![&path.display().to_string()], services);
  nya.run(Payload::empty()).await;
}