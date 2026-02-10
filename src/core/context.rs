use std::{collections::HashMap, fs::read_to_string
};
use serde_json::Value;
use tera::Map;

pub struct NyaContext {
  pub context: HashMap<String, Value>
}

impl NyaContext {
  pub fn new(file_paths: Vec<&str>) -> Self {
      Self {
          context: build_context(file_paths).expect("context load failed")
      }
  }
}

pub fn build_context(paths: Vec<&str>) -> Result<HashMap<String, Value>, String> {
  let mut context: Map<String, Value> = Map::new();
  for path in paths {
    let content = read_to_string(path)
      .map_err(|e| format!("Failed to read context file '{}': {}", path, e))?; 
    let json: Value = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse {}: {}", path, e))?;
    
    // Merge top-level keys
    if let Value::Object(map) = json {
        for (key, value) in map {
            context.insert(key, value);
        }
    }
  }

  Ok(context.into_iter().collect())
}

#[cfg(test)]
mod context_tests {
  use crate::core::context::NyaContext;

  #[test]
  fn get_nya_context_returns_context() -> Result<(), String> {
    let nya_context = NyaContext::new(vec!["./tests/nya_test_config.json"]);
  
    let test_value = nya_context.context.get("test")
      .and_then(|v| v.as_str())
      .ok_or("test1 not found or not a string")?;
    
    assert_eq!(test_value, "context_value");

    Ok(())
  }

  #[test]
  fn can_build_nya_context_from_multiple_locations() -> Result<(), String> {
    let file_paths = vec![
      "./tests/nya_test_config.json", 
      "./tests/nya_test_capsule.json"
    ];

    let nya_context = NyaContext::new(file_paths);
  
    let test_value = nya_context.context.get("test")
      .and_then(|v| v.as_str())
      .ok_or("test1 not found or not a string")?;

    let test_value2 = nya_context.context.get("capsule_name")
      .and_then(|v| v.as_str())
      .ok_or("test1 not found or not a string")?;
    
    assert_eq!(test_value, "context_value");
    assert_eq!(test_value2, "my_capsule");

    Ok(())
  }
}