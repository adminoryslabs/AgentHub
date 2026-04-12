use serde::Deserialize;
use std::process::Command;

use crate::commands::projects::load_projects;
use crate::logging::log_debug;

// ============================================================================
// Environment Detection
// ============================================================================

fn detect_platform() -> String {
    if is_wsl() {
        "wsl".to_string()
    } else if is_windows() {
        "windows".to_string()
    } else if is_macos() {
        "macos".to_string()
    } else {
        "linux".to_string()
    }
}

fn is_wsl() -> bool {
    let env_check = std::env::var("WSL_DISTRO_NAME").is_ok();
    let proc_check = std::fs::read_to_string("/proc/version")
        .map(|c| c.to_lowercase().contains("microsoft") || c.to_lowercase().contains("wsl"))
        .unwrap_or(false);
    let result = env_check || proc_check;
    log_debug(&format!("[env] is_wsl() = {} (env={}, proc={})", result, env_check, proc_check));
    result
}

fn is_macos() -> bool {
    cfg!(target_os = "macos")
}

fn is_windows() -> bool {
    cfg!(target_os = "windows")
}

/// Check if a command exists in PATH.
fn which(cmd: &str) -> bool {
    if is_windows() {
        let variants = [
            cmd.to_string(),
            format!("{}.cmd", cmd),
            format!("{}.exe", cmd),
        ];
        for v in &variants {
            let found = Command::new("where")
                .arg(v)
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false);
            log_debug(&format!("[which] {} -> {} ({})", cmd, found, v));
            if found { return true; }
        }
        // Check common install locations
        if let Some(path) = resolve_command_path(cmd) {
            log_debug(&format!("[which] {} -> found at {}", cmd, path));
            return true;
        }
        log_debug(&format!("[which] {} -> NOT FOUND", cmd));
        false
    } else {
        let found = Command::new("which")
            .arg(cmd)
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false);
        log_debug(&format!("[which] {} -> {} (unix)", cmd, found));
        found
    }
}

/// Check if a command exists inside WSL (from Windows).
fn which_wsl(cmd: &str) -> bool {
    let output = Command::new("wsl")
        .args(["which", cmd])
        .output();
    let found = output.as_ref().map(|o| o.status.success()).unwrap_or(false);
    let stderr = output.as_ref().map(|o| String::from_utf8_lossy(&o.stderr).to_string()).unwrap_or_default();
    let stdout = output.as_ref().map(|o| String::from_utf8_lossy(&o.stdout).to_string()).unwrap_or_default();
    log_debug(&format!("[which_wsl] {} -> {} (stdout={}, stderr={})", cmd, found, stdout.trim(), stderr.trim()));
    found
}

/// Resolve common install paths for commands on Windows
fn resolve_command_path(cmd: &str) -> Option<String> {
    if let Ok(localappdata) = std::env::var("LOCALAPPDATA") {
        match cmd {
            "code" => {
                let paths = [
                    format!("{}\\Programs\\Microsoft VS Code\\bin\\code.cmd", localappdata),
                    format!("{}\\Programs\\Microsoft VS Code\\bin\\code.exe", localappdata),
                ];
                for p in &paths {
                    if std::path::Path::new(p).exists() {
                        return Some(p.clone());
                    }
                }
                if let Ok(pf) = std::env::var("ProgramFiles") {
                    let p = format!("{}\\Microsoft VS Code\\bin\\code.cmd", pf);
                    if std::path::Path::new(&p).exists() { return Some(p); }
                }
            }
            "cursor" => {
                let p = format!("{}\\Programs\\cursor\\cursor.exe", localappdata);
                if std::path::Path::new(&p).exists() { return Some(p); }
            }
            _ => {}
        }
    }
    None
}

/// Check if a path exists, handling cross-environment cases
fn path_exists(path: &str, env: &str) -> bool {
    let result = if is_windows() && env == "wsl" {
        let output = Command::new("wsl")
            .args(["test", "-d", path])
            .output();
        let ok = output.as_ref().map(|o| o.status.success()).unwrap_or(false);
        let stderr = output.as_ref().map(|o| String::from_utf8_lossy(&o.stderr).to_string()).unwrap_or_default();
        log_debug(&format!("[path_exists] wsl test -d '{}' -> {} (stderr={})", path, ok, stderr.trim()));
        ok
    } else {
        let exists = std::path::Path::new(path).exists();
        log_debug(&format!("[path_exists] '{}' -> {} (native)", path, exists));
        exists
    };
    result
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
    let platform = detect_platform();
    log_debug(&format!("[open_editor] START editor={} platform={}", req.editor, platform));

    let store = load_projects()?;
    let project_id = uuid::Uuid::parse_str(&req.project_id)
        .map_err(|_| format!("ID inválido: {}", req.project_id))?;

    let project = store
        .projects
        .iter()
        .find(|p| p.id == project_id)
        .ok_or_else(|| format!("Proyecto no encontrado: {}", req.project_id))?;

    log_debug(&format!("[open_editor] project={} env={} path={}", project.name, project.environment, project.path));

    let editor = &req.editor;

    // Validate editor exists
    let editor_available = if is_windows() && project.environment == "wsl" {
        log_debug(&format!("[open_editor] WSL project - skipping editor validation on Windows side"));
        true
    } else {
        let found = which(editor) || resolve_command_path(editor).is_some();
        log_debug(&format!("[open_editor] editor '{}' available in PATH: {}", editor, found));
        found
    };

    if !editor_available {
        let hint = if is_windows() {
            match editor.as_str() {
                "vscode" => "VS Code no encontrado. Asegurate de instalar con 'Add to PATH' o reinstala marcando esa opcion.",
                "cursor" => "Cursor no encontrado en PATH.",
                _ => return Err(format!("{} no encontrado en PATH. Instalalo primero.", editor)),
            }
        } else {
            return Err(format!("{} no encontrado en PATH. Instalalo primero.", editor));
        };
        return Err(hint.to_string());
    }

    // Validate project path exists
    if !path_exists(&project.path, &project.environment) {
        return Err(format!("Ruta no encontrada: {}", project.path));
    }

    // Launch editor
    log_debug(&format!("[open_editor] launching '{}' for path='{}' env='{}'", editor, project.path, project.environment));

    let result = if is_wsl() {
        log_debug("[open_editor] running from WSL: direct launch");
        Command::new(editor).arg(&project.path).spawn()
    } else if is_windows() && project.environment == "wsl" {
        log_debug(&format!("[open_editor] running from Windows for WSL: wsl {}", editor));
        Command::new("wsl").arg(editor).arg(&project.path).spawn()
    } else {
        log_debug(&format!("[open_editor] native launch: {}", editor));
        Command::new(editor).arg(&project.path).spawn()
    };

    match &result {
        Ok(_) => log_debug(&format!("[open_editor] SUCCESS: {} opened", editor)),
        Err(e) => log_debug(&format!("[open_editor] FAILED: {}", e)),
    }

    result.map_err(|e| format!("No se pudo abrir {}: {}", editor, e))?;

    // Update lastOpenedAt
    let _ = update_last_opened(project_id);

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
    let platform = detect_platform();
    log_debug(&format!("[launch_agent] START agent={} platform={}", req.agent, platform));

    let store = load_projects()?;
    let project_id = uuid::Uuid::parse_str(&req.project_id)
        .map_err(|_| format!("ID inválido: {}", req.project_id))?;

    let project = store
        .projects
        .iter()
        .find(|p| p.id == project_id)
        .ok_or_else(|| format!("Proyecto no encontrado: {}", req.project_id))?;

    log_debug(&format!("[launch_agent] project={} env={} path={}", project.name, project.environment, project.path));

    let agent = &req.agent;

    // Validate agent exists in the right environment
    let agent_available = if is_windows() && project.environment == "wsl" {
        let found = which_wsl(agent);
        log_debug(&format!("[launch_agent] agent '{}' in WSL: {}", agent, found));
        found
    } else {
        let found = which(agent);
        log_debug(&format!("[launch_agent] agent '{}' in {}: {}", agent, platform, found));
        found
    };

    if !agent_available {
        let hint = if is_windows() && project.environment == "wsl" {
            format!("{} no encontrado en WSL. Instalalo dentro de WSL primero.", agent)
        } else {
            format!("{} no encontrado en PATH. Instalalo primero.", agent)
        };
        log_debug(&format!("[launch_agent] FAILED: {}", hint));
        return Err(hint);
    }

    // Validate project path exists
    if !path_exists(&project.path, &project.environment) {
        return Err(format!("Ruta no encontrada: {}", project.path));
    }

    // Launch agent in a visible terminal
    log_debug(&format!("[launch_agent] launching '{}' in terminal for path='{}'", agent, project.path));
    launch_in_terminal(agent, &project.path, &project.environment, &project.name)?;

    // Update lastOpenedAt
    let _ = update_last_opened(project_id);

    Ok(format!("{} launched for {}", req.agent, project.name))
}

// ============================================================================
// Resume Agent Session
// ============================================================================

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResumeAgentSessionRequest {
    pub project_id: String,
    pub agent: String,
    pub session_id: String,
}

#[tauri::command]
pub async fn resume_agent_session(req: ResumeAgentSessionRequest) -> Result<String, String> {
    let platform = detect_platform();
    log_debug(&format!("[resume_session] START agent={} session={} platform={}", req.agent, req.session_id, platform));

    let store = load_projects()?;
    let project_id = uuid::Uuid::parse_str(&req.project_id)
        .map_err(|_| format!("ID inválido: {}", req.project_id))?;

    let project = store
        .projects
        .iter()
        .find(|p| p.id == project_id)
        .ok_or_else(|| format!("Proyecto no encontrado: {}", req.project_id))?;

    log_debug(&format!("[resume_session] project={} env={} path={}", project.name, project.environment, project.path));

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
    log_debug(&format!("[resume_session] command: '{}'", resume_cmd));

    // Validate project path
    if !path_exists(&project.path, &project.environment) {
        return Err(format!("Ruta no encontrada: {}", project.path));
    }

    // Launch in terminal
    launch_in_terminal(&resume_cmd, &project.path, &project.environment, &project.name)?;

    // Update lastOpenedAt
    let _ = update_last_opened(project_id);

    Ok(format!("{} session {} resumed for {}", req.agent, &session_id[..8], project.name))
}

// ============================================================================
// Utility
// ============================================================================

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

/// Launch a command in a visible terminal window.
fn launch_in_terminal(
    cmd: &str,
    path: &str,
    env: &str,
    name: &str,
) -> Result<(), String> {
    let title = format!("{}", name);
    log_debug(&format!("[launch_terminal] cmd='{}' path='{}' env='{}'", cmd, path, env));

    if is_wsl() {
        // Running inside WSL → open Windows Terminal via wt.exe
        let full_cmd = format!("cd '{}' && exec {}", path, cmd);
        log_debug(&format!("[launch_terminal] WSL mode: wt.exe bash -ic '{}'", full_cmd));
        Command::new("wt.exe")
            .arg("--title")
            .arg(&title)
            .arg("--")
            .arg("bash")
            .arg("-ic")
            .arg(&full_cmd)
            .spawn()
            .map_err(|e| {
                let msg = format!("wt.exe failed: {}. Asegurate de tener Windows Terminal instalado.", e);
                log_debug(&format!("[launch_terminal] FAILED: {}", msg));
                msg
            })?;
        Ok(())
    } else if is_windows() && env == "wsl" {
        // Windows native, project lives in WSL
        let wsl_cmd = format!("cd '{}' && exec {}", path, cmd);
        log_debug(&format!("[launch_terminal] Windows→WSL: wt.exe --profile Ubuntu wsl bash -ic '{}'", wsl_cmd));
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
                let msg = format!("wt.exe failed: {}. Asegurate de tener Windows Terminal con perfil 'Ubuntu'. Error: {}", e, e);
                log_debug(&format!("[launch_terminal] FAILED: {}", msg));
                msg
            })?;
        Ok(())
    } else if is_windows() {
        // Windows native, project also on Windows
        let win_path = path.replace('/', "\\");
        log_debug(&format!("[launch_terminal] Windows native: cmd.exe /k '{}' in '{}'", cmd, win_path));
        Command::new("wt.exe")
            .arg("new-tab")
            .arg("--title")
            .arg(&title)
            .arg("--startingDirectory")
            .arg(&win_path)
            .arg("cmd.exe")
            .arg("/k")
            .arg(cmd)
            .spawn()
            .map_err(|e| {
                let msg = format!("wt.exe failed: {}", e);
                log_debug(&format!("[launch_terminal] FAILED: {}", msg));
                msg
            })?;
        Ok(())
    } else if is_macos() {
        let script = format!(
            "tell application \"Terminal\" to do script \"cd '{}' && exec {}\"",
            path, cmd
        );
        Command::new("osascript")
            .arg("-e")
            .arg(&script)
            .spawn()
            .map_err(|e| format!("osascript failed: {}", e))?;
        Ok(())
    } else {
        // Linux native
        let full_cmd = format!("cd '{}' && exec {}", path, cmd);
        for terminal in &["gnome-terminal", "x-terminal-emulator", "konsole", "xterm"] {
            if which(terminal) {
                return match *terminal {
                    "gnome-terminal" => Command::new("gnome-terminal")
                        .arg("--")
                        .arg("bash")
                        .arg("-ic")
                        .arg(&full_cmd)
                        .spawn()
                        .map(|_| ())
                        .map_err(|e| format!("{} failed: {}", terminal, e)),
                    "konsole" => Command::new("konsole")
                        .arg("-e")
                        .arg("bash")
                        .arg("-ic")
                        .arg(&full_cmd)
                        .spawn()
                        .map(|_| ())
                        .map_err(|e| format!("{} failed: {}", terminal, e)),
                    _ => Command::new(terminal)
                        .arg("-e")
                        .arg(format!("bash -ic '{}'", full_cmd))
                        .spawn()
                        .map(|_| ())
                        .map_err(|e| format!("{} failed: {}", terminal, e)),
                };
            }
        }
        Err("No terminal emulator found. Install gnome-terminal, xterm, or similar.".to_string())
    }
}
