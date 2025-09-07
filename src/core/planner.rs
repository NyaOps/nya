use std::sync::Arc;
use tokio::task::JoinHandle;

use crate::core::{context::NyaContext, event_bus::{EventBus, NyaEventBus}, schema::Schema};

pub struct Planner {
  event_bus: Arc<NyaEventBus>,
  schema: Schema,
}

impl Planner {
  pub fn new(bus: Arc<NyaEventBus>, schema: Schema) -> Self {
    Self {
      event_bus: bus,
      schema
    }
  }

  pub async fn execute(&self, ctx: Arc<NyaContext>) -> Result<(), String>{
    
    for (i, step) in self.schema.steps.iter().enumerate() {
        println!("\n Step {}/{}: {}", i + 1, self.schema.steps.len(), step);
        
        let step_handle: JoinHandle<()> = self.event_bus.emit(step.clone(), ctx.clone()).await;
        step_handle.await.map_err(|e| format!("Step '{}' failed: {:?}", step, e))?;
        
        println!("Step {} completed", i + 1);
    }
    
    println!("\n Execution completed successfully!");
    Ok(())
  }
}

#[cfg(test)]
mod planner_tests{
  use std::sync::Arc;
  use serde_json::from_value;

use crate::core::{context::NyaContext, event_bus::{EventBus, NyaEventBus}, planner::Planner, schema::SchemaRegistry, service::{service_tests::TestService, Service}};

  fn get_test_schema_registry() -> SchemaRegistry { 
    SchemaRegistry::new().unwrap()
  }

  fn get_test_bus() -> Arc<NyaEventBus> {
    let mut bus= NyaEventBus::new();
    let svc = Box::new(TestService);
    _ = &bus.on("test_event".to_string(), svc.register()[0].1.clone());
    _ = &bus.on("test_event2".to_string(), svc.register()[1].1.clone());
    Arc::new(bus)
  }

  fn get_test_planner() -> Planner {
    let schema_registry = get_test_schema_registry();
    let bus = get_test_bus();
    let schema = schema_registry.get_schema("test_cmd").unwrap();
    Planner::new(bus, schema.clone())
  }

  #[tokio::test]
  async fn planner_can_execute_schema () -> Result<(), String> {
    let test_planner = get_test_planner();
    let test_ctx = Arc::new(NyaContext::new_create_bus("./context/nya_test_context.json"));
    {
      test_planner.execute(test_ctx.clone()).await?;
    }
    tokio::task::yield_now().await;
    let ctx = test_ctx.context.lock().unwrap();
    let ctx_val = ctx.get("test_key").unwrap();
    let ctx_val2 = ctx.get("test_key2").unwrap();
    let value: String = from_value(ctx_val.clone()).unwrap();
    let value2: String = from_value(ctx_val2.clone()).unwrap();
    assert_eq!(value, "test_value".to_string());
    assert_eq!(value2, "test_value2".to_string());

    Ok(())
  }
}