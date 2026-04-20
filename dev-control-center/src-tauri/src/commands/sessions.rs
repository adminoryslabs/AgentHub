use serde::{Deserialize, Serialize};
use serde_json::Value;
use rusqlite::{params, Connection, OpenFlags};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::fs;
use std::path::PathBuf;
use std::time::SystemTime;

use crate::commands::projects::{normalize_path_for_storage, resolve_filesystem_path};
use crate::logging::log_debug;
use crate::process::hidden_command;

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
        return String::new();
    }

    if is_wsl_path(trimmed) {
        return normalize_path_for_storage(trimmed, "wsl").unwrap_or_else(|_| trimmed.to_string());
    }

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

    let mut sessions = Vec::new();

    // Discover Claude sessions
    match discover_claude_sessions(&lookup_path) {
        Ok(claude) => {
            sessions.extend(claude);
        }
        Err(e) => log_debug(&format!("[sessions] claude discovery error: {}", e)),
    }

    // Discover Qwen sessions
    match discover_qwen_sessions(&lookup_path) {
        Ok(qwen) => {
            sessions.extend(qwen);
        }
        Err(e) => log_debug(&format!("[sessions] qwen discovery error: {}", e)),
    }

    match discover_opencode_sessions(&lookup_path) {
        Ok(opencode) => {
            sessions.extend(opencode);
        }
        Err(e) => log_debug(&format!("[sessions] opencode discovery error: {}", e)),
    }

    sessions.sort_by(|a, b| b.modified_at.cmp(&a.modified_at));

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

    if !claude_dir.exists() {
        return Ok(Vec::new());
    }

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

    // Check if directory exists first
    let dir_check = hidden_command("wsl")
        .args(["test", "-d", &wsl_path])
        .output()
        .map_err(|e| format!("wsl test failed: {}", e))?;

    if !dir_check.status.success() {
        return Ok(Vec::new());
    }

    collect_wsl_sessions("claude", &wsl_path)
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

    if !chats_dir.exists() {
        return Ok(Vec::new());
    }

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

    // Check if directory exists first
    let dir_check = hidden_command("wsl")
        .args(["test", "-d", &wsl_path])
        .output()
        .map_err(|e| format!("wsl test failed: {}", e))?;

    if !dir_check.status.success() {
        return Ok(Vec::new());
    }

    collect_wsl_sessions("qwen", &wsl_path)
}

fn discover_opencode_sessions(project_path: &str) -> Result<Vec<SessionEntry>, String> {
    if is_windows() && is_wsl_path(project_path) {
        return discover_opencode_sessions_wsl(project_path);
    }

    let db_path = opencode_db_path(project_path)?;
    if !std::path::Path::new(&db_path).exists() {
        return Ok(Vec::new());
    }

    let conn = Connection::open_with_flags(
        &db_path,
        OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX,
    )
    .map_err(|e| format!("No se pudo abrir opencode.db: {}", e))?;

    let mut stmt = conn
        .prepare(
            "
            SELECT
                s.id,
                s.title,
                s.time_updated
            FROM session s
            JOIN project p ON p.id = s.project_id
            WHERE s.time_archived IS NULL
              AND (p.worktree = ?1 OR s.directory = ?1)
            ORDER BY s.time_updated DESC
            ",
        )
        .map_err(|e| format!("No se pudo preparar query de OpenCode: {}", e))?;

    let rows = stmt
        .query_map(params![project_path], |row| {
            let session_id: String = row.get(0)?;
            let title: String = row.get(1)?;
            let updated_ms: i64 = row.get(2)?;

            Ok(SessionEntry {
                agent: "opencode".to_string(),
                session_id,
                title,
                modified_at: format_unix_millis(updated_ms),
                size_bytes: 0,
            })
        })
        .map_err(|e| format!("No se pudo leer sesiones de OpenCode: {}", e))?;

    let mut sessions = Vec::new();
    for row in rows {
        let mut session = row.map_err(|e| format!("Fila inválida en sesiones de OpenCode: {}", e))?;
        if let Some(title) = normalize_session_title(session.title.clone()) {
            session.title = title;
            sessions.push(session);
        }
    }

    Ok(sessions)
}

fn discover_opencode_sessions_wsl(project_path: &str) -> Result<Vec<SessionEntry>, String> {
    let username = whoami();
    let db_path = format!("/home/{}/.local/share/opencode/opencode.db", username);
    let script = r#"
import json
import sqlite3
import sys

db_path = sys.argv[1]
project_path = sys.argv[2]

conn = sqlite3.connect(f'file:{db_path}?mode=ro', uri=True)
cur = conn.cursor()
rows = cur.execute('''
    SELECT s.id, s.title, s.time_updated
    FROM session s
    JOIN project p ON p.id = s.project_id
    WHERE s.time_archived IS NULL
      AND (p.worktree = ? OR s.directory = ?)
    ORDER BY s.time_updated DESC
''', (project_path, project_path)).fetchall()

for session_id, title, time_updated in rows:
    print(json.dumps({
        'id': session_id,
        'title': title,
        'time_updated': time_updated,
    }, ensure_ascii=False))
"#;

    let output = hidden_command("wsl")
        .args(["python3", "-c", script, &db_path, project_path])
        .output()
        .map_err(|e| format!("No se pudo consultar sesiones de OpenCode en WSL: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Err(if stderr.is_empty() {
            "Consulta OpenCode WSL falló".to_string()
        } else {
            stderr
        });
    }

    let mut sessions = Vec::new();
    for line in String::from_utf8_lossy(&output.stdout).lines() {
        let Ok(value) = serde_json::from_str::<Value>(line) else {
            continue;
        };

        let Some(session_id) = value.get("id").and_then(Value::as_str) else {
            continue;
        };
        let Some(title_raw) = value.get("title").and_then(Value::as_str) else {
            continue;
        };
        let Some(updated_ms) = value.get("time_updated").and_then(Value::as_i64) else {
            continue;
        };
        let Some(title) = normalize_session_title(title_raw.to_string()) else {
            continue;
        };

        sessions.push(SessionEntry {
            agent: "opencode".to_string(),
            session_id: session_id.to_string(),
            title,
            modified_at: format_unix_millis(updated_ms),
            size_bytes: 0,
        });
    }

    Ok(sessions)
}

fn format_system_time(time: SystemTime) -> String {
    use chrono::{DateTime, Utc};
    let datetime: DateTime<Utc> = time.into();
    datetime.to_rfc3339()
}

fn format_unix_millis(timestamp_ms: i64) -> String {
    use chrono::{TimeZone, Utc};

    Utc.timestamp_millis_opt(timestamp_ms)
        .single()
        .unwrap_or_else(|| Utc.timestamp_opt(0, 0).single().expect("unix epoch should exist"))
        .to_rfc3339()
}

fn opencode_db_path(project_path: &str) -> Result<String, String> {
    let username = whoami();
    let wsl_db_path = format!("/home/{}/.local/share/opencode/opencode.db", username);

    if is_windows() && is_wsl_path(project_path) {
        return resolve_filesystem_path(&wsl_db_path, "wsl");
    }

    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".to_string());
    Ok(PathBuf::from(home)
        .join(".local")
        .join("share")
        .join("opencode")
        .join("opencode.db")
        .to_string_lossy()
        .to_string())
}

fn collect_wsl_sessions(agent: &str, directory: &str) -> Result<Vec<SessionEntry>, String> {
    let list_output = hidden_command("wsl")
        .args(["ls", "-1", directory])
        .output()
        .map_err(|e| format!("wsl ls failed: {}", e))?;

    if !list_output.status.success() {
        log_debug(&format!("[sessions] {} wsl ls failed: {}", agent, String::from_utf8_lossy(&list_output.stderr).trim()));
        return Ok(Vec::new());
    }

    let mut sessions = Vec::new();

    for filename in String::from_utf8_lossy(&list_output.stdout).lines() {
        let filename = filename.trim();
        if !filename.ends_with(".jsonl") {
            continue;
        }

        let file_path = format!("{}/{}", directory.trim_end_matches('/'), filename);
        let stat_output = hidden_command("wsl")
            .args(["stat", "-c", "%Y\t%s\t%n", "--", &file_path])
            .output()
            .map_err(|e| format!("wsl stat failed: {}", e))?;

        if !stat_output.status.success() {
            continue;
        }

        let stat_stdout = String::from_utf8_lossy(&stat_output.stdout).to_string();
        let Some(line) = stat_stdout.lines().next() else {
            continue;
        };

        let Some(candidate) = parse_wsl_metadata_line(line) else {
            continue;
        };

        if let Some(session) = build_session_entry(agent, candidate) {
            sessions.push(session);
        }
    }

    Ok(sessions)
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
    let max_lines_arg = max_lines.to_string();

    let output = hidden_command("wsl")
        .args(["head", "-n", &max_lines_arg, "--", path])
        .output()
        .map_err(|e| format!("wsl head failed: {}", e))?;

    if !output.status.success() {
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
    if let Ok(output) = hidden_command("wsl").args(["whoami"]).output() {
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
