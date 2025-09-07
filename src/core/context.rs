use std::{
  collections::HashMap, 
  fs::read_to_string, sync::{Arc, Mutex}
};
use serde_json::Value;

use crate::core::event_bus::NyaEventBus;

type ExecutionContext = Arc<Mutex<HashMap<String, Value>>>;

pub struct NyaContext {
  pub context: ExecutionContext,
  pub bus: Arc<NyaEventBus>
}


impl NyaContext {
  pub fn new(path: &str, bus: Arc<NyaEventBus>) -> Self {
      Self {
          context: get_context(path).expect("context load failed"),
          bus,
      }
  }
  
  pub fn new_create_bus(path: &str) -> Self {
      let bus = Arc::new(NyaEventBus::new());
      Self {
          context: get_context(path).expect("context load failed"),
          bus,
      }
  }
}

pub fn get_context(path: &str) -> Result<ExecutionContext, String> {
  let content = read_to_string(path)
    .map_err(|e| format!("Failed to read context file '{}': {}", path, e))?; 
  let context: HashMap<String, Value> = serde_json::from_str(&content)
    .map_err(|e| format!("Failed to parse context: {}", e))?;
    Ok(to_async_context(context))
}

fn to_async_context(ctx: HashMap<String, Value>) -> ExecutionContext{
  Arc::new(Mutex::new(ctx))
}

#[cfg(test)]
mod context_tests {
    use crate::core::context::NyaContext;


    #[test]
    fn get_nya_context_returns_context() -> Result<(), String> {
      let nya_context = NyaContext::new_create_bus("./context/nya_test_context.json");
      let ctx = nya_context.context.lock().unwrap();
    
      let test_value = ctx.get("test1")
          .and_then(|v| v.as_str())
          .ok_or("test1 not found or not a string")?;
      
      assert_eq!(test_value, "value1");

      Ok(())
    }
}