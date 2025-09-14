use crate::{core::{payload::{Get, Payload}, service::{handle_function, Service, ServiceRegister}}};
use crate::runtime::nya::Nya;

pub struct NyaCore;

impl Service for NyaCore {
  fn name(&self) -> String {"NyaCore".to_string()}
  fn register(&self) -> ServiceRegister {
    vec![
      ("test".to_string(), handle_function(test_nya_service))
    ]
  }
}

pub async fn test_nya_service(_: Nya, payload: Payload) {
  if let Ok(payload) = payload.get::<&str>() {
    println!("Value from payload: {}", payload);
  }
}

pub fn get_core_services() -> Vec<Box<dyn Service>> {
  vec![
    Box::new(NyaCore)
  ]
}