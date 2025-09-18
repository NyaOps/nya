use std::sync::Arc;
use serde_json::Value;
use tokio::{sync::Mutex, task::JoinHandle};
use crate::core::{context::NyaContext, event_bus::{EventBus, NyaEventBus}, payload::Payload, schema::NyaSchema, service::Service};

struct NyaInternals {
  context: Arc<Mutex<NyaContext>>,
  schema: NyaSchema,
  bus: Arc<NyaEventBus>
}

#[derive(Clone)]
pub struct Nya {
  internals: Arc<NyaInternals>
}

impl Nya {
  pub fn build(cmd: &str, path: &str, reg: Vec<Box<dyn Service>>) -> Self {
    let nya_event_bus = build_nya_bus(reg);
    let ctx = NyaContext::new(path);
    let schema = NyaSchema::new(cmd);
      Self {
        internals: Arc::new(NyaInternals { context: Arc::new(Mutex::new(ctx)), schema: schema, bus: Arc::new(nya_event_bus) }
      )
    }
  }

  pub async fn run(&self, initial_payload: Payload) {
    for (i, step) in self.internals.schema.steps.iter().enumerate() {
        println!("\n Step {}/{}: {}", i + 1, self.internals.schema.steps.len(), step);
        let step_handle: JoinHandle<()> = self.internals.bus.clone().emit(self.clone(), step.clone(), initial_payload.clone()).await;
        _ = step_handle.await.map_err(|e| format!("Step '{}' failed: {:?}", step, e));
        println!("Step {} completed", i + 1);
    }
    println!("\n Execution completed successfully!");
  }

  pub async fn nya_ctx_get(&self, key: &str) -> Value {
    let ctx = self.internals.context.lock().await;
    if let Some(item) = ctx.context.get(key) {
      return item.clone()
    }
    return Value::Null;
  }
}

fn build_nya_bus(reg: Vec<Box<dyn Service>>) -> NyaEventBus {
  let mut nya_event_bus = NyaEventBus::new();
  let mut service_handlers = Vec::new();
  for service in reg.iter().clone() {
    service_handlers.extend(service.register());
  }
  for handler in service_handlers {
    nya_event_bus.on(handler.0, handler.1);
  }
  nya_event_bus
}