use std::{collections::HashMap, sync::{Arc, Mutex}};
use futures::{future::BoxFuture, FutureExt};
use serde_json::Value;
use crate::core::context::NyaContext;

pub type ServiceFunction = Arc<dyn Fn(NyaContext) -> BoxFuture<'static, ()> + Send + Sync>;
pub type ServiceHandler = (String, ServiceFunction);

pub fn handle_function<F, Fut>(f: F) -> ServiceFunction
where
    F: Fn(NyaContext) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ()> + Send + 'static, {
    Arc::new(move |ctx: Arc<Mutex<HashMap<String, Value>>>| f(ctx).boxed())
}

pub trait Service: Send + Sync + 'static {
    fn name(&self) -> String;
    fn register(&self) -> Vec<ServiceHandler>;
}

#[cfg(test)]
pub mod service_tests{
  use std::{collections::HashMap, sync::Mutex};
  use serde_json::from_value;

use crate::core::{context::NyaContext, service::{handle_function, Service, ServiceFunction, ServiceHandler}};

  pub async fn test_fn(ctx: NyaContext) {
    let mut test_ctx = ctx.lock().unwrap();
    test_ctx.insert("test_key".to_string(), serde_json::Value::String("test_value".to_string()));
  }
  pub async fn test_fn2(ctx: NyaContext) {
    let mut test_ctx = ctx.lock().unwrap();
    test_ctx.insert("test_key2".to_string(), serde_json::Value::String("test_value2".to_string()));
  }

  pub struct TestService;
  impl Service for TestService {
    fn name(&self) -> String { "Test Service".to_string()}
    fn register(&self) -> Vec<ServiceHandler> {
        vec![
          ("test".to_string(), handle_function(test_fn)),
          ("test".to_string(), handle_function(test_fn2))
        ]
    }
  }

  #[tokio::test]
  async fn can_create_service_function() {
    let new_svc_fn: ServiceFunction = handle_function(test_fn);
    let new_nya_ctx = NyaContext::new(Mutex::new(HashMap::new()));
    new_svc_fn(new_nya_ctx.clone()).await;
    let ctx = new_nya_ctx.lock().unwrap();
    let ctx_val = ctx.get("test_key").unwrap();
    let value: String = from_value(ctx_val.clone()).unwrap();
    assert_eq!(value, "test_value".to_string());
  }

  #[tokio::test]
  async fn can_create_service() {
    let new_nya_ctx = NyaContext::new(Mutex::new(HashMap::new()));
    let svc = Box::new(TestService);
    let new_svc = &svc.register();
    let new_svc_name = &svc.name();
    let test_fn = new_svc[0].1.clone();
    test_fn(new_nya_ctx.clone()).await;
    
    assert_eq!(*new_svc_name, "Test Service".to_string());

    let ctx = new_nya_ctx.lock().unwrap();
    let ctx_val = ctx.get("test_key").unwrap();
    let value: String = from_value(ctx_val.clone()).unwrap();
    assert_eq!(value, "test_value".to_string());
  }
}