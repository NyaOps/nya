use std::{
  collections::HashMap, 
  fs::read_to_string
};
use serde_json::Value;

pub struct NyaContext {
  pub context: HashMap<String, Value>
}

impl NyaContext {
  pub fn new(path: &str) -> Self {
      Self {
          context: get_context(path).expect("context load failed")
      }
  }
}

pub fn get_context(path: &str) -> Result<HashMap<String, Value>, String> {
  let content = read_to_string(path)
    .map_err(|e| format!("Failed to read context file '{}': {}", path, e))?; 
  let context: HashMap<String, Value> = serde_json::from_str(&content)
    .map_err(|e| format!("Failed to parse context: {}", e))?;
  Ok(context)
}

#[cfg(test)]
mod context_tests {
  use crate::core::context::NyaContext;

  #[test]
  fn get_nya_context_returns_context() -> Result<(), String> {
    let nya_context = NyaContext::new("./tests/context/nya_test_context.json");
  
    let test_value = nya_context.context.get("test")
      .and_then(|v| v.as_str())
      .ok_or("test1 not found or not a string")?;
    
    assert_eq!(test_value, "context_value");

    Ok(())
  }
}