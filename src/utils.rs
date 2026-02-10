use std::{env, path::PathBuf};

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

pub fn resolve_capsule_context(user_input: Option<&str>) -> Option<PathBuf> {
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