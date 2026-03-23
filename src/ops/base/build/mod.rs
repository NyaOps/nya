use crate::{core::{payload::{Payload, Take}, runtime::Nya, service::{Service, ServiceActions, handle_action}}, ops::utils::get_base_nodes};
use crate::ops::{types, utils};
use openssh::Session;

const INSTALL_DOCKER_SCRIPT: &str = include_str!("scripts/install_docker.sh");

pub struct NyaBaseBuild;

use types::BaseNodeConfig;
use utils::create_ssh_session;
impl Service for NyaBaseBuild {
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

  let node_configs: Vec<BaseNodeConfig> = get_base_nodes(nya.clone()).await;

  let mut pre_build_tasks: Vec<(&str, Payload)> = Vec::new();
  for node in node_configs.iter() {
    let session = create_ssh_session(node).await;
    pre_build_tasks.push(("runPreBuild", Payload::new(session)));
  }
  nya.trigger_all(pre_build_tasks).await;
}

async fn run_prebuild_script(_: Nya, payload: Payload) {
  let session = payload.take::<Session>().unwrap();
  let encoded = base64::encode(INSTALL_DOCKER_SCRIPT);
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