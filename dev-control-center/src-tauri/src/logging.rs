use std::fs::OpenOptions;
use std::io::Write;
use std::time::SystemTime;

pub fn log_debug(msg: &str) {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".to_string());

    let log_dir = std::path::PathBuf::from(&home).join(".dev-control-center");
    let _ = std::fs::create_dir_all(&log_dir);

    let log_path = log_dir.join("debug.log");
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    if let Ok(mut f) = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)
    {
        let _ = writeln!(f, "[{}] {}", timestamp, msg);
    }
}
