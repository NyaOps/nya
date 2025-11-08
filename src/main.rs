use nya::{core::{payload::Payload, service::Service}, core_services::nya_core::get_core_services, runtime::nya::Nya};
//TODO: Pass in an initial Payload from cli
#[tokio::main]
async fn main() {
  let services: Vec<Box<dyn Service>> = get_core_services();
  let nya = Nya::build("build:base", "./context/nya_test_context.json", services);
  nya.run(Payload::empty()).await;
}