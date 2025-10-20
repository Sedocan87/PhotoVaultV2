mod commands;
mod db;
mod models;
mod services;

use services::sync_engine::SyncEngine;
use sqlx::SqlitePool;
use tauri::{AppHandle, Manager};
use tokio::sync::Mutex;

pub struct AppState {
    pub sync_engine: Mutex<SyncEngine>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState {
            sync_engine: Mutex::new(SyncEngine::new(
                SqlitePool::connect_lazy("sqlite::memory:").unwrap(),
                None,
            )),
        })
        .invoke_handler(tauri::generate_handler![
            commands::scan_library,
            commands::get_photos,
            commands::move_photos,
            commands::delete_photos,
            commands::rename_photo,
            commands::get_sync_queue_status,
            commands::create_album,
            commands::add_photos_to_album,
            commands::get_albums,
            commands::delete_album,
            commands::add_tag,
            commands::get_all_tags,
            commands::filter_photos,
            commands::search_photos
        ])
        .setup(|app| {
            let handle = app.handle().clone();
            tokio::spawn(async move {
                // Initialize the database
                let app_data_dir = handle
                    .path()
                    .app_data_dir()
                    .expect("Failed to get app data directory");
                let db_path = app_data_dir.join("photovault.db");
                let migrations_path = handle
                    .path()
                    .resolve("migrations", tauri::path::BaseDirectory::Resource)
                    .expect("Failed to resolve migrations path");

                let db_manager = db::manager::DatabaseManager::initialize(
                    db_path,
                    None,
                    &migrations_path,
                )
                .await
                .expect("Failed to initialize database");

                let app_state: tauri::State<AppState> = handle.state();
                let mut sync_engine = app_state.sync_engine.lock().await;
                *sync_engine = SyncEngine::new(db_manager.primary_db, db_manager.backup_db);
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
