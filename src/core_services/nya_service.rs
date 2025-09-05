use crate::core::{context::NyaContext, service::{handle_function, Service}};

pub struct NyaService;
impl Service for NyaService {
    fn name(&self) -> String {"NyaService".to_string()}
    fn register(&self) -> Vec<(String, crate::core::service::ServiceFunction)> {
      vec![("test".to_string(), handle_function(test_nya_service))]
    }
}

pub async fn test_nya_service(ctx: NyaContext) {
  let nya_ctx = ctx.lock().unwrap();
  let test_value = nya_ctx.get("test1")
      .and_then(|v| v.as_str()).unwrap();
  println!("Value from ctx: {}", test_value);
}