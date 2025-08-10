use include_dir::{include_dir, Dir};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Embed entire schemas directory at compile time
static SCHEMAS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/schemas");

#[derive(Debug, Deserialize, Serialize)]
pub struct Schema {
    pub cmd: String,
    pub steps: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SchemaCollection {
    pub schemas: HashMap<String, Schema>,
}

pub struct SchemaRegistry {
    schemas: HashMap<String, Schema>,
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
    let mut all_schemas = HashMap::new();
    
    for file in SCHEMAS_DIR.files() {
        if file.path().extension() == Some("json".as_ref()) {
            let content = file.contents_utf8()
                .ok_or("Invalid UTF-8 in schema file")?;
            
            let schema_collection: SchemaCollection = serde_json::from_str(content)
                .map_err(|e| format!("Failed to parse {}: {}", file.path().display(), e))?;
            
            for (name, schema) in schema_collection.schemas {
                all_schemas.insert(name, schema);
            }
        }
    }
    
    Ok(all_schemas)
}