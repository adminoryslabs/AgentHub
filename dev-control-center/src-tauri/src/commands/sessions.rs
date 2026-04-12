use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::SystemTime;

use crate::logging::log_debug;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionEntry {
    pub agent: String,
    pub session_id: String,
    pub modified_at: String,
    pub size_bytes: u64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSessionsRequest {
    pub project_path: String,
}

/// Encode a project path to the format used by Claude/Qwen.
///
/// Unix:  `/home/mario/AgentHub` → `-home-mario-AgentHub`
/// Win:   `D:/Belcorp/Projects/WS` → `D--Belcorp-Projects-WS`
fn encode_project_path(path: &str) -> String {
    let mut encoded = path
        .replace('/', "-")
        .replace('\\', "-")
        .replace(':', "--");

    // Unix paths start with / → becomes leading -
    if path.starts_with('/') && !encoded.starts_with('-') {
        encoded.insert(0, '-');
    }

    encoded
}

#[tauri::command]
pub async fn get_sessions(req: GetSessionsRequest) -> Result<Vec<SessionEntry>, String> {
    log_debug(&format!("[sessions] get_sessions for path='{}'", req.project_path));

    let mut sessions = Vec::new();

    // Discover Claude sessions
    match discover_claude_sessions(&req.project_path) {
        Ok(claude) => {
            log_debug(&format!("[sessions] claude sessions found: {}", claude.len()));
            sessions.extend(claude);
        }
        Err(e) => log_debug(&format!("[sessions] claude discovery error: {}", e)),
    }

    // Discover Qwen sessions
    match discover_qwen_sessions(&req.project_path) {
        Ok(qwen) => {
            log_debug(&format!("[sessions] qwen sessions found: {}", qwen.len()));
            sessions.extend(qwen);
        }
        Err(e) => log_debug(&format!("[sessions] qwen discovery error: {}", e)),
    }

    sessions.sort_by(|a, b| b.modified_at.cmp(&a.modified_at));

    log_debug(&format!("[sessions] total sessions returned: {}", sessions.len()));
    Ok(sessions)
}

/// Discover Claude sessions. If running on Windows with a WSL project path,
/// scan via `wsl ls` in the WSL filesystem.
fn discover_claude_sessions(project_path: &str) -> Result<Vec<SessionEntry>, String> {
    let encoded = encode_project_path(project_path);

    // If on Windows and project is WSL, scan via wsl
    if is_windows() && is_wsl_path(project_path) {
        return discover_claude_sessions_wsl(&encoded);
    }

    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".to_string());

    let claude_dir = PathBuf::from(&home)
        .join(".claude")
        .join("projects")
        .join(&encoded);

    log_debug(&format!("[sessions] claude dir: home='{}' encoded='{}' path='{}'", home, encoded, claude_dir.display()));

    if !claude_dir.exists() {
        log_debug(&format!("[sessions] claude dir does not exist: {}", claude_dir.display()));
        return Ok(Vec::new());
    }

    log_debug(&format!("[sessions] claude dir exists, scanning..."));

    let mut sessions = Vec::new();

    for entry in fs::read_dir(&claude_dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();

        if !path.is_file() { continue; }
        if path.extension().and_then(|e| e.to_str()) != Some("jsonl") { continue; }

        let metadata = path.metadata().map_err(|e| e.to_string())?;
        let modified = metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH);
        let size = metadata.len();
        let session_id = path.file_stem().map(|s| s.to_string_lossy().to_string()).unwrap_or_default();

        log_debug(&format!("[sessions] claude session: {} ({}B, {:?})", session_id, size, modified));

        sessions.push(SessionEntry {
            agent: "claude".to_string(),
            session_id,
            modified_at: format_system_time(modified),
            size_bytes: size,
        });
    }

    Ok(sessions)
}

/// Discover Claude sessions via WSL (from Windows).
fn discover_claude_sessions_wsl(encoded: &str) -> Result<Vec<SessionEntry>, String> {
    let wsl_path = format!("/home/{}/.claude/projects/{}", whoami(), encoded);
    log_debug(&format!("[sessions] claude wsl scan: {}", wsl_path));

    let output = Command::new("wsl")
        .args(["ls", "-1", "--time-style=full-iso", &wsl_path])
        .output()
        .map_err(|e| format!("wsl ls failed: {}", e))?;

    if !output.status.success() {
        log_debug(&format!("[sessions] claude wsl dir does not exist: {}", wsl_path));
        return Ok(Vec::new());
    }

    let mut sessions = Vec::new();

    for line in String::from_utf8_lossy(&output.stdout).lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 6 { continue; }

        let filename = parts.last().unwrap();
        if !filename.ends_with(".jsonl") { continue; }

        // Parse date and size from ls output
        // Format: permissions links owner group size date time filename
        let size_idx = 4;
        let date_idx = 5;
        let time_idx = 6;

        let size: u64 = parts.get(size_idx).and_then(|s| s.parse().ok()).unwrap_or(0);
        let date_str = format!("{} {}", parts.get(date_idx).unwrap_or(&""), parts.get(time_idx).unwrap_or(&""));
        let session_id = filename.trim_end_matches(".jsonl").to_string();

        // Try to parse the date, fallback to epoch
        let modified = chrono::NaiveDateTime::parse_from_str(&date_str, "%Y-%m-%d %H:%M:%S")
            .ok()
            .map(|dt| dt.and_utc())
            .unwrap_or(chrono::Utc::now())
            .into();

        log_debug(&format!("[sessions] claude wsl session: {} ({}B)", session_id, size));

        sessions.push(SessionEntry {
            agent: "claude".to_string(),
            session_id,
            modified_at: format_system_time(modified),
            size_bytes: size,
        });
    }

    Ok(sessions)
}

/// Discover Qwen sessions. If running on Windows with a WSL project path,
/// scan via `wsl ls` in the WSL filesystem.
fn discover_qwen_sessions(project_path: &str) -> Result<Vec<SessionEntry>, String> {
    let encoded = encode_project_path(project_path);

    // If on Windows and project is WSL, scan via wsl
    if is_windows() && is_wsl_path(project_path) {
        return discover_qwen_sessions_wsl(&encoded);
    }

    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".to_string());

    let chats_dir = PathBuf::from(&home)
        .join(".qwen")
        .join("projects")
        .join(&encoded)
        .join("chats");

    log_debug(&format!("[sessions] qwen chats dir: home='{}' encoded='{}' path='{}'", home, encoded, chats_dir.display()));

    if !chats_dir.exists() {
        log_debug(&format!("[sessions] qwen chats dir does not exist: {}", chats_dir.display()));
        return Ok(Vec::new());
    }

    log_debug(&format!("[sessions] qwen chats dir exists, scanning..."));

    let mut sessions = Vec::new();

    for entry in fs::read_dir(&chats_dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();

        if !path.is_file() { continue; }
        if path.extension().and_then(|e| e.to_str()) != Some("jsonl") { continue; }

        let metadata = path.metadata().map_err(|e| e.to_string())?;
        let modified = metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH);
        let size = metadata.len();
        let session_id = path.file_stem().map(|s| s.to_string_lossy().to_string()).unwrap_or_default();

        log_debug(&format!("[sessions] qwen session: {} ({}B, {:?})", session_id, size, modified));

        sessions.push(SessionEntry {
            agent: "qwen".to_string(),
            session_id,
            modified_at: format_system_time(modified),
            size_bytes: size,
        });
    }

    Ok(sessions)
}

/// Discover Qwen sessions via WSL (from Windows).
fn discover_qwen_sessions_wsl(encoded: &str) -> Result<Vec<SessionEntry>, String> {
    let wsl_path = format!("/home/{}/.qwen/projects/{}/chats", whoami(), encoded);
    log_debug(&format!("[sessions] qwen wsl scan: {}", wsl_path));

    let output = Command::new("wsl")
        .args(["ls", "-1", "--time-style=full-iso", &wsl_path])
        .output()
        .map_err(|e| format!("wsl ls failed: {}", e))?;

    if !output.status.success() {
        log_debug(&format!("[sessions] qwen wsl chats dir does not exist: {}", wsl_path));
        return Ok(Vec::new());
    }

    let mut sessions = Vec::new();

    for line in String::from_utf8_lossy(&output.stdout).lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 6 { continue; }

        let filename = parts.last().unwrap();
        if !filename.ends_with(".jsonl") { continue; }

        let size_idx = 4;
        let date_idx = 5;
        let time_idx = 6;

        let size: u64 = parts.get(size_idx).and_then(|s| s.parse().ok()).unwrap_or(0);
        let date_str = format!("{} {}", parts.get(date_idx).unwrap_or(&""), parts.get(time_idx).unwrap_or(&""));
        let session_id = filename.trim_end_matches(".jsonl").to_string();

        let modified = chrono::NaiveDateTime::parse_from_str(&date_str, "%Y-%m-%d %H:%M:%S")
            .ok()
            .map(|dt| dt.and_utc())
            .unwrap_or(chrono::Utc::now())
            .into();

        log_debug(&format!("[sessions] qwen wsl session: {} ({}B)", session_id, size));

        sessions.push(SessionEntry {
            agent: "qwen".to_string(),
            session_id,
            modified_at: format_system_time(modified),
            size_bytes: size,
        });
    }

    Ok(sessions)
}

fn format_system_time(time: SystemTime) -> String {
    use chrono::{DateTime, Utc};
    let datetime: DateTime<Utc> = time.into();
    datetime.to_rfc3339()
}

fn is_windows() -> bool {
    cfg!(target_os = "windows")
}

fn is_wsl_path(path: &str) -> bool {
    path.starts_with("/home/") || path.starts_with("/mnt/")
}

fn whoami() -> String {
    std::env::var("USER")
        .or_else(|_| std::env::var("USERNAME"))
        .unwrap_or_else(|_| "marioyahuar".to_string())
}
