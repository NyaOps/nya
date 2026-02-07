#[cfg(test)]
pub mod service_tests{
  use std::sync::Arc;
  use serde_json::from_value;

use crate::core::{context::NyaContext, service::{handle_function, NyaService, ServiceFunction, ServiceHandler}};

  pub async fn test_fn(ctx: Arc<NyaContext>) {
    let current_ctx = ctx.clone();
    let mut test_ctx = current_ctx.context.lock().unwrap();
    test_ctx.insert("test_key".to_string(), serde_json::Value::String("test_value".to_string()));
  }
  pub async fn test_fn2(ctx: Arc<NyaContext>) {
    let current_ctx = ctx.clone();
    let mut test_ctx = current_ctx.context.lock().unwrap();
    test_ctx.insert("test_key2".to_string(), serde_json::Value::String("test_value2".to_string()));
  }

  pub struct TestService;
  impl NyaService for TestService {
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
    let new_nya_ctx = Arc::new(NyaContext::new_create_bus("./tests/context/nya_test_context.json"));
    new_svc_fn(new_nya_ctx.clone()).await;
    let ctx = new_nya_ctx.context.lock().unwrap();
    let ctx_val = ctx.get("test_key").unwrap();
    let value: String = from_value(ctx_val.clone()).unwrap();
    assert_eq!(value, "test_value".to_string());
  }

  #[tokio::test]
  async fn can_create_service() {
    let new_nya_ctx = Arc::new(NyaContext::new_create_bus("./tests/context/nya_test_context.json"));
    let svc = Box::new(TestService);
    let new_svc = &svc.register();
    let new_svc_name = &svc.name();
    let test_fn = new_svc[0].1.clone();
    test_fn(new_nya_ctx.clone()).await;
    
    assert_eq!(*new_svc_name, "Test Service".to_string());

    let ctx = new_nya_ctx.context.lock().unwrap();
    let ctx_val = ctx.get("test_key").unwrap();
    let value: String = from_value(ctx_val.clone()).unwrap();
    assert_eq!(value, "test_value".to_string());
  }
}