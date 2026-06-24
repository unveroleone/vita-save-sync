use std::{fs, path::Path};

use crate::constant::{PSP_SAVE_DIR, RETROARCH_DIR};

#[derive(Debug, Clone, PartialEq)]
pub enum EmulatorKind {
    Psp,
    RetroArch,
}

#[derive(Debug, Clone)]
pub struct EmulatorEntry {
    pub id: String,
    pub name: String,
    pub source_path: String,
    pub kind: EmulatorKind,
}

pub fn scan_emulator_entries() -> Vec<EmulatorEntry> {
    let mut entries = Vec::new();

    // PSP/Adrenaline saves — one entry per subfolder in SAVEDATA
    if let Ok(dir) = fs::read_dir(PSP_SAVE_DIR) {
        let mut psp: Vec<EmulatorEntry> = dir
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_dir())
            .map(|e| {
                let folder = e.file_name().to_string_lossy().to_string();
                EmulatorEntry {
                    id: format!("PSP_{}", folder),
                    name: format!("PSP: {}", folder),
                    source_path: format!("{}/{}", PSP_SAVE_DIR, folder),
                    kind: EmulatorKind::Psp,
                }
            })
            .collect();
        psp.sort_by(|a, b| a.id.cmp(&b.id));
        entries.extend(psp);
    }

    // RetroArch — single entry covering savefiles, savestates, and configs
    if Path::new(RETROARCH_DIR).exists() {
        entries.push(EmulatorEntry {
            id: "RETROARCH".to_string(),
            name: "RetroArch".to_string(),
            source_path: RETROARCH_DIR.to_string(),
            kind: EmulatorKind::RetroArch,
        });
    }

    entries
}
