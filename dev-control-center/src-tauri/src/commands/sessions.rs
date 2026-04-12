use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::time::SystemTime;

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
        .replace(':', "-");

    // Unix paths start with / → becomes leading -, but we want a single leading -
    if path.starts_with('/') && !encoded.starts_with('-') {
        encoded.insert(0, '-');
    }

    // Collapse multiple consecutive dashes to single dash
    while encoded.contains("--") {
        encoded = encoded.replace("--", "-");
    }

    encoded
}

#[tauri::command]
pub async fn get_sessions(req: GetSessionsRequest) -> Result<Vec<SessionEntry>, String> {
    let mut sessions = Vec::new();

    // Discover Claude sessions
    if let Ok(claude) = discover_claude_sessions(&req.project_path) {
        sessions.extend(claude);
    }

    // Discover Qwen sessions
    if let Ok(qwen) = discover_qwen_sessions(&req.project_path) {
        sessions.extend(qwen);
    }

    // Sort by modified_at descending
    sessions.sort_by(|a, b| b.modified_at.cmp(&a.modified_at));

    Ok(sessions)
}

fn discover_claude_sessions(project_path: &str) -> Result<Vec<SessionEntry>, String> {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".to_string());

    let encoded = encode_project_path(project_path);
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

        // Claude stores sessions as .jsonl files
        if !path.is_file() {
            continue;
        }
        if path.extension().and_then(|e| e.to_str()) != Some("jsonl") {
            continue;
        }

        let metadata = path.metadata().map_err(|e| e.to_string())?;
        let modified = metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH);
        let size = metadata.len();

        let session_id = path
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_default();

        sessions.push(SessionEntry {
            agent: "claude".to_string(),
            session_id,
            modified_at: format_system_time(modified),
            size_bytes: size,
        });
    }

    Ok(sessions)
}

fn discover_qwen_sessions(project_path: &str) -> Result<Vec<SessionEntry>, String> {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".to_string());

    let encoded = encode_project_path(project_path);
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

        if !path.is_file() {
            continue;
        }
        if path.extension().and_then(|e| e.to_str()) != Some("jsonl") {
            continue;
        }

        let metadata = path.metadata().map_err(|e| e.to_string())?;
        let modified = metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH);
        let size = metadata.len();

        let session_id = path
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_default();

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
