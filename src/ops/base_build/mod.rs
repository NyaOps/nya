use crate::core::{payload::Payload, runtime::Nya, service::{Service, ServiceActions, handle_action}};
use openssh::SessionBuilder;
use serde_json::Value;

pub struct NyaBase;
pub(crate) mod types;
use types::BaseNodeConfig;
impl Service for NyaBase {
  fn name(&self) -> String {"NyaBase".to_string()}
  fn register(&self) -> ServiceActions {
    vec![
      (String::from("onPreBuild"), handle_action(prebuild_action))
    ]
  }
}

async fn prebuild_action(nya: Nya, _: Payload) {
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

  let mut session_builder = SessionBuilder::default();
  session_builder.user(control_plane.user.clone());
  session_builder.keyfile(control_plane.ssh_key_path.clone());

  let new_session = session_builder.connect(control_plane.host.clone()).await;

  let test = match new_session {
    Ok(session) => session,
    Err(e) => {
      println!("Failed to connect to control plane node at {}: {:?}", control_plane.host, e);
      panic!("Failed to connect to control plane node");
    }
  };

  let output = test.command("hostname").output().await.unwrap();
  // println!("Successfully connected to control plane node at {}", output);
  println!("Connected! Remote hostname: {}", String::from_utf8_lossy(&output.stdout));

  test.close().await.unwrap();
  
  println!("Building the base");
  println!("Running the prebuild");
}