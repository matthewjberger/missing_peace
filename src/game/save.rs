use super::state::GameState;
use std::fs;
use std::path::PathBuf;

fn save_directory() -> PathBuf {
    let mut path = dirs_next_or_home();
    path.push("missing_peace");
    path
}

fn dirs_next_or_home() -> PathBuf {
    if let Some(local_data) = std::env::var_os("LOCALAPPDATA") {
        return PathBuf::from(local_data);
    }
    if let Some(home) = std::env::var_os("HOME") {
        let mut path = PathBuf::from(home);
        path.push(".local");
        path.push("share");
        return path;
    }
    PathBuf::from(".")
}

fn save_path() -> PathBuf {
    let mut path = save_directory();
    path.push("save.json");
    path
}

pub fn save_game(game: &GameState) -> Result<(), String> {
    let dir = save_directory();
    fs::create_dir_all(&dir)
        .map_err(|error| format!("Failed to create save directory: {}", error))?;

    let json = serde_json::to_string_pretty(game)
        .map_err(|error| format!("Failed to serialize: {}", error))?;

    fs::write(save_path(), json).map_err(|error| format!("Failed to write save: {}", error))?;

    Ok(())
}

pub fn load_game() -> Option<GameState> {
    let path = save_path();
    if !path.exists() {
        return None;
    }

    let json = fs::read_to_string(path).ok()?;
    serde_json::from_str(&json).ok()
}

pub fn delete_save() {
    let path = save_path();
    if path.exists() {
        let _ = fs::remove_file(path);
    }
}

pub fn has_save() -> bool {
    save_path().exists()
}
