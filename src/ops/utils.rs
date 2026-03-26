use openssh::{Session, SessionBuilder};
use serde_json::Value;
use crate::{core::runtime::Nya, ops::types::{BaseNodeConfig, NodeCommandResult}};

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

pub async fn get_control_plane_config(nya: Nya) -> BaseNodeConfig {
  let control_plane_value: Value = nya.get("nya.control_plane").await;
  BaseNodeConfig::new(control_plane_value)
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

pub async fn run_on_node(session: &Session, command: &str) -> NodeCommandResult {
    match session.command("sh")
        .arg("-c")
        .arg(command)
        .output()
        .await
    {
        Ok(output) => {
            if !output.status.success() {
                eprintln!("Command error: {}", String::from_utf8_lossy(&output.stderr));
                return NodeCommandResult::Failure(String::from_utf8_lossy(&output.stderr).to_string());
            }
            NodeCommandResult::Success
        },
        Err(e) => {
          eprintln!("Command error: {}", e.to_string());
          return NodeCommandResult::Failure(e.to_string());

        },
    }
}

pub async fn get_from_node(session: &Session, command: &str) -> Result<String, String> {
    match session.command("sh")
        .arg("-c")
        .arg(command)
        .output()
        .await
    {
        Ok(output) => {
            if !output.status.success() {
                eprintln!("Command error: {}", String::from_utf8_lossy(&output.stderr));
                return Err(String::from_utf8_lossy(&output.stderr).to_string());
            }
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        },
        Err(e) => {
          eprintln!("Command error: {}", e.to_string());
          return Err(e.to_string());
        },
    }
}

pub async fn prepare_base_context(nya: Nya) {
  let control_plane_value: Value = nya.get("nya.registry_host").await;
  if control_plane_value == Value::Null || control_plane_value.as_str().unwrap_or("").is_empty() {
    let control_plane_value: Value = nya.get("nya.control_plane").await;
    let host = control_plane_value.get("host").and_then(|v| v.as_str()).unwrap_or("");
    let _ = nya.set("nya.registry_host", host.to_string()).await;
  }

  let control_plane_vars = nya.get("nya.control_plane.vars").await;
  let k3s_token = control_plane_vars.get("k3s_token").unwrap().to_string();
  let _ = nya.set("nya.k3s_token", k3s_token).await;
}