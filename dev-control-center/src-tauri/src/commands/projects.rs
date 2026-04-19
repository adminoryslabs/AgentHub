use std::fs;
use std::path::PathBuf;
use std::process::Command;

use crate::commands::ecosystems::load_ecosystems;
use crate::models::project::{Project, ProjectsStore};

fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.to_string())
        }
    })
}

pub(crate) fn is_windows_host() -> bool {
    cfg!(target_os = "windows")
}

fn run_wslpath(flag: &str, path: &str) -> Result<String, String> {
    let output = Command::new("wsl")
        .args(["wslpath", flag, path])
        .output()
        .map_err(|e| format!("No se pudo ejecutar wslpath: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Err(if stderr.is_empty() {
            format!("wslpath fallo para la ruta: {}", path)
        } else {
            stderr
        });
    }

    let value = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if value.is_empty() {
        return Err(format!("wslpath devolvio una ruta vacia para {}", path));
    }

    Ok(value)
}

pub(crate) fn normalize_path_for_comparison(path: &str, environment: &str) -> String {
    let separator = if environment == "windows" { '\\' } else { '/' };
    let mut normalized = path.trim().replace(['/', '\\'], &separator.to_string());

    while normalized.len() > 1 && normalized.ends_with(separator) {
        normalized.pop();
    }

    if environment == "windows" {
        normalized.make_ascii_lowercase();
    }

    normalized
}

pub(crate) fn normalize_path_for_storage(path: &str, environment: &str) -> Result<String, String> {
    let trimmed = path.trim();
    if trimmed.is_empty() {
        return Ok(String::new());
    }

    if environment == "wsl" {
        if trimmed.starts_with("/") {
            return Ok(normalize_path_for_comparison(trimmed, environment));
        }

        if is_windows_host() {
            return run_wslpath("-u", trimmed).map(|value| normalize_path_for_comparison(&value, environment));
        }

        return Ok(normalize_path_for_comparison(trimmed, environment));
    }

    Ok(normalize_path_for_comparison(trimmed, environment))
}

pub(crate) fn resolve_filesystem_path(path: &str, environment: &str) -> Result<String, String> {
    let normalized = normalize_path_for_storage(path, environment)?;

    if environment == "wsl" && is_windows_host() {
        return run_wslpath("-w", &normalized);
    }

    Ok(normalized)
}

fn parse_ecosystem_id(value: Option<String>) -> Result<Option<uuid::Uuid>, String> {
    let Some(value) = normalize_optional_text(value) else {
        return Ok(None);
    };

    uuid::Uuid::parse_str(&value)
        .map(Some)
        .map_err(|_| format!("ecosystemId inválido: {}", value))
}

fn path_belongs_to_root(project_path: &str, root_path: &str, environment: &str) -> bool {
    let project_path = normalize_path_for_comparison(project_path, environment);
    let root_path = normalize_path_for_comparison(root_path, environment);

    if project_path == root_path {
        return true;
    }

    let separator = if environment == "windows" { '\\' } else { '/' };
    let root_prefix = format!("{}{}", root_path, separator);
    project_path.starts_with(&root_prefix)
}

fn validate_ecosystem_id(
    ecosystem_id: Option<uuid::Uuid>,
    environment: &str,
    project_path: &str,
) -> Result<Option<uuid::Uuid>, String> {
    let Some(ecosystem_id) = ecosystem_id else {
        return Ok(None);
    };

    let ecosystems_store = load_ecosystems()?;
    let ecosystem = ecosystems_store
        .ecosystems
        .iter()
        .find(|ecosystem| ecosystem.id == ecosystem_id)
        .ok_or_else(|| format!("Ecosistema no encontrado: {}", ecosystem_id))?;

    if ecosystem.environment != environment {
        return Err(format!(
            "El proyecto usa env '{}' pero el ecosistema '{}' usa env '{}'",
            environment,
            ecosystem.name,
            ecosystem.environment
        ));
    }

    if !path_belongs_to_root(project_path, &ecosystem.root_path, environment) {
        return Err(format!(
            "La ruta del proyecto '{}' no esta dentro del root del ecosistema '{}' ({})",
            project_path,
            ecosystem.name,
            ecosystem.root_path
        ));
    }

    Ok(Some(ecosystem_id))
}

pub(crate) fn get_data_dir() -> PathBuf {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".dev-control-center")
}

fn get_projects_path() -> PathBuf {
    get_data_dir().join("projects.json")
}

pub(crate) fn ensure_data_dir() -> Result<(), String> {
    let dir = get_data_dir();
    if !dir.exists() {
        fs::create_dir_all(&dir).map_err(|e| format!("No se pudo crear el directorio {}: {}", dir.display(), e))?;
    }
    Ok(())
}

pub fn load_projects() -> Result<ProjectsStore, String> {
    let path = get_projects_path();
    if !path.exists() {
        return Ok(ProjectsStore::new());
    }
    let content = fs::read_to_string(&path)
        .map_err(|e| format!("No se pudo leer {}: {}", path.display(), e))?;
    let store: ProjectsStore = serde_json::from_str(&content)
        .map_err(|e| format!("No se pudo parsear {}: {}", path.display(), e))?;
    Ok(store)
}

pub fn save_projects(store: &ProjectsStore) -> Result<(), String> {
    ensure_data_dir()?;
    let path = get_projects_path();
    let content = serde_json::to_string_pretty(store)
        .map_err(|e| format!("No se pudo serializar projects.json: {}", e))?;
    fs::write(&path, content)
        .map_err(|e| format!("No se pudo escribir {}: {}", path.display(), e))?;
    Ok(())
}

#[tauri::command]
pub async fn get_projects() -> Result<Vec<Project>, String> {
    let store = load_projects()?;
    Ok(store.projects)
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateProjectRequest {
    pub name: String,
    pub path: String,
    #[serde(rename = "env")]
    pub environment: String,
    pub preferred_editor: String,
    pub default_agent: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub ecosystem_id: Option<String>,
}

#[tauri::command]
pub async fn create_project(req: CreateProjectRequest) -> Result<Project, String> {
    if req.name.trim().is_empty() {
        return Err("El campo 'name' es requerido".to_string());
    }

    let mut store = load_projects()?;
    let path = normalize_path_for_storage(&req.path, &req.environment)?;
    let ecosystem_id = validate_ecosystem_id(parse_ecosystem_id(req.ecosystem_id)?, &req.environment, &path)?;
    let project = Project::new(
        req.name,
        path,
        req.environment,
        req.preferred_editor,
        req.default_agent,
        req.tags,
        ecosystem_id,
    );
    store.projects.push(project.clone());
    save_projects(&store)?;
    Ok(project)
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProjectRequest {
    pub id: String,
    pub name: String,
    pub path: String,
    #[serde(rename = "env")]
    pub environment: String,
    pub preferred_editor: String,
    pub default_agent: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub ecosystem_id: Option<String>,
}

#[tauri::command]
pub async fn update_project(req: UpdateProjectRequest) -> Result<Project, String> {
    if req.name.trim().is_empty() {
        return Err("El campo 'name' es requerido".to_string());
    }

    let mut store = load_projects()?;
    let project_id = uuid::Uuid::parse_str(&req.id)
        .map_err(|_| format!("ID inválido: {}", req.id))?;

    let path = normalize_path_for_storage(&req.path, &req.environment)?;
    let ecosystem_id = validate_ecosystem_id(parse_ecosystem_id(req.ecosystem_id)?, &req.environment, &path)?;

    let project = store.projects.iter_mut()
        .find(|p| p.id == project_id)
        .ok_or_else(|| format!("Proyecto no encontrado: {}", req.id))?;

    project.name = req.name;
    project.path = path;
    project.environment = req.environment;
    project.preferred_editor = req.preferred_editor;
    project.default_agent = req.default_agent;
    project.tags = req.tags;
    project.ecosystem_id = ecosystem_id;

    let updated = project.clone();
    save_projects(&store)?;
    Ok(updated)
}

#[tauri::command]
pub async fn delete_project(id: String) -> Result<(), String> {
    let mut store = load_projects()?;
    let project_id = uuid::Uuid::parse_str(&id)
        .map_err(|_| format!("ID inválido: {}", id))?;

    let len_before = store.projects.len();
    store.projects.retain(|p| p.id != project_id);

    if store.projects.len() == len_before {
        return Err(format!("Proyecto no encontrado: {}", id));
    }

    save_projects(&store)?;
    Ok(())
}

#[tauri::command]
pub async fn pick_directory() -> Result<Option<String>, String> {
    // En WSL el diálogo nativo de rfd usa D-Bus/portales que no están disponibles.
    // Usamos PowerShell de Windows para mostrar el FolderBrowserDialog.
    let is_wsl = std::env::var("WSL_DISTRO_NAME").is_ok()
        || std::fs::read_to_string("/proc/version")
            .map(|c| c.to_lowercase().contains("microsoft") || c.to_lowercase().contains("wsl"))
            .unwrap_or(false);

    if is_wsl {
        let output = std::process::Command::new("powershell.exe")
            .args([
                "-NoProfile",
                "-NonInteractive",
                "-Command",
                "[System.Reflection.Assembly]::LoadWithPartialName('System.Windows.Forms') | Out-Null; \
                 $d = New-Object System.Windows.Forms.FolderBrowserDialog; \
                 $d.Description = 'Seleccionar carpeta del proyecto'; \
                 $d.RootFolder = 'MyComputer'; \
                 if ($d.ShowDialog() -eq 'OK') { $d.SelectedPath }",
            ])
            .output()
            .map_err(|e| format!("PowerShell no disponible: {}", e))?;

        let raw = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if raw.is_empty() {
            return Ok(None); // usuario canceló
        }

        // Convertir ruta Windows (C:\...) a ruta WSL (/mnt/c/...) con wslpath
        let wsl_path = std::process::Command::new("wslpath")
            .arg(&raw)
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or(raw);

        return Ok(Some(wsl_path));
    }

    let path = rfd::AsyncFileDialog::new()
        .pick_folder()
        .await;

    Ok(path.map(|p| p.path().to_string_lossy().to_string()))
}
