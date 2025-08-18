use std::{
  collections::HashMap, 
  fs::read_to_string, sync::{Arc, Mutex}
};
use serde_json::Value;

type ExecutionContext = HashMap<String, Value>;
pub type NyaContext = Arc<Mutex<ExecutionContext>>;

pub fn get_context(path: &str) -> Result<NyaContext, String> {
  let content = read_to_string(path)
    .map_err(|e| format!("Failed to read context file '{}': {}", path, e))?; 
  let context: ExecutionContext = serde_json::from_str(&content)
    .map_err(|e| format!("Failed to parse context: {}", e))?;
    Ok(to_nya_context(context))
}

fn to_nya_context(ctx: ExecutionContext) -> NyaContext {
  Arc::new(Mutex::new(ctx))
}

#[cfg(test)]
mod context_tests {
    use crate::core::context::get_context;

    #[test]
    fn get_context_returns_context() -> Result<(), String> {
      let nya_context = get_context("./context/nya_test_context.json")?;
      let ctx = nya_context.lock().unwrap();
    
      let test_value = ctx.get("test1")
          .and_then(|v| v.as_str())
          .ok_or("test1 not found or not a string")?;
      
      assert_eq!(test_value, "value1");

      Ok(())
    }
}