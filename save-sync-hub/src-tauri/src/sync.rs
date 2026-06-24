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
    /// Hash of local directory at the time of last sync.
    pub last_synced_hash: Option<String>,
    /// Hash of cloud zip at the time of last sync.
    pub last_synced_cloud_hash: Option<String>,
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
    ) {
        (None, None) => SyncStatus::LocalOnly,
        (Some(_), None) => SyncStatus::LocalOnly,
        (None, Some(_)) => SyncStatus::CloudOnly,
        (Some(local), Some(_cloud)) => {
            // Use separate reference hashes because local (dir hash) and
            // cloud (zip hash) are computed differently and never equal.
            let local_changed = entry
                .last_synced_hash
                .as_ref()
                .map(|h| h != local)
                .unwrap_or(true);
            let cloud_changed = entry
                .last_synced_cloud_hash
                .as_ref()
                .map(|h| h != _cloud)
                .unwrap_or(true);
            match (local_changed, cloud_changed) {
                (false, false) => SyncStatus::InSync,
                (true, false) => SyncStatus::UploadNeeded,
                (false, true) => SyncStatus::DownloadAvailable,
                (true, true) => SyncStatus::Conflict,
            }
        }
    }
}
