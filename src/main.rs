use std::sync::Arc;

use nya::core::{event_bus::{EventBus, NyaEventBus}, planner::Planner, registry::Registry, service::ServiceHandler};

#[tokio::main]
async fn main() {
  let mut nya_event_bus = NyaEventBus::new();
  let nya_registry = Registry::new();
  let mut service_handlers: Vec<ServiceHandler> = Vec::new();
  for service in nya_registry.services.iter().clone() {
    service_handlers.extend(service.register());
  }
  for handler in service_handlers {
    nya_event_bus.on(handler.0, handler.1);
  }
  let schema = nya_registry.schemas.get_schema("test_cmd2").unwrap();
  let nya_planner: Planner = Planner::new(Arc::new(nya_event_bus), schema.clone());
  // need to trigger planner 
  _ = nya_planner.execute(nya_registry.context).await;
}