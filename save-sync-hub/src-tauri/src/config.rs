use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncConfig {
    pub server_url: String,
    pub api_token: String,
    pub device_name: String,
    #[serde(default)]
    pub sources: Vec<SourceConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceConfig {
    pub id: String,
    pub platform: String,
    #[serde(rename = "customPath")]
    pub custom_path: String,
    pub label: String,
}

impl Default for SyncConfig {
    fn default() -> Self {
        Self {
            server_url: String::new(),
            api_token: String::new(),
            device_name: hostname(),
            sources: Vec::new(),
        }
    }
}

fn hostname() -> String {
    std::env::var("COMPUTERNAME")
        .or_else(|_| std::env::var("HOSTNAME"))
        .or_else(|_| std::env::var("USER"))
        .unwrap_or_else(|_| "unknown".to_string())
}

fn config_path() -> PathBuf {
    dirs_config().join("save-sync-hub").join("config.json")
}

fn dirs_config() -> PathBuf {
    #[cfg(target_os = "macos")]
    {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        PathBuf::from(home).join("Library").join("Application Support")
    }
    #[cfg(target_os = "linux")]
    {
        let config = std::env::var("XDG_CONFIG_HOME").unwrap_or_else(|_| {
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
            format!("{}/.config", home)
        });
        PathBuf::from(config)
    }
    #[cfg(target_os = "windows")]
    {
        let appdata = std::env::var("APPDATA").unwrap_or_else(|_| ".".to_string());
        PathBuf::from(appdata)
    }
    #[cfg(target_os = "android")]
    {
        PathBuf::from("/data/data/com.unveroleone.save-sync-hub")
    }
}

pub fn load_config() -> SyncConfig {
    let path = config_path();
    if path.exists() {
        if let Ok(data) = std::fs::read_to_string(&path) {
            return serde_json::from_str(&data).unwrap_or_default();
        }
    }
    SyncConfig::default()
}

pub fn save_config(config: &SyncConfig) -> Result<(), String> {
    let path = config_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| format!("Create config dir failed: {}", e))?;
    }
    let json = serde_json::to_string_pretty(config).map_err(|e| format!("Serialize failed: {}", e))?;
    std::fs::write(&path, json).map_err(|e| format!("Write config failed: {}", e))?;
    Ok(())
}
