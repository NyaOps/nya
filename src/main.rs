use nya::{core::service::Service, core_services::nya_core::get_core_services, runtime::nya::Nya};

#[tokio::main]
async fn main() {
  let services: Vec<Box<dyn Service>> = get_core_services();
  let nya = Nya::build("test_cmd2", "./context/nya_test_context.json", services);
  nya.run().await;
}