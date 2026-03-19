use crate::core::{payload::{Get, Payload, Take}, runtime::Nya, service::{Service, ServiceActions, handle_action}};
use openssh::Session;
use serde_json::Value;

pub struct NyaBase;
pub(crate) mod types;
pub(crate) mod utils;

use types::BaseNodeConfig;
use utils::create_ssh_session;
impl Service for NyaBase {
  fn name(&self) -> String {"NyaBase".to_string()}
  fn register(&self) -> ServiceActions {
    vec![
      (String::from("onPreBuild"), handle_action(prebuild_action)),
      (String::from("runPreBuild"), handle_action(run_prebuild_script))
    ]
  }
}

async fn prebuild_action(nya: Nya, _: Payload) {
  println!("Building the base");
  println!("Running the prebuild");

  let control_plane_value: Value = nya.get("nya.control_plane").await;
  let nodes_values: Value = nya.get("nya.nodes").await;

  let control_plane: BaseNodeConfig = BaseNodeConfig ::new(control_plane_value);
  let nodes: Vec<BaseNodeConfig> = nodes_values
    .as_array()
    .unwrap_or(&vec![])
    .iter()
    .map(|node| BaseNodeConfig::new(node.clone()))
    .collect();

  let mut all_nodes = vec![control_plane.clone()]; 
  all_nodes.extend(nodes);

  let mut pre_build_tasks: Vec<(&str, Payload)> = Vec::new();
  for node in all_nodes.iter() {
    let session = create_ssh_session(node).await;
    pre_build_tasks.push(("runPreBuild", Payload::new(session)));
  }
  nya.trigger_all(pre_build_tasks).await;
}

async fn run_prebuild_script(_: Nya, payload: Payload) {
  let session = payload.take::<Session>().unwrap();
  let output = &session.command("hostname").output().await.unwrap();
  println!("Connected! Remote hostname: {}", String::from_utf8_lossy(&output.stdout));
  session.close().await.unwrap();
}