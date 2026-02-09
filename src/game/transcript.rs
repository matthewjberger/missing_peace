use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::sync::OnceLock;

static TRANSCRIPT_PATH: OnceLock<PathBuf> = OnceLock::new();

pub fn init() {
    let path = std::env::current_dir()
        .unwrap_or_default()
        .join("simulation_transcript.txt");
    let _ = fs::write(&path, "");
    TRANSCRIPT_PATH.get_or_init(|| path);
}

pub fn log(line: &str) {
    if let Some(path) = TRANSCRIPT_PATH.get()
        && let Ok(mut file) = fs::OpenOptions::new().append(true).open(path)
    {
        let _ = writeln!(file, "{}", line);
    }
}

pub fn log_separator() {
    log("────────────────────────────────────────");
}
