use std::{collections::HashMap, fs::read_to_string
};
use std::path::PathBuf;
use serde_json::Value;
use tera::Map;

pub struct NyaContext {
  pub context: HashMap<String, Value>
}

impl NyaContext {
  pub fn new(config: PathBuf, capsule: Option<PathBuf>) -> Self {
      Self {
          context: build_context(config, capsule).expect("context load failed")
      }
  }
}

pub fn build_context(config: PathBuf, capsule: Option<PathBuf>) -> Result<HashMap<String, Value>, String> {
  let mut context: Map<String, Value> = Map::new();

  let paths: Vec<PathBuf> = {
    let mut v = vec![config];
    if let Some(c) = capsule {
      v.push(c);
    }
    v
  };

  for path in paths {
    let content = read_to_string(&path)
      .map_err(|e| format!("Failed to read context file '{}': {}", path.display(), e))?;
    let json: Value = serde_json::from_str(&content)
      .map_err(|e| format!("Failed to parse {}: {}", path.display(), e))?;

    if let Value::Object(map) = json {
      for (k, value) in map {
          context.insert(k, value);
      }
    }
  }
  Ok(context.into_iter().collect())
}

#[cfg(test)]
mod context_tests {
  use std::path::PathBuf;
  use crate::core::context::NyaContext;

  #[test]
  fn get_nya_context_returns_context() -> Result<(), String> {
    let path = PathBuf::from("./tests/nya_test_config.json");
    let nya_context = NyaContext::new(path, None);
  
    let test_value = nya_context.context.get("test")
      .and_then(|v| v.as_str())
      .ok_or("test1 not found or not a string")?;
    
    assert_eq!(test_value, "context_value");

    Ok(())
  }

  #[test]
  fn can_build_nya_context_from_multiple_locations() -> Result<(), String> {
    let config_path = PathBuf::from("./tests/nya_test_config.json");
    let capsule_path = PathBuf::from("./tests/nya_test_capsule.json");
    let nya_context = NyaContext::new(config_path, Some(capsule_path));
  
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