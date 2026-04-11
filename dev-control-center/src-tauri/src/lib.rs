pub mod commands {
    pub mod projects;
    pub mod actions;
}

pub mod models {
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
            commands::actions::open_editor,
            commands::actions::launch_agent,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
