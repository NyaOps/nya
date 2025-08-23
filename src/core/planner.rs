// // planner's functionality
// //1) gets/receives a schema and bus
// //2) emits events
// //3) ends the CLI

// use std::sync::Arc;
// use tokio::task::JoinHandle;
// use uuid::Uuid;

// use crate::core::{event_bus::{EventBus, NyaEventBus}, payload::Payload, schema::Schema};

// pub struct Planner {
//   event_bus: Arc<NyaEventBus>,
//   trace_id: String,
//   schema: Schema,
// }

// impl Planner {
//   fn new(bus: Arc<NyaEventBus>, schema: Schema) -> Self {
//     Self {
//       event_bus: bus,
//       trace_id: Uuid::new_v4().to_string(),
//       schema
//     }
//   }

//   async fn execute(&self, payload: Payload) -> Result<(), String>{
    
//     for (i, step) in self.schema.steps.iter().enumerate() {
//         println!("\n Step {}/{}: {}", i + 1, self.schema.steps.len(), step);
        
//         let step_handle: JoinHandle<()> = self.event_bus.emit(step.clone(), payload.clone()).await;
//         step_handle.await.map_err(|e| format!("Step '{}' failed: {:?}", step, e))?;
        
//         println!("Step {} completed", i + 1);
//     }
    
//     println!("\n Execution completed successfully!");
//     Ok(())
//   }
// }