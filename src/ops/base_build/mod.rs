use crate::core::service::{Service, ServiceActions};

pub struct NyaBase;

impl Service for NyaBase {
  fn name(&self) -> String {"NyaBase".to_string()}
  fn register(&self) -> ServiceActions {
    vec![
    ]
  }
}