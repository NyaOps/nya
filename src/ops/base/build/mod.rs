use crate::core::service::{Service, ServiceActions, handle_action};
use crate::ops::base::build::control_plane::build_control_plane_action;
use crate::ops::base::build::prebuild::{prebuild_action, run_prebuild_script};
pub(crate) mod prebuild;
pub(crate) mod control_plane;


pub struct NyaBaseBuild;

impl Service for NyaBaseBuild {
  fn name(&self) -> String {"NyaBase".to_string()}
  fn register(&self) -> ServiceActions {
    vec![
      (String::from("onPreBuild"), handle_action(prebuild_action)),
      (String::from("runPreBuild"), handle_action(run_prebuild_script)),
      (String::from("onBuildControlPlane"), handle_action(build_control_plane_action))
    ]
  }
}