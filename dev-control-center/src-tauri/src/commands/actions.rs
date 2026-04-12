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

/// Check if a command exists in PATH.
/// On Windows, also checks .cmd variants and common install locations.
fn which(cmd: &str) -> bool {
    if is_windows() {
        // Check the command directly
        if Command::new("where")
            .arg(cmd)
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
        {
            return true;
        }
        // Check .cmd variant (e.g., code.cmd for VS Code)
        if Command::new("where")
            .arg(format!("{}.cmd", cmd))
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
        {
            return true;
        }
        // Check .exe variant
        if Command::new("where")
            .arg(format!("{}.exe", cmd))
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
        {
            return true;
        }
        // Fallback: check common install locations
        if command_exists_at(&resolve_command_path(cmd)) {
            return true;
        }
        false
    } else {
        Command::new("which")
            .arg(cmd)
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }
}

/// Check if a command exists inside WSL (from Windows).
fn which_wsl(cmd: &str) -> bool {
    Command::new("wsl")
        .args(["which", cmd])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Resolve common install paths for commands on Windows
fn resolve_command_path(cmd: &str) -> Option<String> {
    if let Ok(localappdata) = std::env::var("LOCALAPPDATA") {
        match cmd {
            "code" => {
                let path = format!(
                    "{}\\Programs\\Microsoft VS Code\\bin\\code.cmd",
                    localappdata
                );
                if std::path::Path::new(&path).exists() {
                    return Some(path);
                }
                // Also check Program Files
                if let Ok(programfiles) = std::env::var("ProgramFiles") {
                    let path = format!(
                        "{}\\Microsoft VS Code\\bin\\code.cmd",
                        programfiles
                    );
                    if std::path::Path::new(&path).exists() {
                        return Some(path);
                    }
                }
            }
            "cursor" => {
                let path = format!("{}\\Programs\\cursor\\cursor.exe", localappdata);
                if std::path::Path::new(&path).exists() {
                    return Some(path);
                }
            }
            _ => {}
        }
    }
    None
}

fn command_exists_at(path: &Option<String>) -> bool {
    path.as_ref().map(|p| std::path::Path::new(p).exists()).unwrap_or(false)
}

/// Check if a path exists, handling cross-environment cases
fn path_exists(path: &str, env: &str) -> bool {
    if is_windows() && env == "wsl" {
        // Running on Windows, project lives in WSL — validate via wsl
        Command::new("wsl")
            .args(["test", "-d", path])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    } else {
        std::path::Path::new(path).exists()
    }
}

// ============================================================================
// Open Editor (VSCode / Cursor)
// ============================================================================

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

    let editor = &req.editor;

    // Validate editor exists
    let editor_available = if is_windows() && project.environment == "wsl" {
        // Project is in WSL — editor validation doesn't matter as much
        // since `wsl code` will work if code is installed in WSL
        true // trust it; wsl will error if not found
    } else {
        which(editor) || (is_windows() && editor == "vscode" && resolve_command_path("code").is_some())
    };

    if !editor_available {
        let hint = match editor.as_str() {
            "vscode" if is_windows() => "VS Code no encontrado en PATH. Asegurate de instalar VS Code con 'Add to PATH' o reinstálalo marcando esa opción.",
            "cursor" if is_windows() => "Cursor no encontrado en PATH.",
            _ => return Err(format!("{} no encontrado en PATH. Instalalo primero.", editor)),
        };
        return Err(hint.to_string());
    }

    // Validate project path exists
    if !path_exists(&project.path, &project.environment) {
        return Err(format!("Ruta no encontrada: {}", project.path));
    }

    // Launch editor
    let result = if is_wsl() {
        Command::new(editor).arg(&project.path).spawn()
    } else if is_windows() && project.environment == "wsl" {
        Command::new("wsl").arg(editor).arg(&project.path).spawn()
    } else if is_macos() {
        Command::new(editor).arg(&project.path).spawn()
    } else {
        Command::new(editor).arg(&project.path).spawn()
    };

    result.map_err(|e| format!("No se pudo abrir {}: {}", editor, e))?;

    // Update lastOpenedAt
    update_last_opened(project_id)?;

    Ok(format!("{} opened in {}", req.editor, project.name))
}

// ============================================================================
// Launch Agent (Claude Code / OpenCode / QwenCode)
// ============================================================================

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

    let agent = &req.agent;

    // Validate agent exists — check in the right environment
    let agent_available = if is_windows() && project.environment == "wsl" {
        // Project is in WSL — agent must exist in WSL, not Windows
        which_wsl(agent)
    } else {
        which(agent)
    };

    if !agent_available {
        let hint = if is_windows() && project.environment == "wsl" {
            format!("{} no encontrado en WSL. Instalalo dentro de WSL primero.", agent)
        } else {
            format!("{} no encontrado en PATH. Instalalo primero.", agent)
        };
        return Err(hint);
    }

    // Validate project path exists
    if !path_exists(&project.path, &project.environment) {
        return Err(format!("Ruta no encontrada: {}", project.path));
    }

    // Launch agent in a visible terminal
    launch_in_terminal(agent, &project.path, &project.environment, &project.name)?;

    // Update lastOpenedAt
    update_last_opened(project_id)?;

    Ok(format!("{} launched for {}", req.agent, project.name))
}

/// Resume a specific agent session by ID
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResumeAgentSessionRequest {
    pub project_id: String,
    pub agent: String,
    pub session_id: String,
}

#[tauri::command]
pub async fn resume_agent_session(req: ResumeAgentSessionRequest) -> Result<String, String> {
    let store = load_projects()?;
    let project_id = uuid::Uuid::parse_str(&req.project_id)
        .map_err(|_| format!("ID inválido: {}", req.project_id))?;

    let project = store
        .projects
        .iter()
        .find(|p| p.id == project_id)
        .ok_or_else(|| format!("Proyecto no encontrado: {}", req.project_id))?;

    let agent = &req.agent;
    let session_id = &req.session_id;

    // Validate agent exists in the right environment
    let agent_available = if is_windows() && project.environment == "wsl" {
        which_wsl(agent)
    } else {
        which(agent)
    };

    if !agent_available {
        let hint = if is_windows() && project.environment == "wsl" {
            format!("{} no encontrado en WSL. Instalalo dentro de WSL primero.", agent)
        } else {
            format!("{} no encontrado en PATH. Instalalo primero.", agent)
        };
        return Err(hint);
    }

    // Build resume command
    let resume_flag = if agent == "claude" { "-r" } else { "--resume" };
    let resume_cmd = format!("{} {} {}", agent, resume_flag, session_id);

    // Validate project path
    if !path_exists(&project.path, &project.environment) {
        return Err(format!("Ruta no encontrada: {}", project.path));
    }

    // Launch in terminal
    launch_in_terminal(&resume_cmd, &project.path, &project.environment, &project.name)?;

    // Update lastOpenedAt
    update_last_opened(project_id)?;

    Ok(format!("{} session {} resumed for {}", req.agent, &session_id[..8], project.name))
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
fn launch_in_terminal(
    agent: &str,
    path: &str,
    env: &str,
    name: &str,
) -> Result<(), String> {
    let title = format!("{} - {}", agent, name);

    if is_wsl() {
        // Running inside WSL → open Windows Terminal via wt.exe
        let cmd = format!("cd '{}' && exec {}", path, agent);
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
    } else if is_windows() && env == "wsl" {
        // Windows native, project lives in WSL
        // Launch agent inside WSL via wt.exe + wsl
        let wsl_cmd = format!("cd '{}' && exec {}", path, agent);
        Command::new("wt.exe")
            .arg("new-tab")
            .arg("--title")
            .arg(&title)
            .arg("--profile")
            .arg("Ubuntu")
            .arg("wsl")
            .arg("bash")
            .arg("-ic")
            .arg(&wsl_cmd)
            .spawn()
            .map_err(|e| {
                format!(
                    "wt.exe failed: {}. Asegurate de tener Windows Terminal instalado con un perfil 'Ubuntu'.",
                    e
                )
            })?;
        Ok(())
    } else if is_windows() {
        // Windows native, project also on Windows
        // Use /k to keep terminal open after command
        let win_path = path.replace('/', "\\");
        Command::new("wt.exe")
            .arg("new-tab")
            .arg("--title")
            .arg(&title)
            .arg("--startingDirectory")
            .arg(&win_path)
            .arg("cmd.exe")
            .arg("/k")
            .arg(agent)
            .spawn()
            .map_err(|e| format!("wt.exe failed: {}", e))?;
        Ok(())
    } else if is_macos() {
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
        // Linux native
        let cmd = format!("cd '{}' && exec {}", path, agent);
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
