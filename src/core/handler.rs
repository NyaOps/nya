use crate::core::payload::{Payload};
pub trait AsyncHandler: Send + Sync + 'static {}
impl<T: Send + Sync + 'static> AsyncHandler for T {}

#[async_trait::async_trait]
pub trait Handler: AsyncHandler {
  fn name(&self) -> &'static str {
    std::any::type_name::<Self>()
  }
  fn short_name(&self) -> &'static str {
      std::any::type_name::<Self>().rsplit("::").next().unwrap_or("Unknown")
  }
  async fn run(&self, payload: Payload);
}

#[cfg(test)]
mod handler_tests{
  use std::{sync::{Arc, Mutex}};
  use crate::core::{handler::Handler, payload::{extract, Payload, payload}};

  pub struct TestHandler;

  #[async_trait::async_trait]
  impl Handler for TestHandler {
      async fn run(&self, _: Payload) {
            println!("test_event");
      }
  }

  pub struct LogHandler {
    pub messages: Arc<Mutex<Vec<String>>>, // Public for test inspection
  }

  impl LogHandler {
      pub fn new() -> Self {
          Self {
              messages: Arc::new(Mutex::new(Vec::new())),
          }
      }
      
      // Helper for tests
      pub fn get_messages(&self) -> Vec<String> {
          self.messages.lock().unwrap().clone()
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

  #[test]
  fn can_get_handler_names() {
    let handler = TestHandler;
    let handler_name = &handler.name();
    let handler_short_name = &handler.short_name();
    assert!(&handler_name.contains("TestHandler"));
    assert_eq!(*handler_short_name, "TestHandler");
  }

  #[tokio::test]
  async fn can_run_function() {
      let handler = LogHandler::new();
      let _ = handler.run(payload("test_string".to_string())).await;
      assert_eq!(handler.get_messages().len(), 1);
      assert_eq!(handler.get_messages()[0], "test_string");
  }
}