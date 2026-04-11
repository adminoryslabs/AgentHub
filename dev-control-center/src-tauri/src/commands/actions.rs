use serde::Deserialize;
use std::process::Command;

use crate::commands::projects::load_projects;

// ============================================================================
// Environment Detection
// ============================================================================

fn is_wsl() -> bool {
    if std::env::var("WSL_DISTRO_NAME").is_ok() {
        return true;
    }
    if let Ok(content) = std::fs::read_to_string("/proc/version") {
        let lower = content.to_lowercase();
        if lower.contains("microsoft") || lower.contains("wsl") {
            return true;
        }
    }
    false
}

fn is_macos() -> bool {
    cfg!(target_os = "macos")
}

fn is_windows() -> bool {
    cfg!(target_os = "windows")
}

fn which(cmd: &str) -> bool {
    if is_windows() {
        Command::new("where")
            .arg(cmd)
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    } else {
        Command::new("which")
            .arg(cmd)
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }
}

// ============================================================================
// Open Editor (VSCode / Cursor)
// ============================================================================

fn get_editor_binary(editor: &str) -> String {
    match editor {
        "vscode" => "code".to_string(),
        "cursor" => "cursor".to_string(),
        _ => editor.to_string(),
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenEditorRequest {
    pub project_id: String,
    pub editor: String,
}

#[tauri::command]
pub async fn open_editor(req: OpenEditorRequest) -> Result<String, String> {
    let store = load_projects()?;
    let project_id = uuid::Uuid::parse_str(&req.project_id)
        .map_err(|_| format!("ID inválido: {}", req.project_id))?;

    let project = store
        .projects
        .iter()
        .find(|p| p.id == project_id)
        .ok_or_else(|| format!("Proyecto no encontrado: {}", req.project_id))?;

    let editor = get_editor_binary(&req.editor);

    // Validate editor exists in PATH
    if !which(&editor) {
        return Err(format!(
            "{} no encontrado en PATH. Instalalo primero.",
            editor
        ));
    }

    // Validate project path exists
    let path = std::path::Path::new(&project.path);
    if !path.exists() {
        return Err(format!("Ruta no encontrada: {}", project.path));
    }

    // Launch editor (GUI apps open their own window)
    // When running in WSL, `code` (VS Code Remote) works directly
    let result = if is_wsl() {
        // Already in WSL - direct launch works (VS Code Remote handles the rest)
        Command::new(&editor).arg(&project.path).spawn()
    } else if is_windows() && project.environment == "wsl" {
        // Windows native, project lives in WSL
        Command::new("wsl")
            .arg(&editor)
            .arg(&project.path)
            .spawn()
    } else if is_macos() {
        Command::new(&editor).arg(&project.path).spawn()
    } else {
        // Same environment
        Command::new(&editor).arg(&project.path).spawn()
    };

    result.map_err(|e| format!("No se pudo abrir {}: {}", editor, e))?;

    // Update lastOpenedAt
    update_last_opened(project_id)?;

    Ok(format!("{} opened in {}", req.editor, project.name))
}

// ============================================================================
// Launch Agent (Claude Code / OpenCode / QwenCode)
// ============================================================================

fn get_agent_binary(agent: &str) -> String {
    match agent {
        "claude" => "claude".to_string(),
        "opencode" => "opencode".to_string(),
        "qwen" => "qwen".to_string(),
        _ => agent.to_string(),
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LaunchAgentRequest {
    pub project_id: String,
    pub agent: String,
}

#[tauri::command]
pub async fn launch_agent(req: LaunchAgentRequest) -> Result<String, String> {
    let store = load_projects()?;
    let project_id = uuid::Uuid::parse_str(&req.project_id)
        .map_err(|_| format!("ID inválido: {}", req.project_id))?;

    let project = store
        .projects
        .iter()
        .find(|p| p.id == project_id)
        .ok_or_else(|| format!("Proyecto no encontrado: {}", req.project_id))?;

    let agent = get_agent_binary(&req.agent);

    // Validate agent exists in PATH
    if !which(&agent) {
        return Err(format!(
            "{} no encontrado en PATH. Instalalo primero.",
            agent
        ));
    }

    // Validate project path exists
    let path = std::path::Path::new(&project.path);
    if !path.exists() {
        return Err(format!("Ruta no encontrada: {}", project.path));
    }

    // Launch agent in a visible terminal
    launch_in_terminal(&agent, &project.path, &project.name)?;

    // Update lastOpenedAt
    update_last_opened(project_id)?;

    Ok(format!("{} launched for {}", req.agent, project.name))
}

/// Update the lastOpenedAt timestamp for a project
fn update_last_opened(project_id: uuid::Uuid) -> Result<(), String> {
    let mut store = load_projects()?;
    let project = store
        .projects
        .iter_mut()
        .find(|p| p.id == project_id)
        .ok_or_else(|| format!("Project not found: {}", project_id))?;

    project.last_opened_at = Some(chrono::Utc::now());
    crate::commands::projects::save_projects(&store)
}

/// Launch an agent command in a visible terminal window.
/// Handles WSL, Windows native, Mac, and Linux native.
fn launch_in_terminal(agent: &str, path: &str, name: &str) -> Result<(), String> {
    let cmd = format!("cd '{}' && exec {}", path, agent);
    let title = format!("{} - {}", agent, name);

    if is_wsl() {
        // Running inside WSL → open Windows Terminal via wt.exe
        // wt.exe is accessible from WSL and opens a new tab in Windows Terminal
        Command::new("wt.exe")
            .arg("--title")
            .arg(&title)
            .arg("--")
            .arg("bash")
            .arg("-ic")
            .arg(&cmd)
            .spawn()
            .map_err(|e| {
                format!(
                    "wt.exe failed: {}. Asegurate de tener Windows Terminal instalado.",
                    e
                )
            })?;
        Ok(())
    } else if is_windows() {
        // Windows native → use Windows Terminal with cmd.exe
        let win_path = path.replace('/', "\\");
        let win_cmd = format!("cd /d {} && {}", win_path, agent);
        Command::new("wt.exe")
            .arg("--title")
            .arg(&title)
            .arg("cmd.exe")
            .arg("/c")
            .arg(&win_cmd)
            .spawn()
            .map_err(|e| format!("wt.exe failed: {}", e))?;
        Ok(())
    } else if is_macos() {
        // Mac → use osascript to open Terminal.app
        let script = format!(
            "tell application \"Terminal\" to do script \"cd '{}' && exec {}\"",
            path, agent
        );
        Command::new("osascript")
            .arg("-e")
            .arg(&script)
            .spawn()
            .map_err(|e| format!("osascript failed: {}", e))?;
        Ok(())
    } else {
        // Linux native → try common terminals
        for terminal in &["gnome-terminal", "x-terminal-emulator", "konsole", "xterm"] {
            if which(terminal) {
                return match *terminal {
                    "gnome-terminal" => Command::new("gnome-terminal")
                        .arg("--")
                        .arg("bash")
                        .arg("-ic")
                        .arg(&cmd)
                        .spawn()
                        .map(|_| ())
                        .map_err(|e| format!("{} failed: {}", terminal, e)),
                    "konsole" => Command::new("konsole")
                        .arg("-e")
                        .arg("bash")
                        .arg("-ic")
                        .arg(&cmd)
                        .spawn()
                        .map(|_| ())
                        .map_err(|e| format!("{} failed: {}", terminal, e)),
                    _ => Command::new(terminal)
                        .arg("-e")
                        .arg(format!("bash -ic '{}'", cmd))
                        .spawn()
                        .map(|_| ())
                        .map_err(|e| format!("{} failed: {}", terminal, e)),
                };
            }
        }
        Err("No terminal emulator found. Install gnome-terminal, xterm, or similar.".to_string())
    }
}
