use include_dir::{include_dir, Dir};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Embed entire schemas directory at compile time
static SCHEMAS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/schemas");

#[derive(Debug, Deserialize, Serialize)]
pub struct Schema {
    pub steps: Vec<String>,
}

type SchemaCollection = HashMap<String, Schema>;

pub struct SchemaRegistry {
  schemas: SchemaCollection,
}

impl SchemaRegistry {
    pub fn new() -> Result<Self, String> {
        let core_schemas = get_core_schemas()?;
        Ok(Self {
            schemas: core_schemas,
        })
    }
    
    pub fn get_schema(&self, name: &str) -> Option<&Schema> {
        self.schemas.get(name)
    }
    
    pub fn list_schemas(&self) -> Vec<&String> {
        self.schemas.keys().collect()
    }
}

fn get_core_schemas() -> Result<HashMap<String, Schema>, String> {
    let mut all_schemas: SchemaCollection = SchemaCollection::new();
    
    for file in SCHEMAS_DIR.files() {
        if file.path().extension() == Some("json".as_ref()) {
            let content = file.contents_utf8()
                .ok_or("Invalid UTF-8 in schema file")?;
            
            let schema_collection: SchemaCollection = serde_json::from_str(content)
                .map_err(|e| format!("Failed to parse {}: {}", file.path().display(), e))?;
            
            for (name, schema) in schema_collection {
                all_schemas.insert(name, schema);
            }
        }
    }
    Ok(all_schemas)
}

#[cfg(test)]
mod schema_tests {
    use crate::core::schema::SchemaRegistry;

    #[test]
    fn schema_registry_lists_current_schemas() -> Result<(), String> {
      let registry: SchemaRegistry = SchemaRegistry::new()?;
      let schema_names: Vec<&String> = registry.list_schemas();
      let found:bool = schema_names.iter().any(|name| *name == "test_cmd_1");
      assert!(found, "test_cmd_1 schema should exist");
      assert!(schema_names.len() > 0, "Should have at least one schema");
      Ok(())
    }
    
    #[test]
    fn can_get_specific_schema() -> Result<(), String> {
        let registry = SchemaRegistry::new()?;
        let schema = registry.get_schema("test_cmd_1");
        assert!(schema.is_some(), "test_cmd_1 schema should exist");
        if let Some(schema) = schema {
            assert_eq!(schema.steps.len(), 3);
            assert_eq!(schema.steps[0], "test_event_1");
        }
        Ok(())
    }
    
    #[test]
    fn returns_none_for_nonexistent_schema() -> Result<(), String> {
        let registry = SchemaRegistry::new()?;
        let schema = registry.get_schema("nonexistent");
        assert!(schema.is_none());
        Ok(())
    }
}