// event bus functionality
// is an object with a map of events to handlers
// has pub function
// has sub function 
use nya::Event;

trait EventBus {
  fn sub(&mut self, event: Box<dyn Event>);
  fn trigger(&self, event: &dyn Event, payload: &dyn Any);
}

impl EventBus {
  
}