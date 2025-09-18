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

pub async fn test_nya_service(nya: Nya, payload: Payload) {
  let ctx_val = nya.nya_ctx_get("test").await;
  let pay_val = payload.get::<&str>().unwrap();
  println!("Value from payload: {}", pay_val);
  println!("Value from context: {}", ctx_val.to_string());
}

pub fn get_core_services() -> Vec<Box<dyn Service>> {
  vec![
    Box::new(NyaCore)
  ]
}