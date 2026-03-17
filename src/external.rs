use crate::{core::{NyaCore, service::Service}, ops::{base_build::NyaBase, ship::NyaShip}};

pub fn get_core_services() -> Vec<Box<dyn Service>> {
  vec![
    Box::new(NyaCore),
    Box::new(NyaBase),
    Box::new(NyaShip)
  ]
}