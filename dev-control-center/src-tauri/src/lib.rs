pub mod commands {
    pub mod projects;
    pub mod ecosystems;
    pub mod actions;
    pub mod sessions;
    pub mod notes;
}

pub mod logging;
pub mod process;
pub mod models {
    pub mod ecosystem;
    pub mod project;
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::projects::get_projects,
            commands::projects::create_project,
            commands::projects::update_project,
            commands::projects::delete_project,
            commands::ecosystems::get_ecosystems,
            commands::ecosystems::create_ecosystem,
            commands::ecosystems::scan_ecosystem_folder,
            commands::ecosystems::import_ecosystem_folder,
            commands::ecosystems::update_ecosystem,
            commands::ecosystems::delete_ecosystem,
            commands::actions::open_editor,
            commands::actions::open_ecosystem_editor,
            commands::actions::launch_agent,
            commands::actions::launch_ecosystem_agent,
            commands::actions::resume_agent_session,
            commands::actions::resume_ecosystem_agent_session,
            commands::actions::open_terminal,
            commands::actions::open_global_terminal,
            commands::actions::open_agent_settings,
            commands::sessions::get_sessions,
            commands::notes::get_project_note,
            commands::notes::save_project_note,
            commands::notes::get_ecosystem_note,
            commands::notes::save_ecosystem_note,
            commands::notes::get_general_note,
            commands::notes::save_general_note,
            commands::projects::pick_directory,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
