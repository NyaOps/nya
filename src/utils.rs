use std::{env, path::PathBuf, process::Stdio};

use tokio::process::Command;

use crate::defaults;

pub enum ConfigStatus {
    Exists(PathBuf),
    Missing(PathBuf),
}

pub fn resolve_base_config(user_input: Option<&str>) -> ConfigStatus {
    let path = if let Some(input) = user_input {
        let expanded = shellexpand::tilde(input).to_string();
        PathBuf::from(expanded)
    } else {
        let expanded = shellexpand::tilde(defaults::BASE_CONFIG_DEFAULT_LOCATION).to_string();
        PathBuf::from(expanded)
    };
    
    let final_path = if path.is_dir() {
        path.join(defaults::BASE_CONFIG_DEFAULT_FILE_NAME)
    } else {
        path
    };
    
    if !final_path.exists() {
        return ConfigStatus::Missing(final_path);
    }
    
    ConfigStatus::Exists(final_path)
}

pub fn resolve_capsule(user_input: Option<&str>) -> Option<PathBuf> {
    let path = if let Some(input) = user_input {
        let expanded = shellexpand::tilde(input).to_string();
        PathBuf::from(expanded)
    } else {
        env::current_dir().ok()?.join(".nya").join("nya.json")
    };
    
    if path.exists() {
        Some(path)
    } else {
        None  // Optional, so just return None
    }
}

pub fn generate_sha(location: &str) -> String {
    // Try git sha first
    let output = std::process::Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .current_dir(location)
        .output();
        
    if let Ok(out) = output {
        if out.status.success() {
            return String::from_utf8_lossy(&out.stdout).trim().to_string();
        }
    }
    
    // Fallback to timestamp-based sha
    use std::time::{SystemTime, UNIX_EPOCH};
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    format!("{:x}", ts)[..7].to_string()
}

pub async fn run_ssh(host: &str, user: &str, key: &str, cmd: &str) -> Result<(), String> {
    let mut command = Command::new("ssh");
    command
        .args([
            "-i", key,
            "-o", "StrictHostKeyChecking=no",
            "-o", "UserKnownHostsFile=/dev/null",
            "-o", "IdentitiesOnly=yes",
            &format!("{}@{}", user, host),
            cmd
        ])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let output = command.output().await.map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    Ok(())
}