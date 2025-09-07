use std::sync::Arc;
use nya::core::{event_bus::{EventBus, NyaEventBus}, planner::Planner, registry::{register_context, register_schemas, register_services}};

#[tokio::main]
async fn main() {
  let mut nya_event_bus = NyaEventBus::new();
  let services = register_services();
  let mut service_handlers = Vec::new();
  for service in services.iter().clone() {
    service_handlers.extend(service.register());
  }
  for handler in service_handlers {
    nya_event_bus.on(handler.0, handler.1);
  }
  let arc_bus = Arc::new(nya_event_bus);
  let schema_registry = register_schemas();
  let schema = schema_registry.get_schema("test_cmd2").unwrap();
  let nya_planner: Planner = Planner::new(arc_bus.clone(), schema.clone());
  // need to trigger planner 
  _ = nya_planner.execute(register_context("./context/nya_test_context.json", arc_bus.clone())).await;
}