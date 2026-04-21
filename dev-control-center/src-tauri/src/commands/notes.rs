use std::fs;
use std::path::PathBuf;

use serde::Deserialize;

use crate::commands::ecosystems::load_ecosystems;
use crate::commands::projects::load_projects;

fn get_notes_dir() -> PathBuf {
    crate::commands::projects::get_data_dir().join("notes")
}

fn ensure_dir(dir: &PathBuf) -> Result<(), String> {
    if !dir.exists() {
        fs::create_dir_all(&dir)
            .map_err(|e| format!("No se pudo crear el directorio {}: {}", dir.display(), e))?;
    }
    Ok(())
}

fn ensure_notes_dir() -> Result<(), String> {
    ensure_dir(&get_notes_dir())
}

fn get_project_notes_dir() -> PathBuf {
    get_notes_dir().join("projects")
}

fn get_ecosystem_notes_dir() -> PathBuf {
    get_notes_dir().join("ecosystems")
}

fn get_project_note_path(project_id: uuid::Uuid) -> PathBuf {
    get_project_notes_dir().join(format!("{}.md", project_id))
}

fn get_legacy_project_note_path(project_id: uuid::Uuid) -> PathBuf {
    get_notes_dir().join(format!("{}.md", project_id))
}

fn get_ecosystem_note_path(ecosystem_id: uuid::Uuid) -> PathBuf {
    get_ecosystem_notes_dir().join(format!("{}.md", ecosystem_id))
}

fn get_general_note_path() -> PathBuf {
    get_notes_dir().join("_general.md")
}

fn validate_project_exists(project_id: uuid::Uuid) -> Result<(), String> {
    let store = load_projects()?;
    store
        .projects
        .iter()
        .find(|project| project.id == project_id)
        .ok_or_else(|| format!("Proyecto no encontrado: {}", project_id))?;
    Ok(())
}

fn validate_ecosystem_exists(ecosystem_id: uuid::Uuid) -> Result<(), String> {
    let store = load_ecosystems()?;
    store
        .ecosystems
        .iter()
        .find(|ecosystem| ecosystem.id == ecosystem_id)
        .ok_or_else(|| format!("Ecosistema no encontrado: {}", ecosystem_id))?;
    Ok(())
}

fn read_note(path: &PathBuf) -> Result<String, String> {
    if !path.exists() {
        return Ok(String::new());
    }

    fs::read_to_string(path)
        .map_err(|e| format!("No se pudo leer {}: {}", path.display(), e))
}

fn write_note(path: &PathBuf, content: &str) -> Result<(), String> {
    ensure_notes_dir()?;
    if let Some(parent) = path.parent() {
        ensure_dir(&parent.to_path_buf())?;
    }
    fs::write(path, content)
        .map_err(|e| format!("No se pudo escribir {}: {}", path.display(), e))
}

fn read_project_note(project_id: uuid::Uuid) -> Result<String, String> {
    let note_path = get_project_note_path(project_id);
    if note_path.exists() {
        return read_note(&note_path);
    }

    read_note(&get_legacy_project_note_path(project_id))
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectNoteRequest {
    pub project_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveProjectNoteRequest {
    pub project_id: String,
    pub content: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EcosystemNoteRequest {
    pub ecosystem_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveEcosystemNoteRequest {
    pub ecosystem_id: String,
    pub content: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveGeneralNoteRequest {
    pub content: String,
}

#[tauri::command]
pub async fn get_project_note(req: ProjectNoteRequest) -> Result<String, String> {
    let project_id = uuid::Uuid::parse_str(&req.project_id)
        .map_err(|_| format!("ID inválido: {}", req.project_id))?;

    validate_project_exists(project_id)?;
    read_project_note(project_id)
}

#[tauri::command]
pub async fn save_project_note(req: SaveProjectNoteRequest) -> Result<(), String> {
    let project_id = uuid::Uuid::parse_str(&req.project_id)
        .map_err(|_| format!("ID inválido: {}", req.project_id))?;

    validate_project_exists(project_id)?;
    write_note(&get_project_note_path(project_id), &req.content)
}

#[tauri::command]
pub async fn get_ecosystem_note(req: EcosystemNoteRequest) -> Result<String, String> {
    let ecosystem_id = uuid::Uuid::parse_str(&req.ecosystem_id)
        .map_err(|_| format!("ID inválido: {}", req.ecosystem_id))?;

    validate_ecosystem_exists(ecosystem_id)?;
    read_note(&get_ecosystem_note_path(ecosystem_id))
}

#[tauri::command]
pub async fn save_ecosystem_note(req: SaveEcosystemNoteRequest) -> Result<(), String> {
    let ecosystem_id = uuid::Uuid::parse_str(&req.ecosystem_id)
        .map_err(|_| format!("ID inválido: {}", req.ecosystem_id))?;

    validate_ecosystem_exists(ecosystem_id)?;
    write_note(&get_ecosystem_note_path(ecosystem_id), &req.content)
}

#[tauri::command]
pub async fn get_general_note() -> Result<String, String> {
    read_note(&get_general_note_path())
}

#[tauri::command]
pub async fn save_general_note(req: SaveGeneralNoteRequest) -> Result<(), String> {
    write_note(&get_general_note_path(), &req.content)
}
