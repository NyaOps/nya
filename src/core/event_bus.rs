use std::collections::HashMap;
use std::sync::Arc;

use crate::core::event::Event;
use crate::core::handler:: Handler;
use crate::core::payload::Payload;
use tokio; 

pub trait AsyncBus: Send + Sync + 'static {}
impl<T: Send + Sync + 'static> AsyncBus for T {}

struct NyaEventBus {
  event_handlers: HashMap<Event, Vec<Arc<dyn Handler>>>,
}

impl NyaEventBus {
  pub fn new() -> Self {
    Self {
      event_handlers: HashMap::new(),
    }
  }
}

#[async_trait::async_trait]
trait EventBus : AsyncBus {
  fn on(&mut self, event: Event, handler: Arc<dyn Handler>);
  async fn emit(&self, event: Event, payload: Payload);
}

#[async_trait::async_trait]
impl EventBus for NyaEventBus {
    fn on(&mut self, event: Event, handler: Arc<dyn Handler>) {
      self.event_handlers
        .entry(event)
        .or_insert_with(Vec::new)
        .push(handler)
      }
    
    async fn emit(&self, event: Event, payload: Payload) {
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
  use super::*;

  struct TestEventHandler;
  struct LogEventHandler;

  #[async_trait::async_trait]
  impl Handler for TestEventHandler {
      async fn handler(&self, payload: &mut dyn Any) {
          if let Some(_cell) = payload.downcast_ref::<RefCell<String>>() {
            println!("test_event");
          }
      }
  }

  #[async_trait::async_trait]
  impl Handler for LogEventHandler {
      async fn handler(&self, payload: &mut dyn Any) {
          if let Some(logs) = payload.downcast_ref::<RefCell<Vec<String>>>() {
            logs.borrow_mut().push(String::from("test_string"));
          }
      }
  }

  #[test]
  fn can_get_event_handler_names() {
    let event = TestEventHandler;
    let event_name = &event.name();
    let event_short_name = &event.short_name();
    assert!(&event_name.contains("TestEventHandler"));
    assert_eq!(*event_short_name, "TestEventHandler");
  }

  #[tokio::test]
  async fn can_call_handler() {
      let mut logs: RefCell<Vec<String>> = RefCell::new(vec![]);
      let event = LogEventHandler;

      let _ = event.handler(&mut logs as &mut dyn Any).await;

      assert_eq!(logs.borrow().len(), 1);
      assert_eq!(logs.borrow()[0], "test_string");
  }
}
