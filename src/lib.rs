use std::any::Any;

pub trait Event {
  fn name(&self) -> &'static str {
    std::any::type_name::<Self>()
  }
  fn short_name(&self) -> &'static str {
      std::any::type_name::<Self>().rsplit("::").next().unwrap_or("Unknown")
  }
  fn handler(&self, payload: &mut dyn Any);
}

#[cfg(test)]
mod lib_tests{
  use std::cell::RefCell;
  use super::*;

  struct TestEvent;
  struct LogEvent;

  impl Event for TestEvent {
      fn handler(&self, payload: &mut dyn Any) {
          if let Some(_cell) = payload.downcast_ref::<RefCell<String>>() {
            println!("test_event");
          }
      }
  }

  impl Event for LogEvent {
      fn handler(&self, payload: &mut dyn Any) {
          if let Some(logs) = payload.downcast_ref::<RefCell<Vec<String>>>() {
            logs.borrow_mut().push(String::from("test_string"));
          }
      }
  }

  #[test]
  fn can_get_event_names() {
    let event = TestEvent;
    let event_name = &event.name();
    let event_short_name = &event.short_name();
    assert_eq!(*event_name, "nya::lib_tests::TestEvent");
    assert_eq!(*event_short_name, "TestEvent");
  }

  #[test]
  fn can_call_handler() {
      let mut logs: RefCell<Vec<String>> = RefCell::new(vec![]);
      let event = LogEvent;

      event.handler(&mut logs as &mut dyn Any);

      assert_eq!(logs.borrow().len(), 1);
      assert_eq!(logs.borrow()[0], "test_string");
  }
}
