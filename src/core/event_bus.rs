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

#[cfg(test)]
mod event_bus_tests{
  use serde_json::{from_value, Value};

use crate::{core::
    {
      event_bus::{EventBus, NyaEventBus}, payload::Payload, service::
        {
          service_tests::TestService, Service
        }
    }, runtime::nya::Nya};

  #[tokio::test]
  async fn can_register_events() {
    let mut event_bus = NyaEventBus::new();
    let svc = Box::new(TestService);
    event_bus.on("test_event".to_string(), svc.register()[0].1.clone());
    assert_eq!(event_bus.event_handlers.len(), 1);
  }

  #[tokio::test]
  async fn event_bus_can_run_handlers_on_event() {
    let mut event_bus = NyaEventBus::new();
    let svc = Box::new(TestService);
    let handler= svc.register()[0].1.clone();
    let event_name = svc.register()[0].0.clone();
    let test_nya = Nya::build("test_cmd", "./context/nya_test_context.json", vec![Box::new(TestService)]);
    {
      event_bus.on(event_name.clone(), handler);
      event_bus.emit(test_nya.clone(), event_name, Payload::empty()).await;
    }
    tokio::task::yield_now().await;
    let ctx_val = test_nya.get("test_key").await;
    let value: String = from_value(ctx_val.clone()).unwrap();
    assert_eq!(value, "test_value".to_string());
  }

  #[tokio::test]
  async fn event_bus_can_run_multiple_handlers_for_same_event() {
    let mut event_bus = NyaEventBus::new();
    let svc = Box::new(TestService);
    let handler1= svc.register()[0].1.clone();
    let handler2= svc.register()[1].1.clone();
    let event_name = svc.register()[0].0.clone();
    let test_nya = Nya::build("test_cmd", "./context/nya_test_context.json", vec![Box::new(TestService)]);
    {
      event_bus.on(event_name.clone(), handler1);
      event_bus.on(event_name.clone(), handler2);
      event_bus.emit(test_nya.clone(), event_name, Payload::empty()).await;
    }
    tokio::task::yield_now().await;
    let ctx_val = test_nya.get("test_key").await;
    let ctx_val2 = test_nya.get("test_key2").await;
    let value: String = from_value(ctx_val.clone()).unwrap();
    let value2: String = from_value(ctx_val2.clone()).unwrap();
    assert_eq!(value, "test_value".to_string());
    assert_eq!(value2, "test_value2".to_string());
  }

  #[tokio::test]
  async fn event_bus_doesnt_run_if_theres_no_event() {
    let mut event_bus = NyaEventBus::new();
    let svc = Box::new(TestService);
    let handler= svc.register()[0].1.clone();
    let event_name = svc.register()[0].0.clone();
    let test_nya = Nya::build("test_cmd", "./context/nya_test_context.json", vec![Box::new(TestService)]);
    {
      event_bus.on(event_name.clone(), handler);
      event_bus.emit(test_nya.clone(), "fake_event".to_string(), Payload::empty()).await;
    }
    tokio::task::yield_now().await;
    let ctx_val = test_nya.get("test_key").await;
    assert_eq!(Value::Null, ctx_val);
  }

}
