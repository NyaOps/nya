use crate::{core::{payload::{Payload, Take}, runtime::Nya, service::{Service, ServiceActions, handle_action}}, ops::utils::get_base_nodes};
use crate::ops::{types, utils};
use openssh::Session;

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

  let node_configs: Vec<BaseNodeConfig> = get_base_nodes(nya.clone()).await;

  let mut cleanup_tasks: Vec<(&str, Payload)> = Vec::new();
  for node in node_configs.iter() {
    let session = create_ssh_session(node).await;
    cleanup_tasks.push(("runCleanup", Payload::new(session)));
  }
  nya.trigger_all(cleanup_tasks).await;
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