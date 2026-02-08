use crate::{core::{payload::Payload, service::Service, core_services::nya_core::get_core_services}, runtime::nya::Nya};

pub async fn build(config: String) {
  let services: Vec<Box<dyn Service>> = get_core_services();
  let nya = Nya::build("base:build", &config, services);
  nya.run(Payload::empty()).await;
}

pub async fn destroy(config: String) {
  let services: Vec<Box<dyn Service>> = get_core_services();
  let nya = Nya::build("base:destroy", &config, services);
  nya.run(Payload::empty()).await;
}