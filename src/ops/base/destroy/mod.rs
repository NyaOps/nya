use crate::{core::{payload::{Payload, Take}, runtime::Nya, service::{Service, ServiceActions, handle_action}}, ops::utils::{get_base_nodes, run_on_node}};
use crate::ops::{types, utils};
use openssh::Session;

const REMOVE_DOCKER_SCRIPT: &str = include_str!("scripts/remove_docker.sh");
const REMOVE_K3S_SCRIPT: &str = include_str!("scripts/remove_k3s.sh");
const REMOVE_MKCERT_SCRIPT: &str = include_str!("scripts/remove_mkcert.sh");
const REMOVE_INGRESS_SCRIPT: &str = include_str!("scripts/remove_ingress.sh");

pub struct NyaBaseDestroy;

use types::BaseNodeConfig;
use utils::create_ssh_session;
impl Service for NyaBaseDestroy {
  fn name(&self) -> String {"NyaBase".to_string()}
  fn register(&self) -> ServiceActions {
    vec![
      (String::from("onDestroyBase"), handle_action(destroy_action)),
      (String::from("runCleanup"), handle_action(run_cleanup_script)),
      (String::from("runRemoveMkcert"), handle_action(run_remove_mkcert_script)),
      (String::from("runRemoveIngress"), handle_action(run_remove_ingress_script)),
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
  
  let control_plane_config: BaseNodeConfig = utils::get_control_plane_config(nya.clone()).await;
  let ingress_session = create_ssh_session(&control_plane_config).await;
  let mkcert_session = create_ssh_session(&control_plane_config).await;
  nya.trigger("runRemoveIngress", Payload::new(ingress_session)).await;
  nya.trigger("runRemoveMkcert", Payload::new(mkcert_session)).await;
}

async fn run_cleanup_script(_: Nya, payload: Payload) {
  let session = payload.take::<Session>().unwrap();
  let _ = session.command("sh") 
      .arg("-c")
      .arg(REMOVE_DOCKER_SCRIPT)
      .output()
      .await
      .unwrap();
  let _ = session.command("sh") 
      .arg("-c")
      .arg(REMOVE_K3S_SCRIPT)
      .output()
      .await
      .unwrap();
}

async fn run_remove_mkcert_script(_: Nya, payload: Payload) {
  let session = payload.take::<Session>().unwrap();
  let _ = session.command("sh") 
      .arg("-c")
      .arg(REMOVE_MKCERT_SCRIPT)
      .output()
      .await
      .unwrap();
}

async fn run_remove_ingress_script(_: Nya, payload: Payload) {
  let session = payload.take::<Session>().unwrap();
  let _ = session.command("sh") 
      .arg("-c")
      .arg(REMOVE_INGRESS_SCRIPT)
      .output()
      .await
      .unwrap();
}