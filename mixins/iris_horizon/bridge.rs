// Bridge between Iris Shaders and the Horizon client engine (Game, Window, Options).

use crate::game::Game;
use crate::game::iris_horizon as iris;
use std::cell::RefCell;
use std::path::PathBuf;

thread_local! {
    static SETTINGS: RefCell<iris::IrisSettings> = RefCell::new(iris::IrisSettings::default());
    static AVAILABLE_PACKS: RefCell<Vec<String>> = RefCell::new(Vec::new());
}

/// Returns the shaderpacks directory inside Horizon's installation folder
pub fn shaderpacks_dir() -> PathBuf {
    if let Ok(appdata) = std::env::var("APPDATA") {
        let p = PathBuf::from(appdata).join("horizon").join("shaderpacks");
        let _ = std::fs::create_dir_all(&p);
        p
    } else {
        let p = PathBuf::from("shaderpacks");
        let _ = std::fs::create_dir_all(&p);
        p
    }
}

/// Rescans available shaderpack directories and .zip files
pub fn scan_shaderpacks() {
    let mut packs = vec!["(OFF)".to_string()];
    let dir = shaderpacks_dir();
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() || path.extension().map_or(false, |ext| ext == "zip") {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    packs.push(name.to_string());
                }
            }
        }
    }
    AVAILABLE_PACKS.with(|p| *p.borrow_mut() = packs);
}

pub fn settings(_game: &mut Game<'_>) -> iris::IrisSettings {
    SETTINGS.with(|s| s.borrow().clone())
}

pub fn click(_game: &mut Game<'_>, button: usize) {
    if button == 1 {
        // Cycle through discovered shaderpacks
        scan_shaderpacks();
        AVAILABLE_PACKS.with(|packs_ref| {
            let packs = packs_ref.borrow();
            if packs.is_empty() {
                return;
            }
            SETTINGS.with(|s_ref| {
                let mut s = s_ref.borrow_mut();
                let current_idx = packs.iter().position(|p| p == &s.shader_pack).unwrap_or(0);
                let next_idx = (current_idx + 1) % packs.len();
                s.shader_pack = packs[next_idx].clone();
                s.enabled = s.shader_pack != "(OFF)";
            });
        });
    } else {
        SETTINGS.with(|s| s.borrow_mut().click(button));
    }
}

pub fn frame(_game: &mut Game<'_>) {
    // Per-frame graphics state synchronization
}

pub fn command(_game: &mut Game<'_>, command: &str) -> bool {
    let trimmed = command.trim();
    if trimmed == "/iris" || trimmed == "/shaders" {
        scan_shaderpacks();
        return true;
    }
    if let Some(rest) = trimmed.strip_prefix("/iris ") {
        if rest == "reload" {
            scan_shaderpacks();
            return true;
        }
    }
    false
}

pub fn hud(
    _game: &mut Game<'_>,
    _batch: &mut horizon_client::hud::HudBatch,
    _corners: &mut horizon_client::extras_hud::Corners,
) {
    // F3 debug HUD overlay integration
}
