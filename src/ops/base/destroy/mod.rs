use crate::core::{payload::{Get, Payload, Take}, runtime::Nya, service::{Service, ServiceActions, handle_action}};
use crate::ops::{types, utils};
use openssh::Session;
use serde_json::Value;

const REMOVE_DOCKER_SCRIPT: &str = include_str!("scripts/remove_docker.sh");

pub struct NyaBaseDestroy;

use types::BaseNodeConfig;
use utils::create_ssh_session;
impl Service for NyaBaseDestroy {
  fn name(&self) -> String {"NyaBase".to_string()}
  fn register(&self) -> ServiceActions {
    vec![
      (String::from("onDestroyBase"), handle_action(destroy_action)),
      (String::from("runCleanup"), handle_action(run_cleanup_script))
    ]
  }
}

async fn destroy_action(nya: Nya, _: Payload) {
  println!("Destorying the base");

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
    pre_build_tasks.push(("runCleanup", Payload::new(session)));
  }
  nya.trigger_all(pre_build_tasks).await;
}

async fn run_cleanup_script(_: Nya, payload: Payload) {
  let session = payload.take::<Session>().unwrap();
  let encoded = base64::encode(REMOVE_DOCKER_SCRIPT);
  let output = session.command("sh")
      .arg("-c")
      .arg(format!("echo {} | base64 -d | sh", encoded))
      .output()
      .await
      .unwrap();

  println!("{}", String::from_utf8_lossy(&output.stdout));
  eprintln!("{}", String::from_utf8_lossy(&output.stderr));
  session.close().await.unwrap();
}