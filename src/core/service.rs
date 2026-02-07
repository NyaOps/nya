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

use crate::{core::{payload::Payload, service::{handle_function, Service, ServiceFunction, ServiceRegister}}, runtime::nya::Nya};

  pub async fn test_fn(nya: Nya, _: Payload) {
    nya.set("test_key", serde_json::Value::String("test_value".to_string())).await;
  }
  pub async fn test_fn2(nya: Nya, _: Payload) {
    nya.set("test_key2", serde_json::Value::String("test_value2".to_string())).await;
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
  

  #[tokio::test]
  async fn can_create_service_function() {
    let new_svc_fn: ServiceFunction = handle_function(test_fn);
    let test_nya = Nya::build("test_cmd", "./tests/context/nya_test_context.json", vec![Box::new(TestService)]);
    new_svc_fn(test_nya.clone(), Payload::empty()).await;
    let value_json = test_nya.get("test_key").await;
    let value = value_json.as_str().unwrap();
    assert_eq!(value, "test_value");
  }

  #[tokio::test]
  async fn can_create_service() {
    let svc = Box::new(TestService);
    let new_svc = &svc.register();
    let new_svc_name = &svc.name();
    let value: usize = 2;
    assert_eq!(*new_svc_name, "Test Service".to_string());
    assert_eq!(value, new_svc.len());
  }
}