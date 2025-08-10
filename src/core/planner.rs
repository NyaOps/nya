// planner's functionality
//1) receives a command
//2) gets/receives a schema
//3) emits events
//4) ends the CLI

use std::sync::Arc;
use uuid::Uuid;

use crate::core::{event_bus::NyaEventBus, payload::Payload, schema::Schema};

pub struct Planner {
  event_bus: Arc<NyaEventBus>,
  trace_id: String,
  schemas: Vec<Schema>,
}

impl Planner {
  fn new(bus: Arc<NyaEventBus>, schemas: Vec<Schema>) -> Self {
    Self {
      event_bus: bus,
      trace_id: Uuid::new_v4().to_string(),
      schemas
    }
  }

  async fn execute_command(cmd: String, payload: Payload) {

  }
}