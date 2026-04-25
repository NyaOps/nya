use crate::{core::{NyaCore, service::Service}, ops::{base::{build::NyaBaseBuild, destroy::NyaBaseDestroy}, ship::NyaShip}};

pub fn get_core_services() -> Vec<Box<dyn Service>> {
  vec![
    Box::new(NyaCore),
    Box::new(NyaBaseBuild),
    Box::new(NyaBaseDestroy),
    Box::new(NyaShip)
  ]
}