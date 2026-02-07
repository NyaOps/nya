use crate::{core::{payload::Payload, service::Service, core_services::nya_core::get_core_services}, runtime::nya::Nya};

pub async fn build(context: String) {
  let services: Vec<Box<dyn Service>> = get_core_services();
  let nya = Nya::build("base:build", &context, services);
  nya.run(Payload::empty()).await;
}

pub async fn destroy(context: String) {
  let services: Vec<Box<dyn Service>> = get_core_services();
  let nya = Nya::build("base:destroy", &context, services);
  nya.run(Payload::empty()).await;
}