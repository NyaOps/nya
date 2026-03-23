use openssh::{Session, SessionBuilder};
use serde_json::Value;
use crate::core::runtime::Nya;

use super::types::BaseNodeConfig;

pub async fn get_base_nodes(nya: Nya) -> Vec<BaseNodeConfig> {
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
  all_nodes 
}

pub async fn create_ssh_session(node: &BaseNodeConfig) -> Session {
    let mut session_builder = SessionBuilder::default();
    session_builder.user(node.user.clone());
    session_builder.keyfile(node.ssh_key_path.clone());

    match session_builder.connect(node.host.clone()).await {
        Ok(session) => session,
        Err(e) =>  { 
          println!("Failed to connect to node at {}: {:?}", node.host, e);
          panic!("Failed to connect to node");
        },
    }
}