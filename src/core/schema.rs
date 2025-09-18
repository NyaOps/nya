use include_dir::{include_dir, Dir};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Embed entire schemas directory at compile time
static SCHEMAS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/schemas");

type SchemaCollection = HashMap<String, NyaSchema>;
type NyaSchemaSteps = Vec<String>;
// type NyaSchemaInitialPayload

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NyaSchema {
    pub steps: NyaSchemaSteps,
    // pub initial_payload: HashMap<String, Value>
}

impl NyaSchema {
    pub fn new(cmd: &str) -> Self {
        match get_schema(cmd) {
            Ok(schema) => Self {
                steps: schema,
            },
            Err(err) => { panic!("{}", err)}
        }
    }
    
    //TODO: create fn to return schema collection
    // pub fn list_schemas(&self) -> Vec<&String> {
    //     self.schemas.keys().collect()
    // }
}

fn get_schema(cmd: &str) -> Result<NyaSchemaSteps, String> {
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
    if let Some(schema) = all_schemas.get(cmd).clone() {
        return Ok(schema.clone().steps);
    }
    Err("Get Schema(): wasn't able to successfully retrieve schema".to_string())
}

#[cfg(test)]
mod schema_tests {
    use crate::core::schema::NyaSchema;

    #[test]
    fn can_get_schema() {
        let found = NyaSchema::new("test_cmd");
        let steps_len: usize = 2;
        assert_eq!(found.steps.len(), steps_len);
    }
    
    #[test]
    #[should_panic]
    fn panics_for_nonexistent_schema() {
        _ = NyaSchema::new("nonexistent");
    }
}