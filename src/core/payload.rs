use std::{any::Any, sync::Arc};

pub type Payload = Arc<dyn Any + Send + Sync>;

// For preparing event payload for async handler
#[inline]
pub fn payload<T: Any + Send + Sync>(value: T) -> Payload {
  Arc::new(value)
}

// For extracting the payload data 
#[inline]
pub fn extract<T: Any>(p: &Payload) -> Option<&T> {
  p.downcast_ref::<T>()
}

//For extracting the payload data, but keeping it in an ARC
#[inline]
pub fn extract_owned<T: Any + Send + Sync>(p: &Payload) -> Option<Arc<T>> {
  let cloned = Arc::clone(p);
  Arc::downcast::<T>(cloned).ok()
}

// For when you just want to check if payload is a certain type
#[inline]
pub fn is_type<T: Any>(p: &Payload) -> bool {
    p.downcast_ref::<T>().is_some()
}

// For debugging - what type is this payload?
pub fn type_name(p: &Payload) -> &'static str {
    std::any::type_name_of_val(p.as_ref())
}


#[cfg(test)]
mod payload_tests {
  use std::sync::Arc;
  use crate::core::payload::{extract, extract_owned, is_type, payload, type_name};
  const TEST_NUMBER: i32 = 77; 

  #[test]
  fn payload_creates_arc_extract_returns_downcast() {
    let test_payload = payload(77);
    assert_eq!(extract::<i32>(&test_payload).unwrap(), &TEST_NUMBER);
  }

  #[test]
  fn payload_creates_arc_extract_owned_returns_downcast_in_arc() {
    let test_payload = payload(77);
    assert_eq!(extract_owned::<i32>(&test_payload).unwrap(), Arc::new(TEST_NUMBER));
  }

  #[test]
  fn verifies_the_type() {
    let test_payload = payload(&TEST_NUMBER);
    assert!(is_type::<&i32>(&test_payload));
  }

  #[test]
  fn returns_type_name() {
    let test_payload = payload(77);
    assert_eq!(type_name(&test_payload), "dyn core::any::Any + core::marker::Send + core::marker::Sync");
  }
  #[test]
  fn extract_wrong_type_returns_none() {
      let test_payload = payload("hello");
      assert!(extract::<i32>(&test_payload).is_none());
  }
}