use std::collections::HashMap;
use std::sync::Arc;
use crate::core::payload::Payload;
use crate::core::service::ServiceFunction;
use crate::runtime::nya::Nya;
use tokio::task::JoinHandle;

pub struct NyaEventBus {
  event_handlers: HashMap<String, Vec<ServiceFunction>>
}

impl NyaEventBus {
  pub fn new() -> Self {
    Self {
      event_handlers: HashMap::new(),
    }
  }
}

#[async_trait::async_trait]
pub trait EventBus: Send + Sync + 'static {
  fn on(&mut self, event: String, handler: ServiceFunction);
  async fn emit(&self, nya: Nya, event: String, payload: Payload) -> JoinHandle<()>;
}

#[async_trait::async_trait]
impl EventBus for NyaEventBus {
    fn on(&mut self, event: String, handler: ServiceFunction) {
      self.event_handlers
        .entry(event)
        .or_insert_with(Vec::new)
        .push(handler)
      }
    
    async fn emit(&self, nya: Nya, event: String, payload: Payload) -> JoinHandle<()> {
      let mut join_handles = Vec::new();
        if let Some(handlers) = self.event_handlers.get(&event) {
            for handler in handlers {
              let nya_clone = nya.clone();
              let payload_clone = payload.clone();
              let handler_clone = Arc::clone(handler);
              let handle = tokio::spawn(async move {
                  handler_clone(nya_clone, payload_clone).await;
              });
              join_handles.push(handle);
            }
        }
      tokio::spawn(async move {
        for handle in join_handles {
          let _ = handle.await;
        }
      })
    }
}

// #[cfg(test)]
// mod event_bus_tests{
//   use std::sync::{Arc, Mutex};
//   use serde_json::from_value;

//   use crate::core::
//     {
//       context::NyaContext, 
//       event_bus::{NyaEventBus, EventBus}, 
//       service::
//         {
//           service_tests::TestService, Service
//         }
//     };

//   #[tokio::test]
//   async fn can_register_events() {
//     let event_bus = Arc::new(Mutex::new(NyaEventBus::new()));
//     let mut bus = event_bus.lock().unwrap();
//     let svc = Box::new(TestService);
//     bus.on("test_event".to_string(), svc.register()[0].1.clone());
//     assert_eq!(bus.event_handlers.len(), 1);
//   }

//   #[tokio::test]
//   async fn event_bus_can_run_handlers_on_event() {
//     let event_bus = Arc::new(Mutex::new(NyaEventBus::new()));
//     let svc = Box::new(TestService);
//     let handler= svc.register()[0].1.clone();
//     let event_name = svc.name();
//     let new_nya_ctx = Arc::new(NyaContext::new_create_bus("./context/nya_test_context.json"));
//     {
//       let mut bus = event_bus.lock().unwrap();
//       bus.on(event_name.clone(), handler);
//       bus.emit(event_name, new_nya_ctx.clone()).await;
//     }
//     tokio::task::yield_now().await;
//     let ctx = new_nya_ctx.context.lock().unwrap();
//     let ctx_val = ctx.get("test_key").unwrap();
//     let value: String = from_value(ctx_val.clone()).unwrap();
//     assert_eq!(value, "test_value".to_string());
//   }

//   #[tokio::test]
//   async fn event_bus_can_run_multiple_handlers_for_same_event() {
//     let event_bus = Arc::new(Mutex::new(NyaEventBus::new()));
//     let svc = Box::new(TestService);
//     let handler1= svc.register()[0].1.clone();
//     let handler2= svc.register()[1].1.clone();
//     let event_name = svc.name();
//     let new_nya_ctx = Arc::new(NyaContext::new_create_bus("./context/nya_test_context.json"));
//     {
//       let mut bus = event_bus.lock().unwrap();
//       bus.on(event_name.clone(), handler1);
//       bus.on(event_name.clone(), handler2);
//       bus.emit(event_name, new_nya_ctx.clone()).await;
//     }
//     tokio::task::yield_now().await;
//     let ctx = new_nya_ctx.context.lock().unwrap();
//     let ctx_val = ctx.get("test_key").unwrap();
//     let ctx_val2 = ctx.get("test_key2").unwrap();
//     let value: String = from_value(ctx_val.clone()).unwrap();
//     let value2: String = from_value(ctx_val2.clone()).unwrap();
//     assert_eq!(value, "test_value".to_string());
//     assert_eq!(value2, "test_value2".to_string());
//   }

//   #[tokio::test]
//   async fn event_bus_doesnt_run_if_theres_no_event() {
//     let event_bus = Arc::new(Mutex::new(NyaEventBus::new()));
//     let svc = Box::new(TestService);
//     let handler= svc.register()[0].1.clone();
//     let event_name = svc.name();
//     let new_nya_ctx = Arc::new(NyaContext::new_create_bus("./context/nya_test_context.json"));
//     {
//       let mut bus = event_bus.lock().unwrap();
//       bus.on(event_name.clone(), handler);
//       bus.emit("fake_event".to_string(), new_nya_ctx.clone()).await;
//     }
//     tokio::task::yield_now().await;
//     let ctx = new_nya_ctx.context.lock().unwrap();
//     let ctx_val = ctx.capacity();
//     let value: usize = 3;
//     assert_eq!(value, ctx_val);
//   }

//   // TODO: Now that bus is part of context, need to test 
//   // if service handler can call bus from context to trigger 
//   // event to run other handlers.

// }
