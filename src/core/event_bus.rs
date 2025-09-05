use std::collections::HashMap;
use std::sync::Arc;
use crate::core::context::NyaContext;
use crate::core::service::{ServiceFunction};
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
  async fn emit(&self, event: String, ctx: NyaContext) -> JoinHandle<()>;
}

#[async_trait::async_trait]
impl EventBus for NyaEventBus {
    fn on(&mut self, event: String, handler: ServiceFunction) {
      self.event_handlers
        .entry(event)
        .or_insert_with(Vec::new)
        .push(handler)
      }
    
    async fn emit(&self, event: String, ctx: NyaContext) -> JoinHandle<()> {
      let mut join_handles = Vec::new();
        if let Some(handlers) = self.event_handlers.get(&event) {
            for handler in handlers {
              let payload_clone = Arc::clone(&ctx);
              let handler_clone = Arc::clone(handler);
              let handle = tokio::spawn(async move {
                  handler_clone(payload_clone).await;
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

#[cfg(test)]
mod event_bus_tests{
  use std::{collections::HashMap, sync::{Arc, Mutex}};
  use serde_json::from_value;

  use crate::core::
    {
      context::NyaContext, 
      event_bus::{NyaEventBus, EventBus}, 
      service::
        {
          service_tests::TestService, Service
        }
    };

  #[tokio::test]
  async fn can_register_events() {
    let event_bus = Arc::new(Mutex::new(NyaEventBus::new()));
    let mut bus = event_bus.lock().unwrap();
    let svc = Box::new(TestService);
    bus.on("test_event".to_string(), svc.register()[0].1.clone());
    assert_eq!(bus.event_handlers.len(), 1);
  }

  #[tokio::test]
  async fn event_bus_can_run_handlers_on_event() {
    let event_bus = Arc::new(Mutex::new(NyaEventBus::new()));
    let svc = Box::new(TestService);
    let handler= svc.register()[0].1.clone();
    let event_name = svc.name();
    let new_nya_ctx = NyaContext::new(Mutex::new(HashMap::new()));
    {
      let mut bus = event_bus.lock().unwrap();
      bus.on(event_name.clone(), handler);
      bus.emit(event_name, new_nya_ctx.clone()).await;
    }
    tokio::task::yield_now().await;
    let ctx = new_nya_ctx.lock().unwrap();
    let ctx_val = ctx.get("test_key").unwrap();
    let value: String = from_value(ctx_val.clone()).unwrap();
    assert_eq!(value, "test_value".to_string());
  }

  #[tokio::test]
  async fn event_bus_can_run_multiple_handlers_for_same_event() {
    let event_bus = Arc::new(Mutex::new(NyaEventBus::new()));
    let svc = Box::new(TestService);
    let handler1= svc.register()[0].1.clone();
    let handler2= svc.register()[1].1.clone();
    let event_name = svc.name();
    let new_nya_ctx = NyaContext::new(Mutex::new(HashMap::new()));
    {
      let mut bus = event_bus.lock().unwrap();
      bus.on(event_name.clone(), handler1);
      bus.on(event_name.clone(), handler2);
      bus.emit(event_name, new_nya_ctx.clone()).await;
    }
    tokio::task::yield_now().await;
    let ctx = new_nya_ctx.lock().unwrap();
    let ctx_val = ctx.get("test_key").unwrap();
    let ctx_val2 = ctx.get("test_key2").unwrap();
    let value: String = from_value(ctx_val.clone()).unwrap();
    let value2: String = from_value(ctx_val2.clone()).unwrap();
    assert_eq!(value, "test_value".to_string());
    assert_eq!(value2, "test_value2".to_string());
  }

  #[tokio::test]
  async fn event_bus_doesnt_run_if_theres_no_event() {
    let event_bus = Arc::new(Mutex::new(NyaEventBus::new()));
    let svc = Box::new(TestService);
    let handler= svc.register()[0].1.clone();
    let event_name = svc.name();
    let new_nya_ctx = NyaContext::new(Mutex::new(HashMap::new()));
    {
      let mut bus = event_bus.lock().unwrap();
      bus.on(event_name.clone(), handler);
      bus.emit("fake_event".to_string(), new_nya_ctx.clone()).await;
    }
    tokio::task::yield_now().await;
    let ctx = new_nya_ctx.lock().unwrap();
    let ctx_val = ctx.capacity();
    let value: usize = 0;
    assert_eq!(value, ctx_val);
  }

}
