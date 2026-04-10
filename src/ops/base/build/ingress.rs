use crate::{core::{payload::Payload, runtime::Nya}, ops::{types::NodeCommandResult, utils::{get_control_plane_config, run_on_node}}};
use crate::ops::{types, utils};
use openssh::Session;
use serde::Serialize;
use types::BaseNodeConfig;
use utils::create_ssh_session;
use serde_json::Value;
use tera::Context;

const SETUP_INGRESS_SCRIPT: &str = include_str!("scripts/setup_ingress.sh");
const SETUP_INGRESS_CLOUD_SCRIPT: &str = include_str!("scripts/setup_ingress_cloud.sh");

#[derive(Serialize)]
struct IngressContext {
  domain: String,
  network_cidr: String,
  secret_name: String,
}

pub async fn setup_ingress(nya: Nya, _: Payload) {
  println!("Setting up ingress");

  let node_configs: BaseNodeConfig = get_control_plane_config(nya.clone()).await;
  let session: Session = create_ssh_session(&node_configs).await;

  let control_plane_vars: Value = nya.get("nya.control_plane.vars").await;
  let control_plane_type = control_plane_vars.get("control_plane_type").and_then(|v| v.as_str()).unwrap_or("bare_metal");
  let domain: String = control_plane_vars.get("domain_name").and_then(|v| v.as_str()).unwrap_or("").to_string();
  let network_cidr: String = nya.get("network_cidr").await.as_str().unwrap_or("").to_string();
  let secret_name: String = control_plane_vars.get("secret_name").and_then(|v| v.as_str()).unwrap_or("").to_string();

  let ingress_context: IngressContext = IngressContext {
    domain,
    network_cidr,
    secret_name,
  };

  let ingress_script = if control_plane_type == "cloud" { SETUP_INGRESS_CLOUD_SCRIPT } else { SETUP_INGRESS_SCRIPT };
  let context_value: Value = serde_json::to_value(&ingress_context).unwrap();
  let tera_context: Context = Context::from_serialize(&context_value).unwrap();
  let rendered_script: String = tera::Tera::one_off(ingress_script, &tera_context, false).unwrap();

  let result: NodeCommandResult = run_on_node(&session, &rendered_script).await;
  match result {
    NodeCommandResult::Success => {},
    NodeCommandResult::Failure(err) => { 
      eprintln!("Ingress setup failed: {}", err);
      return;
    }
  }
}