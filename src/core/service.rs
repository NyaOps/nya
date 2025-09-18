use std::sync::Arc;
use futures::{future::BoxFuture, FutureExt};
use crate::{core::payload::Payload, runtime::{nya::Nya}};

pub type ServiceFunction = Arc<dyn Fn(Nya, Payload) -> BoxFuture<'static, ()> + Send + Sync>;
pub type ServiceRegister = Vec<(String, ServiceFunction)>;

pub fn handle_function<F, Fut>(f: F) -> ServiceFunction
where
    F: Fn(Nya, Payload) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ()> + Send + 'static, {
    Arc::new(move |nya: Nya, payload: Payload| f(nya, payload).boxed())
}

pub trait Service: Send + Sync + 'static {
    fn name(&self) -> String;
    fn register(&self) -> ServiceRegister;
}

#[cfg(test)]
pub mod service_tests{
  use std::sync::Arc;
  use serde_json::from_value;

use crate::{core::{context::NyaContext, payload::Payload, service::{handle_function, Service, ServiceFunction, ServiceRegister}}, runtime::nya::Nya};

// TODO: Since we're not longer passing the context into the service functions, we need 
// Nya to have an api to allow us to get and set values to the context. Come back
// later to these unit tests, or come up with a different testing pattern. 

  pub async fn test_fn(nya: Nya, payload: Payload) {
    // let current_ctx = ctx.clone();
    // let mut test_ctx = current_ctx.context.lock().unwrap();
    // test_ctx.insert("test_key".to_string(), serde_json::Value::String("test_value".to_string()));
  }
  pub async fn test_fn2(nya: Nya, payload: Payload) {
    // let current_ctx = ctx.clone();
    // let mut test_ctx = current_ctx.context.lock().unwrap();
    // test_ctx.insert("test_key2".to_string(), serde_json::Value::String("test_value2".to_string()));
  }

  pub struct TestService;
  impl Service for TestService {
    fn name(&self) -> String { "Test Service".to_string()}
    fn register(&self) -> ServiceRegister {
        vec![
          ("test".to_string(), handle_function(test_fn)),
          ("test".to_string(), handle_function(test_fn2))
        ]
    }
  }

//   #[tokio::test]
//   async fn can_create_service_function() {
//     let new_svc_fn: ServiceFunction = handle_function(test_fn);
//     let new_nya_ctx = Arc::new(NyaContext::new_create_bus("./context/nya_test_context.json"));
//     new_svc_fn(new_nya_ctx.clone()).await;
//     let ctx = new_nya_ctx.context.lock().unwrap();
//     let ctx_val = ctx.get("test_key").unwrap();
//     let value: String = from_value(ctx_val.clone()).unwrap();
//     assert_eq!(value, "test_value".to_string());
//   }

//   #[tokio::test]
//   async fn can_create_service() {
//     let new_nya_ctx = Arc::new(NyaContext::new_create_bus("./context/nya_test_context.json"));
//     let svc = Box::new(TestService);
//     let new_svc = &svc.register();
//     let new_svc_name = &svc.name();
//     let test_fn = new_svc[0].1.clone();
//     test_fn(new_nya_ctx.clone()).await;
    
//     assert_eq!(*new_svc_name, "Test Service".to_string());

//     let ctx = new_nya_ctx.context.lock().unwrap();
//     let ctx_val = ctx.get("test_key").unwrap();
//     let value: String = from_value(ctx_val.clone()).unwrap();
//     assert_eq!(value, "test_value".to_string());
//   }
}