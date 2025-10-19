mod commands;
mod db;
mod models;
mod services;

use std::error::Error;
use services::sync_engine::SyncEngine;
use sqlx::SqlitePool;
use tokio::sync::Mutex;

pub struct AppState {
    pub sync_engine: Mutex<SyncEngine>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    tauri::Builder::default()
        .manage(AppState {
            // Placeholder for SqlitePool
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
        .setup(|app| Box::pin(async move {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        }))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
