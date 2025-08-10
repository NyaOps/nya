use std::collections::HashMap;
use std::sync::Arc;

use crate::core::handler:: Handler;
use crate::core::payload::Payload;
use tokio; 

pub trait AsyncBus: Send + Sync + 'static {}
impl<T: Send + Sync + 'static> AsyncBus for T {}

pub struct NyaEventBus {
  event_handlers: HashMap<String, Vec<Arc<dyn Handler>>>,
}

impl NyaEventBus {
  pub fn new() -> Self {
    Self {
      event_handlers: HashMap::new(),
    }
  }
}

#[async_trait::async_trait]
pub trait EventBus: AsyncBus {
  fn on(&mut self, event: String, handler: Arc<dyn Handler>);
  async fn emit(&self, event: String, payload: Payload);
}

#[async_trait::async_trait]
impl EventBus for NyaEventBus {
    fn on(&mut self, event: String, handler: Arc<dyn Handler>) {
      self.event_handlers
        .entry(event)
        .or_insert_with(Vec::new)
        .push(handler)
      }
    
    async fn emit(&self, event: String, payload: Payload) {
        if let Some(handlers) = self.event_handlers.get(&event) {
            for handler in handlers {
              let payload_clone = Arc::clone(&payload);
              let handler_clone = Arc::clone(handler);
              tokio::spawn(async move {
                  handler_clone.run(payload_clone).await;
              });
            }
        }
    }
}

// make this dryn
#[cfg(test)]
mod event_bus_tests{
  use std::sync::Mutex;
  use crate::core::payload::{extract, payload};
  use tokio;

use super::*;
  
  pub struct LogHandler {
    pub messages: Arc<Mutex<Vec<String>>>, // Public for test inspection
  }

  impl LogHandler {
      pub fn new() -> Self {
          Self {
              messages: Arc::new(Mutex::new(Vec::new())),
          }
      }
  }

  #[async_trait::async_trait]
  impl Handler for LogHandler {
      async fn run(&self, payload: Payload) {
          if let Some(message) = extract::<String>(&payload) {
              let mut msgs = self.messages.lock().unwrap();
              msgs.push(message.clone());
          }
      }
  }

  #[tokio::test]
  async fn can_register_events() {
    let event_bus = Arc::new(Mutex::new(NyaEventBus::new()));
    let handler = LogHandler::new();
    let mut bus = event_bus.lock().unwrap();
    bus.on("test_event".to_string(), Arc::new(handler));
    assert_eq!(bus.event_handlers.len(), 1);
  }

  #[tokio::test]
  async fn event_bus_can_run_handlers_on_event() {
    let event_bus = Arc::new(Mutex::new(NyaEventBus::new()));
    let handler = Arc::new(LogHandler::new());
    let arc_msg = Arc::clone(&handler.messages);
    {
      let mut bus = event_bus.lock().unwrap();
      bus.on("test_event".to_string(), handler);
      bus.emit("test_event".to_string(), payload("test_string".to_string())).await;
    }
    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
    let value: usize = 1; 
    assert_eq!(&arc_msg.lock().unwrap().len(), &value);
    assert_eq!(&arc_msg.lock().unwrap()[0], "test_string");
  }

  #[tokio::test]
  async fn event_bus_can_run_multiple_handlers_for_same_event() {
    let event_bus = Arc::new(Mutex::new(NyaEventBus::new()));
    let handler = Arc::new(LogHandler::new());
    let handler2 = Arc::new(LogHandler::new());
    let arc_msg = Arc::clone(&handler.messages);
    let arc_msg2 = Arc::clone(&handler2.messages);
    {
      let mut bus = event_bus.lock().unwrap();
      bus.on("test_event".to_string(), handler);
      bus.on("test_event".to_string(), handler2);
      bus.emit("test_event".to_string(), payload("test_string".to_string())).await;
    }
    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
    let value: usize = 1; 
    assert_eq!(&arc_msg.lock().unwrap().len(), &value);
    assert_eq!(&arc_msg2.lock().unwrap().len(), &value);
  }

  #[tokio::test]
  async fn event_bus_doesnt_run_if_theres_no_event() {
    let event_bus = Arc::new(Mutex::new(NyaEventBus::new()));
    let handler = Arc::new(LogHandler::new());
    let arc_msg = Arc::clone(&handler.messages);
    {
      let mut bus = event_bus.lock().unwrap();
      bus.on("fake_event".to_string(), handler);
      bus.emit("test_event".to_string(), payload("test_string".to_string())).await;
    }
    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
    let value: usize = 0; 
    assert_eq!(&arc_msg.lock().unwrap().len(), &value);
  }

}
