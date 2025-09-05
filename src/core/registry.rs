use crate::{core::{context::{get_nya_context, NyaContext}, schema::SchemaRegistry, service::Service}, core_services::nya_service::NyaService};

pub struct Registry {
  pub services: Vec<Box<dyn Service>>,
  pub schemas: SchemaRegistry,
  pub context: NyaContext
}

impl Registry {
  pub fn new() -> Registry {
    Self {
      services: vec![
        Box::new(NyaService),
      ],
      schemas: SchemaRegistry::new().unwrap(),
      context: get_nya_context("./context/nya_test_context.json").unwrap()
    }
  }
}

#[cfg(test)]
mod registry_tests {
    use crate::core::registry::Registry;

  #[test]
  fn registry_initializes() {
    let registry = Registry::new();
    assert!(!registry.services.is_empty());
  }
}