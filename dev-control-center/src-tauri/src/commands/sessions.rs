use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::SystemTime;

use crate::commands::projects::normalize_path_for_storage;
use crate::logging::log_debug;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionEntry {
    pub agent: String,
    pub session_id: String,
    pub title: String,
    pub modified_at: String,
    pub size_bytes: u64,
}

#[derive(Debug, Clone)]
enum SessionLocation {
    Local(PathBuf),
    Wsl(String),
}

#[derive(Debug, Clone)]
struct SessionCandidate {
    session_id: String,
    location: SessionLocation,
    modified: SystemTime,
    size_bytes: u64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSessionsRequest {
    pub project_path: String,
}

/// Encode a project path to the format used by Claude/Qwen.
///
/// Unix:  `/home/mario/AgentHub` → `-home-mario-AgentHub`
/// Win:   `D:\Belcorp\Projects\WS` → `D--Belcorp-Projects-WS`
///
/// The key insight: Windows drive paths like `D:\path` encode as `D--path`
/// (the `:\` becomes `--`, remaining `\` become `-`).
fn encode_project_path(path: &str) -> String {
    // First handle Windows drive paths: D:\path → D--path
    // Detect Windows drive pattern (letter followed by :\)
    if path.len() >= 3 && path.chars().nth(1) == Some(':') && (path.chars().nth(2) == Some('\\') || path.chars().nth(2) == Some('/')) {
        // Replace the drive colon + separator with --
        let rest = &path[2..]; // \Belcorp\Projects or /Belcorp/Projects
        let encoded_rest = rest.replace('\\', "-").replace('/', "-");
        // path[0] is the drive letter, then --, then rest
        return format!("{}--{}", &path[0..1], &encoded_rest[1..]); // skip leading \ or /
    }

    // Unix paths: /home/mario/AgentHub → -home-mario-AgentHub
    let mut encoded = path.replace('/', "-");

    // Unix paths start with / → becomes leading -
    if path.starts_with('/') && !encoded.starts_with('-') {
        encoded.insert(0, '-');
    }

    encoded
}

fn normalize_lookup_path(path: &str) -> String {
    let trimmed = path.trim();
    if trimmed.is_empty() {
        log_debug("[sessions] normalize_lookup_path: empty input");
        return String::new();
    }

    if is_wsl_path(trimmed) {
        let normalized = normalize_path_for_storage(trimmed, "wsl")
            .unwrap_or_else(|err| {
                log_debug(&format!(
                    "[sessions] normalize_lookup_path: WSL normalization failed for '{}' error='{}'",
                    trimmed, err
                ));
                trimmed.to_string()
            });
        log_debug(&format!(
            "[sessions] normalize_lookup_path: WSL '{}' -> '{}'",
            trimmed, normalized
        ));
        return normalized;
    }

    log_debug(&format!(
        "[sessions] normalize_lookup_path: non-WSL '{}' kept as-is",
        trimmed
    ));
    trimmed.to_string()
}

fn build_session_entry(agent: &str, candidate: SessionCandidate) -> Option<SessionEntry> {
    let title = derive_session_title(agent, &candidate)?;

    Some(SessionEntry {
        agent: agent.to_string(),
        session_id: candidate.session_id,
        title,
        modified_at: format_system_time(candidate.modified),
        size_bytes: candidate.size_bytes,
    })
}

#[tauri::command]
pub async fn get_sessions(req: GetSessionsRequest) -> Result<Vec<SessionEntry>, String> {
    let lookup_path = normalize_lookup_path(&req.project_path);
    log_debug(&format!(
        "[sessions] get_sessions for path='{}' normalized='{}'",
        req.project_path, lookup_path
    ));

    let mut sessions = Vec::new();

    // Discover Claude sessions
    match discover_claude_sessions(&lookup_path) {
        Ok(claude) => {
            log_debug(&format!("[sessions] claude sessions found: {}", claude.len()));
            sessions.extend(claude);
        }
        Err(e) => log_debug(&format!("[sessions] claude discovery error: {}", e)),
    }

    // Discover Qwen sessions
    match discover_qwen_sessions(&lookup_path) {
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
    log_debug(&format!(
        "[sessions] discover_claude_sessions path='{}' encoded='{}' is_windows={} is_wsl_path={}",
        project_path,
        encoded,
        is_windows(),
        is_wsl_path(project_path)
    ));

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

        if let Some(session) = build_session_entry("claude", SessionCandidate {
            session_id,
            location: SessionLocation::Local(path),
            modified,
            size_bytes: size,
        }) {
            sessions.push(session);
        }
    }

    Ok(sessions)
}

/// Discover Claude sessions via WSL (from Windows).
fn discover_claude_sessions_wsl(encoded: &str) -> Result<Vec<SessionEntry>, String> {
    let username = whoami();
    let wsl_path = format!("/home/{}/.claude/projects/{}", username, encoded);
    log_debug(&format!(
        "[sessions] claude wsl scan username='{}' encoded='{}' path='{}'",
        username, encoded, wsl_path
    ));

    // Check if directory exists first
    let dir_check = Command::new("wsl")
        .args(["test", "-d", &wsl_path])
        .output()
        .map_err(|e| format!("wsl test failed: {}", e))?;

    log_debug(&format!(
        "[sessions] claude wsl dir check status={} stderr='{}'",
        dir_check.status.success(),
        String::from_utf8_lossy(&dir_check.stderr).trim()
    ));

    if !dir_check.status.success() {
        log_debug(&format!("[sessions] claude wsl dir does not exist: {}", wsl_path));
        return Ok(Vec::new());
    }

    // Get file metadata directly from WSL so ordering stays correct.
    let list_script = "shopt -s nullglob; for file in \"$1\"/*.jsonl; do [ -f \"$file\" ] || continue; stat -c '%Y\t%s\t%n' -- \"$file\"; done";
    let output = Command::new("wsl")
        .args(["bash", "-lc", list_script, "_", &wsl_path])
        .output()
        .map_err(|e| format!("wsl metadata scan failed: {}", e))?;

    log_debug(&format!(
        "[sessions] claude wsl metadata scan status={} stdout_lines={} stderr='{}'",
        output.status.success(),
        String::from_utf8_lossy(&output.stdout).lines().count(),
        String::from_utf8_lossy(&output.stderr).trim()
    ));

    if !output.status.success() {
        log_debug("[sessions] claude wsl metadata scan failed");
        return Ok(Vec::new());
    }

    let mut sessions = Vec::new();

    for line in String::from_utf8_lossy(&output.stdout).lines() {
        let Some(candidate) = parse_wsl_metadata_line(line) else { continue; };

        log_debug(&format!("[sessions] claude wsl session: {}", candidate.session_id));
        if let Some(session) = build_session_entry("claude", candidate) {
            sessions.push(session);
        }
    }

    Ok(sessions)
}

/// Discover Qwen sessions. If running on Windows with a WSL project path,
/// scan via `wsl ls` in the WSL filesystem.
fn discover_qwen_sessions(project_path: &str) -> Result<Vec<SessionEntry>, String> {
    let encoded = encode_project_path(project_path);
    log_debug(&format!(
        "[sessions] discover_qwen_sessions path='{}' encoded='{}' is_windows={} is_wsl_path={}",
        project_path,
        encoded,
        is_windows(),
        is_wsl_path(project_path)
    ));

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

        if let Some(session) = build_session_entry("qwen", SessionCandidate {
            session_id,
            location: SessionLocation::Local(path),
            modified,
            size_bytes: size,
        }) {
            sessions.push(session);
        }
    }

    Ok(sessions)
}

/// Discover Qwen sessions via WSL (from Windows).
fn discover_qwen_sessions_wsl(encoded: &str) -> Result<Vec<SessionEntry>, String> {
    let username = whoami();
    let wsl_path = format!("/home/{}/.qwen/projects/{}/chats", username, encoded);
    log_debug(&format!(
        "[sessions] qwen wsl scan username='{}' encoded='{}' path='{}'",
        username, encoded, wsl_path
    ));

    // Check if directory exists first
    let dir_check = Command::new("wsl")
        .args(["test", "-d", &wsl_path])
        .output()
        .map_err(|e| format!("wsl test failed: {}", e))?;

    log_debug(&format!(
        "[sessions] qwen wsl dir check status={} stderr='{}'",
        dir_check.status.success(),
        String::from_utf8_lossy(&dir_check.stderr).trim()
    ));

    if !dir_check.status.success() {
        log_debug(&format!("[sessions] qwen wsl chats dir does not exist: {}", wsl_path));
        return Ok(Vec::new());
    }

    // Get file metadata directly from WSL so ordering stays correct.
    let list_script = "shopt -s nullglob; for file in \"$1\"/*.jsonl; do [ -f \"$file\" ] || continue; stat -c '%Y\t%s\t%n' -- \"$file\"; done";
    let output = Command::new("wsl")
        .args(["bash", "-lc", list_script, "_", &wsl_path])
        .output()
        .map_err(|e| format!("wsl metadata scan failed: {}", e))?;

    log_debug(&format!(
        "[sessions] qwen wsl metadata scan status={} stdout_lines={} stderr='{}'",
        output.status.success(),
        String::from_utf8_lossy(&output.stdout).lines().count(),
        String::from_utf8_lossy(&output.stderr).trim()
    ));

    if !output.status.success() {
        log_debug("[sessions] qwen wsl metadata scan failed");
        return Ok(Vec::new());
    }

    let mut sessions = Vec::new();

    for line in String::from_utf8_lossy(&output.stdout).lines() {
        let Some(candidate) = parse_wsl_metadata_line(line) else { continue; };

        log_debug(&format!("[sessions] qwen wsl session: {}", candidate.session_id));
        if let Some(session) = build_session_entry("qwen", candidate) {
            sessions.push(session);
        }
    }

    Ok(sessions)
}

fn format_system_time(time: SystemTime) -> String {
    use chrono::{DateTime, Utc};
    let datetime: DateTime<Utc> = time.into();
    datetime.to_rfc3339()
}

fn parse_wsl_metadata_line(line: &str) -> Option<SessionCandidate> {
    let mut parts = line.splitn(3, '\t');
    let modified_epoch = parts.next()?.trim().parse::<i64>().ok()?;
    let size_bytes = parts.next()?.trim().parse::<u64>().ok()?;
    let path = parts.next()?.trim().to_string();

    let session_id = PathBuf::from(&path)
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())?;

    Some(SessionCandidate {
        session_id,
        location: SessionLocation::Wsl(path),
        modified: system_time_from_unix_epoch(modified_epoch),
        size_bytes,
    })
}

fn system_time_from_unix_epoch(epoch_seconds: i64) -> SystemTime {
    if epoch_seconds <= 0 {
        return SystemTime::UNIX_EPOCH;
    }

    SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(epoch_seconds as u64)
}

fn derive_session_title(agent: &str, candidate: &SessionCandidate) -> Option<String> {
    let lines = match &candidate.location {
        SessionLocation::Local(path) => read_local_prefix_lines(path, 24).ok()?,
        SessionLocation::Wsl(path) => read_wsl_prefix_lines(path, 24).ok()?,
    };

    extract_title_from_lines(agent, &lines)
}

fn read_local_prefix_lines(path: &PathBuf, max_lines: usize) -> Result<Vec<String>, String> {
    let file = File::open(path).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);

    reader
        .lines()
        .take(max_lines)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())
}

fn read_wsl_prefix_lines(path: &str, max_lines: usize) -> Result<Vec<String>, String> {
    let head_script = format!("head -n {} -- \"$1\"", max_lines);

    let output = Command::new("wsl")
        .args(["bash", "-lc", &head_script, "_", path])
        .output()
        .map_err(|e| format!("wsl head failed: {}", e))?;

    if !output.status.success() {
        log_debug(&format!(
            "[sessions] wsl head failed path='{}' stderr='{}'",
            path,
            String::from_utf8_lossy(&output.stderr).trim()
        ));
        return Err("wsl head failed".to_string());
    }

    Ok(String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|line| line.to_string())
        .collect())
}

fn extract_title_from_lines(agent: &str, lines: &[String]) -> Option<String> {
    for line in lines {
        let Ok(value) = serde_json::from_str::<Value>(line) else {
            continue;
        };
        if value.get("type").and_then(Value::as_str) != Some("user") {
            continue;
        }

        let text = match agent {
            "claude" => extract_claude_user_text(&value),
            "qwen" => extract_qwen_user_text(&value),
            _ => None,
        };

        if let Some(text) = text.and_then(normalize_session_title) {
            return Some(text);
        }
    }

    None
}

fn extract_claude_user_text(value: &Value) -> Option<String> {
    let message = value.get("message")?;

    if let Some(content) = message.get("content").and_then(Value::as_str) {
        return Some(content.to_string());
    }

    extract_text_from_parts(message.get("content")?)
}

fn extract_qwen_user_text(value: &Value) -> Option<String> {
    let message = value.get("message")?;
    extract_text_from_parts(message.get("parts")?)
}

fn extract_text_from_parts(value: &Value) -> Option<String> {
    let parts = value.as_array()?;

    for part in parts {
        if let Some(text) = part.get("text").and_then(Value::as_str) {
            if !text.trim().is_empty() {
                return Some(text.to_string());
            }
        }
    }

    None
}

fn normalize_session_title(raw: String) -> Option<String> {
    let cleaned = raw
        .replace("\r", " ")
        .replace("\n", " ")
        .split_whitespace()
        .map(|part| part.trim_start_matches('>'))
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
        .trim()
        .to_string();

    if cleaned.is_empty() {
        return None;
    }

    let mut title = cleaned;
    if title.len() > 96 {
        title.truncate(93);
        title.push_str("...");
    }

    Some(title)
}

fn is_windows() -> bool {
    cfg!(target_os = "windows")
}

fn is_wsl_path(path: &str) -> bool {
    path.starts_with("/home/")
        || path.starts_with("/mnt/")
        || path.starts_with(r"\\wsl.localhost\")
        || path.starts_with(r"\\wsl$\")
        || path.starts_with(r"\wsl.localhost\")
        || path.starts_with(r"\wsl$\")
}

fn whoami() -> String {
    // Try WSL username first (when running on Windows but scanning WSL filesystem)
    if let Ok(output) = std::process::Command::new("wsl").args(["whoami"]).output() {
        log_debug(&format!(
            "[sessions] wsl whoami status={} stdout='{}' stderr='{}'",
            output.status.success(),
            String::from_utf8_lossy(&output.stdout).trim(),
            String::from_utf8_lossy(&output.stderr).trim()
        ));
        if output.status.success() {
            let name = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !name.is_empty() {
                return name;
            }
        }
    }
    // Fallback to environment variables
    std::env::var("USER")
        .or_else(|_| std::env::var("USERNAME"))
        .unwrap_or_else(|_| "marioyahuar".to_string())
}
