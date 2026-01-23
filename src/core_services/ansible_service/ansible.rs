use std::{env::{self, temp_dir}, process::Stdio};

use anyhow::Error;
use serde_json::{Value, to_string, to_string_pretty};
use tokio::{io::{AsyncBufReadExt, BufReader}, process::Command};

use crate::core::{payload::Payload, service::{handle_function, Service, ServiceRegister}};
use crate::runtime::nya::Nya;
use std::fs;
use std::path::PathBuf;
use regex::Regex;

pub struct Ansible;
const BUILD_CONTROL_PLANE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/playbooks/build_control_plane.yml"
); 

const BUILD_NODES: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/playbooks/build_nodes.yml"
);

impl Service for Ansible {
  fn name(&self) -> String {"Ansible".to_string()}
  fn register(&self) -> ServiceRegister {
    vec![
      ("onBuildMainServer".to_string(), handle_function(build_control_plane)),
      ("onBuildNodeServers".to_string(), handle_function(build_nodes)),
    ]
  }
}

pub async fn build_control_plane(nya: Nya, _: Payload) {
  _ = &nya.trigger("log", Payload::new("Building control plane...".to_string())).await;

  let nya_inventory_value = nya.get("nya.control_plane").await;
  let mut tmp_inv_path = PathBuf::new();
  match to_string_pretty(&nya_inventory_value) {
    Ok(inv) => {
      // write to a temp file instead of passing inline:
      tmp_inv_path = temp_dir().join("inventory.json");
      match std::fs::write(&tmp_inv_path, &inv) {
        Err(e) => { let _ = nya.trigger("log", Payload::new(format!("Failed to create temp inventory file: {e}"))).await; },
        _ => ()
      }
    },
    Err(e) => { let _ = nya.trigger("log", Payload::new(format!("Couldn't read inventory: {e}"))).await; }
  };

  let nya_vars_value = nya.get("nya.control_plane.vars").await;
  let vars_json = to_string(&nya_vars_value).unwrap_or_else(|_| "{}".to_string());
  let vars_arg = format!("{}", vars_json);

  let temp_path_str = tmp_inv_path.to_string_lossy().to_string();
  println!("{}", vars_arg);
  let args = vec![BUILD_CONTROL_PLANE, "-i", &temp_path_str, "-e", &vars_arg];
  if let Err(err) = run_playbook(args, nya.clone()).await {
    let _ = nya.trigger("log", Payload::new(format!("Ansible failed: {err}"))).await;
  } else {
      let _ = nya.trigger("log", Payload::new("Control plane built successfully.".to_string())).await;
  }
}

async fn build_nodes (nya: Nya, _: Payload) {
  _ = &nya.trigger("log", Payload::new("Building nodes...".to_string())).await;

  let nya_inventory_value = nya.get("nya.nodes").await;
  let mut tmp_inv_path = PathBuf::new();
  match to_string_pretty(&nya_inventory_value) {
    Ok(inv) => {
      // write to a temp file instead of passing inline:
      tmp_inv_path = temp_dir().join("inventory.json");
      match std::fs::write(&tmp_inv_path, &inv) {
        Err(e) => { let _ = nya.trigger("log", Payload::new(format!("Failed to create temp inventory file: {e}"))).await; },
        _ => ()
      }
    },
    Err(e) => { let _ = nya.trigger("log", Payload::new(format!("Couldn't read inventory: {e}"))).await; }
  };

  let node_token = nya.get("k3s_token").await;
  let mut nya_vars_value = nya.get("nya.nodes.vars").await;

  if let Value::Object(map) = &mut nya_vars_value {
    if let Value::String(token) = node_token {
        map.insert("k3s_node_token".into(), Value::String(token));
    }
  
  }
  
  let vars_json = to_string(&nya_vars_value).unwrap_or_else(|_| "{}".to_string());
  let vars_arg = format!("{}", vars_json);

  let temp_path_str = tmp_inv_path.to_string_lossy().to_string();
  println!("{}", vars_arg);
  let args = vec![BUILD_NODES, "-i", &temp_path_str, "-e", &vars_arg];
  if let Err(err) = run_playbook(args, nya.clone()).await {
    let _ = nya.trigger("log", Payload::new(format!("Ansible failed: {err}"))).await;
  } else {
      let _ = nya.trigger("log", Payload::new("Control plane built successfully.".to_string())).await;
  }
}

async fn run_playbook(cmd_args: Vec<&str>, nya: Nya) -> Result<(), Error> {

  let cp_dir = PathBuf::from("/tmp/ssh");
  fs::create_dir_all(&cp_dir)?;

  // drastically shorter ControlPath template
  let cp_pattern = cp_dir.join("a%h-%p-%r"); // "a" prefix ensures filename not too long
  let token_pattern = Regex::new(r#"K3S_TOKEN=(?P<token>[A-Za-z0-9:\.]+)"#).unwrap();

  let ssh_args = format!(
    "-o ControlMaster=auto -o ControlPersist=60s -o ControlPath={}",
    cp_pattern.display()
  );

  let mut child = Command::new("ansible-playbook")
    .args(cmd_args)
    .env("ANSIBLE_SSH_ARGS", ssh_args)
    .stdin(Stdio::null())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()?;

    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap();

    let mut out_reader = BufReader::new(stdout).lines();
    let mut err_reader = BufReader::new(stderr).lines();

    // Pump stdout
    let mut out_task = tokio::spawn({
        let nya = nya.clone();
        async move {
            while let Ok(Some(line)) = out_reader.next_line().await {
              if let Some(caps) = token_pattern.captures(&line) {
                  let token = caps.name("token").unwrap().as_str().to_string();
                  nya.set("k3s_token", token.clone()).await;
                  let _ = nya.trigger("log", Payload::new(format!("Captured K3s token: {}", token))).await;
              }
              let _ = nya.trigger("log", Payload::new(line.clone())).await;
            }
        }
    });

    // Pump stderr
    let mut err_task = tokio::spawn({
        let nya = nya.clone();
        async move {
            while let Ok(Some(line)) = err_reader.next_line().await {
                let _ = nya.trigger("log", Payload::new(line.clone())).await;
            }
        }
    });

    let status = child.wait().await?;

    let _ = (&mut out_task).await;
    let _ = (&mut err_task).await;

    if !status.success() {
        anyhow::bail!("ansible-playbook failed with {}", status);
    }
  Ok(())
}
