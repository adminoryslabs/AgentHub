use std::collections::HashSet;
use std::fs;
use std::path::Path;

use crate::commands::projects::{
    ensure_data_dir,
    get_data_dir,
    normalize_path_for_comparison,
    normalize_path_for_storage,
    resolve_filesystem_path,
};
use crate::models::project::Project;
use crate::models::ecosystem::{Ecosystem, EcosystemsStore};

fn normalize_required_text(value: String, field: &str) -> Result<String, String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(format!("El campo '{}' es requerido", field));
    }

    Ok(trimmed.to_string())
}

fn get_ecosystems_path() -> std::path::PathBuf {
    get_data_dir().join("ecosystems.json")
}

fn validate_unique_name(store: &EcosystemsStore, name: &str, excluding_id: Option<uuid::Uuid>) -> Result<(), String> {
    let duplicated = store.ecosystems.iter().any(|ecosystem| {
        excluding_id.map(|id| ecosystem.id != id).unwrap_or(true)
            && ecosystem.name.eq_ignore_ascii_case(name)
    });

    if duplicated {
        return Err(format!("Ya existe un ecosistema con el nombre '{}'", name));
    }

    Ok(())
}

fn validate_root_path(root_path: &str, environment: &str) -> Result<(), String> {
    let filesystem_path = resolve_filesystem_path(root_path, environment)?;
    let path = Path::new(&filesystem_path);
    if !path.exists() {
        return Err(format!("Ruta no encontrada: {}", root_path));
    }

    if !path.is_dir() {
        return Err(format!("La ruta no es una carpeta: {}", root_path));
    }

    Ok(())
}

fn create_ecosystem_record(
    store: &mut EcosystemsStore,
    name: String,
    root_path: String,
    environment: String,
    preferred_editor: String,
    default_agent: String,
) -> Result<Ecosystem, String> {
    validate_unique_name(store, &name, None)?;
    validate_root_path(&root_path, &environment)?;

    let ecosystem = Ecosystem::new(name, root_path, environment, preferred_editor, default_agent);
    store.ecosystems.push(ecosystem.clone());
    Ok(ecosystem)
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

fn validate_assigned_projects(root_path: &str, environment: &str, ecosystem_id: uuid::Uuid) -> Result<(), String> {
    let projects_store = crate::commands::projects::load_projects()?;
    let invalid_project = projects_store.projects.iter().find(|project| {
        project.ecosystem_id == Some(ecosystem_id)
            && (project.environment != environment
                || !path_belongs_to_root(&project.path, root_path, environment))
    });

    if let Some(project) = invalid_project {
        return Err(format!(
            "El proyecto '{}' ya no encaja en este ecosistema. Ajusta sus asignaciones antes de guardar.",
            project.name
        ));
    }

    Ok(())
}

fn build_imported_project(path: &str, ecosystem: &Ecosystem) -> Result<Project, String> {
    let name = Path::new(path)
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or_else(|| format!("No se pudo derivar el nombre del proyecto desde {}", path))?
        .to_string();

    Ok(Project::new(
        name,
        path.to_string(),
        ecosystem.environment.clone(),
        ecosystem.preferred_editor.clone(),
        ecosystem.default_agent.clone(),
        Vec::new(),
        Some(ecosystem.id),
    ))
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EcosystemFolderCandidate {
    pub name: String,
    pub path: String,
    pub is_already_registered: bool,
    pub existing_project_name: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanEcosystemFolderRequest {
    pub root_path: String,
    #[serde(rename = "env")]
    pub environment: String,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanEcosystemFolderResponse {
    pub candidates: Vec<EcosystemFolderCandidate>,
}

pub fn load_ecosystems() -> Result<EcosystemsStore, String> {
    let path = get_ecosystems_path();
    if !path.exists() {
        return Ok(EcosystemsStore::new());
    }

    let content = fs::read_to_string(&path)
        .map_err(|e| format!("No se pudo leer {}: {}", path.display(), e))?;
    let store: EcosystemsStore = serde_json::from_str(&content)
        .map_err(|e| format!("No se pudo parsear {}: {}", path.display(), e))?;

    Ok(store)
}

pub fn save_ecosystems(store: &EcosystemsStore) -> Result<(), String> {
    ensure_data_dir()?;
    let path = get_ecosystems_path();
    let content = serde_json::to_string_pretty(store)
        .map_err(|e| format!("No se pudo serializar ecosystems.json: {}", e))?;

    fs::write(&path, content)
        .map_err(|e| format!("No se pudo escribir {}: {}", path.display(), e))?;

    Ok(())
}

#[tauri::command]
pub async fn get_ecosystems() -> Result<Vec<Ecosystem>, String> {
    let store = load_ecosystems()?;
    Ok(store.ecosystems)
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateEcosystemRequest {
    pub name: String,
    pub root_path: String,
    #[serde(rename = "env")]
    pub environment: String,
    pub preferred_editor: String,
    pub default_agent: String,
}

#[tauri::command]
pub async fn create_ecosystem(req: CreateEcosystemRequest) -> Result<Ecosystem, String> {
    let mut store = load_ecosystems()?;
    let name = normalize_required_text(req.name, "name")?;
    let root_path = normalize_path_for_storage(&normalize_required_text(req.root_path, "rootPath")?, &req.environment)?;
    let ecosystem = create_ecosystem_record(
        &mut store,
        name,
        root_path,
        req.environment,
        req.preferred_editor,
        req.default_agent,
    )?;
    save_ecosystems(&store)?;
    Ok(ecosystem)
}

#[tauri::command]
pub async fn scan_ecosystem_folder(req: ScanEcosystemFolderRequest) -> Result<ScanEcosystemFolderResponse, String> {
    let root_path = normalize_path_for_storage(&normalize_required_text(req.root_path, "rootPath")?, &req.environment)?;
    validate_root_path(&root_path, &req.environment)?;
    let scan_path = resolve_filesystem_path(&root_path, &req.environment)?;

    let projects_store = crate::commands::projects::load_projects()?;
    let registered_by_path: std::collections::HashMap<String, String> = projects_store
        .projects
        .iter()
        .map(|project| {
            (
                normalize_path_for_comparison(&project.path, &project.environment),
                project.name.clone(),
            )
        })
        .collect();

    let normalized_root = normalize_path_for_comparison(&root_path, &req.environment);
    let mut candidates = Vec::new();

    let entries = fs::read_dir(&scan_path)
        .map_err(|e| format!("No se pudo leer {}: {}", root_path, e))?;

    for entry in entries {
        let entry = entry.map_err(|e| format!("No se pudo leer una entrada de {}: {}", root_path, e))?;
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }

        let path_str = path.to_string_lossy().to_string();
        let candidate_path = normalize_path_for_storage(&path_str, &req.environment)?;
        let normalized_path = normalize_path_for_comparison(&candidate_path, &req.environment);

        if normalized_path == normalized_root {
            continue;
        }

        let name = entry.file_name().to_string_lossy().to_string();
        let existing_project_name = registered_by_path.get(&normalized_path).cloned();

        candidates.push(EcosystemFolderCandidate {
            name,
            path: candidate_path,
            is_already_registered: existing_project_name.is_some(),
            existing_project_name,
        });
    }

    candidates.sort_by(|left, right| left.name.cmp(&right.name));

    Ok(ScanEcosystemFolderResponse { candidates })
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateEcosystemRequest {
    pub id: String,
    pub name: String,
    pub root_path: String,
    #[serde(rename = "env")]
    pub environment: String,
    pub preferred_editor: String,
    pub default_agent: String,
}

#[tauri::command]
pub async fn update_ecosystem(req: UpdateEcosystemRequest) -> Result<Ecosystem, String> {
    let mut store = load_ecosystems()?;
    let ecosystem_id = uuid::Uuid::parse_str(&req.id)
        .map_err(|_| format!("ID inválido: {}", req.id))?;

    let name = normalize_required_text(req.name, "name")?;
    let root_path = normalize_path_for_storage(&normalize_required_text(req.root_path, "rootPath")?, &req.environment)?;
    validate_unique_name(&store, &name, Some(ecosystem_id))?;
    validate_root_path(&root_path, &req.environment)?;
    validate_assigned_projects(&root_path, &req.environment, ecosystem_id)?;

    let ecosystem = store
        .ecosystems
        .iter_mut()
        .find(|ecosystem| ecosystem.id == ecosystem_id)
        .ok_or_else(|| format!("Ecosistema no encontrado: {}", req.id))?;

    ecosystem.name = name;
    ecosystem.root_path = root_path;
    ecosystem.environment = req.environment;
    ecosystem.preferred_editor = req.preferred_editor;
    ecosystem.default_agent = req.default_agent;

    let updated = ecosystem.clone();
    save_ecosystems(&store)?;
    Ok(updated)
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportEcosystemFolderRequest {
    pub name: String,
    pub root_path: String,
    #[serde(rename = "env")]
    pub environment: String,
    pub preferred_editor: String,
    pub default_agent: String,
    pub selected_paths: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportEcosystemFolderResponse {
    pub ecosystem: Ecosystem,
    pub imported_projects: Vec<Project>,
}

#[tauri::command]
pub async fn import_ecosystem_folder(
    req: ImportEcosystemFolderRequest,
) -> Result<ImportEcosystemFolderResponse, String> {
    let name = normalize_required_text(req.name, "name")?;
    let root_path = normalize_path_for_storage(&normalize_required_text(req.root_path, "rootPath")?, &req.environment)?;
    validate_root_path(&root_path, &req.environment)?;

    if req.selected_paths.is_empty() {
        return Err("Debes seleccionar al menos una carpeta para importar".to_string());
    }

    let scan = scan_ecosystem_folder(ScanEcosystemFolderRequest {
        root_path: root_path.clone(),
        environment: req.environment.clone(),
    }).await?;

    let selected_set: HashSet<String> = req
        .selected_paths
        .iter()
        .map(|path| normalize_path_for_comparison(path, &req.environment))
        .collect();
    let candidates_by_path: std::collections::HashMap<String, EcosystemFolderCandidate> = scan
        .candidates
        .into_iter()
        .map(|candidate| (normalize_path_for_comparison(&candidate.path, &req.environment), candidate))
        .collect();

    let mut import_paths = Vec::new();
    for normalized_path in &selected_set {
        let candidate = candidates_by_path
            .get(normalized_path)
            .ok_or_else(|| format!("La carpeta seleccionada ya no es valida: {}", normalized_path))?;

        if candidate.is_already_registered {
            return Err(format!(
                "La carpeta '{}' ya esta registrada como proyecto{}",
                candidate.path,
                candidate
                    .existing_project_name
                    .as_ref()
                    .map(|name| format!(" ({})", name))
                    .unwrap_or_default()
            ));
        }

        import_paths.push(candidate.path.clone());
    }

    let mut ecosystems_store = load_ecosystems()?;
    let ecosystem = create_ecosystem_record(
        &mut ecosystems_store,
        name,
        root_path,
        req.environment.clone(),
        req.preferred_editor,
        req.default_agent,
    )?;

    let mut projects_store = crate::commands::projects::load_projects()?;
    let mut imported_projects = Vec::new();

    for path in import_paths {
        let project = build_imported_project(&path, &ecosystem)?;
        projects_store.projects.push(project.clone());
        imported_projects.push(project);
    }

    save_ecosystems(&ecosystems_store)?;
    crate::commands::projects::save_projects(&projects_store)?;

    Ok(ImportEcosystemFolderResponse {
        ecosystem,
        imported_projects,
    })
}

#[tauri::command]
pub async fn delete_ecosystem(id: String) -> Result<(), String> {
    let ecosystem_id = uuid::Uuid::parse_str(&id)
        .map_err(|_| format!("ID inválido: {}", id))?;

    let mut ecosystems_store = load_ecosystems()?;
    let len_before = ecosystems_store.ecosystems.len();
    ecosystems_store.ecosystems.retain(|ecosystem| ecosystem.id != ecosystem_id);

    if ecosystems_store.ecosystems.len() == len_before {
        return Err(format!("Ecosistema no encontrado: {}", id));
    }

    let mut projects_store = crate::commands::projects::load_projects()?;
    for project in &mut projects_store.projects {
        if project.ecosystem_id == Some(ecosystem_id) {
            project.ecosystem_id = None;
        }
    }

    save_ecosystems(&ecosystems_store)?;
    crate::commands::projects::save_projects(&projects_store)?;
    Ok(())
}
