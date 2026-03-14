use std::{env::{self, temp_dir}, process::Stdio};

use anyhow::Error;
use serde_json::{Value, to_string, to_string_pretty};
use tera::{Context, Tera};
use tokio::{io::{AsyncBufReadExt, BufReader}, process::Command};

use crate::{core::{payload::Payload, service::{Service, ServiceRegister, handle_function}}, embedded::{self, NYA_BACKEND_TEMPLATE, NYA_CHART, NYA_DEPLOYMENT_TEMPLATE, NYA_FRONTEND_TEMPLATE}, utils::run_ssh};
use crate::runtime::nya::Nya;
use std::fs;
use std::path::PathBuf;
use regex::Regex;
use tempfile::TempDir;

pub struct NyaBase;

impl Service for NyaBase {
  fn name(&self) -> String {"NyaBase".to_string()}
  fn register(&self) -> ServiceRegister {
    vec![
    ]
  }
}