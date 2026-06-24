use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LocalManifest {
    pub updated_at: String,
    pub games: HashMap<String, GameSyncEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GameSyncEntry {
    pub title: String,
    pub local_hash: Option<String>,
    pub local_timestamp: Option<String>,
    pub cloud_hash: Option<String>,
    pub cloud_timestamp: Option<String>,
    pub last_synced_hash: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SyncStatus {
    InSync,
    UploadNeeded,
    DownloadAvailable,
    Conflict,
    LocalOnly,
    CloudOnly,
}

impl LocalManifest {
    pub fn load(path: &PathBuf) -> LocalManifest {
        if let Ok(data) = std::fs::read_to_string(path) {
            serde_json::from_str(&data).unwrap_or_default()
        } else {
            LocalManifest::default()
        }
    }

    pub fn save(&self, path: &PathBuf) {
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = std::fs::write(path, json);
        }
    }
}

pub fn compute_status(entry: &GameSyncEntry) -> SyncStatus {
    match (
        &entry.local_hash,
        &entry.cloud_hash,
        &entry.last_synced_hash,
    ) {
        (None, None, _) => SyncStatus::LocalOnly,
        (Some(_), None, _) => SyncStatus::LocalOnly,
        (None, Some(_), _) => SyncStatus::CloudOnly,
        (Some(local), Some(cloud), last_synced) => {
            if local == cloud {
                return SyncStatus::InSync;
            }
            let local_changed = last_synced.as_ref().map(|h| h != local).unwrap_or(true);
            let cloud_changed = last_synced.as_ref().map(|h| h != cloud).unwrap_or(true);
            match (local_changed, cloud_changed) {
                (true, false) => SyncStatus::UploadNeeded,
                (false, true) => SyncStatus::DownloadAvailable,
                (true, true) => SyncStatus::Conflict,
                (false, false) => SyncStatus::InSync,
            }
        }
    }
}
