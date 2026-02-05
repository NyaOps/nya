use crate::{core::{payload::Payload, service::Service}, core_services::nya_core::get_core_services, runtime::nya::Nya};

pub async fn build() {
  let services: Vec<Box<dyn Service>> = get_core_services();
  let nya = Nya::build("base:build", "./context/nya_test_context.json", services);
  nya.run(Payload::empty()).await;
}