use std::path::{Path, PathBuf};
use std::sync::Arc;
use serde::Serialize;
use serde_json::Value;
use tokio::{sync::Mutex, task::JoinHandle};
use crate::core::{context::NyaContext, event_bus::{EventBus, NyaEventBus}, payload::Payload, schema::NyaSchema, service::Service, task_tracker::TaskTracker};
use crate::external::get_core_services;

struct NyaInternals {
  context: Arc<Mutex<NyaContext>>,
  schema: NyaSchema,
  bus: Arc<NyaEventBus>,
  task_tracker: TaskTracker,
  base_config_path: PathBuf,
  capsule_path: Option<PathBuf>,
}

#[derive(Clone)]
pub struct Nya {
  internals: Arc<NyaInternals>
}

impl Nya {
  pub async fn run(cmd: &str, config: PathBuf, capsule: Option<PathBuf>) {
    let services = get_core_services();
    let nya = Nya::build(cmd, config, capsule, services);
    nya.execute(Payload::empty()).await;
  }

  pub fn build(cmd: &str, config: PathBuf, capsule: Option<PathBuf>, reg: Vec<Box<dyn Service>>) -> Self {
    let nya_event_bus = build_nya_bus(reg);
    let ctx = NyaContext::new(config.clone(), capsule.clone());
    let schema = NyaSchema::new(cmd);
    let internals = NyaInternals {
      context: Arc::new(Mutex::new(ctx)),
      schema,
      bus: Arc::new(nya_event_bus),
      task_tracker: TaskTracker::new(),
      base_config_path: config.clone(),
      capsule_path: capsule.clone()
    };

    Self {
      internals: Arc::new(internals)
    }
  }

  pub async fn execute(&self, initial_payload: Payload) {
    for step in self.internals.schema.steps.iter() {
      self.internals.bus.clone().emit(self.clone(), step.clone(), initial_payload.clone()).await;
      self.internals.task_tracker.wait_all().await;
    }
  }

  pub async fn get(&self, key: &str) -> Value {
    let ctx = self.internals.context.lock().await;
    if let Some(item) = ctx.context.get(key) {
      return item.clone()
    }
    return Value::Null;
  }

  pub async fn set<T: Serialize>(&self, key: &str, value: T) {
    let mut ctx = self.internals.context.lock().await;
    if let Ok(json_value) = serde_json::to_value::<T>(value) {
      ctx.context.insert(key.to_string(), json_value);
    }
  }

  pub async fn trigger(&self, event: &str, payload: Payload) {
    let nya = self.clone();
    let event_name = event.to_string();
    let handle: JoinHandle<()> = tokio::spawn(async move {
        nya.internals.bus.emit(nya.clone(), event_name, payload).await;
    });
    self.internals.task_tracker.add(handle).await;
  }

  pub async fn trigger_all(&self, triggers: Vec<(&str, Payload)>) {
    for (event, payload) in triggers {
      self.trigger(event, payload).await;
    }
  }
  
  pub fn base_config_path(&self) -> &Path {
    &self.internals.base_config_path
  }

  pub fn capsule_path(&self) -> Option<&Path> {
    self.internals.capsule_path.as_deref()
  }
}

fn build_nya_bus(reg: Vec<Box<dyn Service>>) -> NyaEventBus {
  let mut nya_event_bus = NyaEventBus::new();
  let mut service_handlers = Vec::new();
  for service in reg.iter().clone() {
    service_handlers.extend(service.register());
  }
  for handler in service_handlers {
    nya_event_bus.on(handler.0, handler.1);
  }
  nya_event_bus
}

#[cfg(test)]
mod nya_tests {
    use std::path::PathBuf;
    use crate::{core::{payload::Payload, service::service_tests::TestService, runtime::Nya}};

  #[test]
  fn can_build_nya() {
    let _ = Nya::build("test_cmd", PathBuf::from("./tests/nya_test_config.json"), None, vec![Box::new(TestService)]);
  }

  #[tokio::test]
  async fn can_run_nya_schema() {
    let nya = Nya::build("test_cmd2", PathBuf::from("./tests/nya_test_config.json"), None, vec![Box::new(TestService)]);
    nya.execute(Payload::empty()).await;
    tokio::task::yield_now().await;
    let ctx = nya.internals.context.lock().await;
    let val1 = ctx.context.get("test_key").unwrap().as_str().unwrap();
    assert_eq!("test_value", val1);
  }

#[tokio::test]
  async fn can_get_value_from_nya() {
    let nya = Nya::build("test_cmd2", PathBuf::from("./tests/nya_test_config.json"), None, vec![Box::new(TestService)]);
    nya.execute(Payload::empty()).await;
    tokio::task::yield_now().await;
    let nya_val = &nya.get("test_key").await;
    assert_eq!("test_value", nya_val.as_str().unwrap());
  }

  #[tokio::test]
  async fn can_set_value_on_nya() {
    let nya = Nya::build("test_cmd2", PathBuf::from("./tests/nya_test_config.json"), None, vec![Box::new(TestService)]);
    let _ = &nya.set("test_key", "test_value").await;
    let nya_val = &nya.get("test_key").await;
    let val1 = nya_val.as_str().unwrap();
    assert_eq!("test_value", val1);
  }

  #[tokio::test]
  async fn can_trigger_nya_event() {
    let nya = Nya::build("test_cmd2", PathBuf::from("./tests/nya_test_config.json"), None, vec![Box::new(TestService)]);
    {
      nya.trigger("test", Payload::empty()).await;
    }
    tokio::task::yield_now().await;
    let ctx = nya.internals.context.lock().await;
    let val1 = ctx.context.get("test_key").unwrap().as_str().unwrap();
    assert_eq!("test_value", val1);
  }
}