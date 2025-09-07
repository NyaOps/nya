use std::sync::Arc;

use crate::{core::{context::NyaContext, event_bus::NyaEventBus, schema::SchemaRegistry, service::Service}, core_services::nya_service::NyaService};

pub fn register_services() -> Vec<Box<dyn Service>> {
  vec![
    Box::new(NyaService),
  ]
}

pub fn register_schemas() -> SchemaRegistry { SchemaRegistry::new().unwrap() }

pub fn register_context(ctx_path: &str, bus: Arc<NyaEventBus>) -> Arc<NyaContext> {
  Arc::new(NyaContext::new(ctx_path, bus.clone()))
}