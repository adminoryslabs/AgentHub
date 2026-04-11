use std::fs;
use std::path::PathBuf;

use crate::models::project::{Project, ProjectsStore};

fn get_projects_dir() -> PathBuf {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".dev-control-center")
}

fn get_projects_path() -> PathBuf {
    get_projects_dir().join("projects.json")
}

fn ensure_projects_dir() -> Result<(), String> {
    let dir = get_projects_dir();
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
    ensure_projects_dir()?;
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
}

#[tauri::command]
pub async fn create_project(req: CreateProjectRequest) -> Result<Project, String> {
    if req.name.trim().is_empty() {
        return Err("El campo 'name' es requerido".to_string());
    }

    let mut store = load_projects()?;
    let project = Project::new(
        req.name,
        req.path,
        req.environment,
        req.preferred_editor,
        req.default_agent,
        req.tags,
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
}

#[tauri::command]
pub async fn update_project(req: UpdateProjectRequest) -> Result<Project, String> {
    if req.name.trim().is_empty() {
        return Err("El campo 'name' es requerido".to_string());
    }

    let mut store = load_projects()?;
    let project_id = uuid::Uuid::parse_str(&req.id)
        .map_err(|_| format!("ID inválido: {}", req.id))?;

    let project = store.projects.iter_mut()
        .find(|p| p.id == project_id)
        .ok_or_else(|| format!("Proyecto no encontrado: {}", req.id))?;

    project.name = req.name;
    project.path = req.path;
    project.environment = req.environment;
    project.preferred_editor = req.preferred_editor;
    project.default_agent = req.default_agent;
    project.tags = req.tags;

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
