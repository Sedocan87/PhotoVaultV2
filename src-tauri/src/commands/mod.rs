use crate::models::{album::Album, duplicate::DuplicateGroup, filter::FilterCriteria, operation::Operation, photo::Photo, rename::{RenamePreview, RenameResult}, restore::RestoreSummary, tag::Tag};
use crate::services::{duplicate::DuplicateDetector, filter::FilterService, rename::RenameService, restore::RestoreService, tags::TagService};
use crate::AppState;
use std::path::PathBuf;
use tauri::State;

#[derive(Debug, Clone, serde::Serialize)]
pub struct QueueStatus {
    pub pending_operations: usize,
}

#[tauri::command]
pub async fn scan_library(primary_path: String) -> Result<Vec<Photo>, String> {
    // Placeholder implementation
    println!("Scanning library at: {}", primary_path);
    Ok(vec![])
}

#[tauri::command]
pub async fn get_photos(limit: i64, offset: i64) -> Result<Vec<Photo>, String> {
    // Placeholder implementation
    println!("Getting photos with limit: {} and offset: {}", limit, offset);
    Ok(vec![])
}

#[tauri::command]
pub async fn move_photos(
    photo_ids: Vec<i64>,
    target_path: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    // This is a placeholder. In a real app, you would look up the photo paths from the database.
    let operation = Operation::Move {
        from: PathBuf::from("..."),
        to: PathBuf::from(target_path),
    };
    state.sync_engine.lock().await.execute_operation(operation).await
}

#[tauri::command]
pub async fn delete_photos(photo_ids: Vec<i64>, state: State<'_, AppState>) -> Result<(), String> {
    // This is a placeholder. In a real app, you would look up the photo paths from the database.
    let operation = Operation::Delete {
        path: PathBuf::from("..."),
    };
    state.sync_engine.lock().await.execute_operation(operation).await
}

#[tauri::command]
pub async fn rename_photo(
    photo_id: i64,
    new_name: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    // This is a placeholder. In a real app, you would look up the photo path from the database.
    let operation = Operation::Rename {
        path: PathBuf::from("..."),
        new_name,
    };
    state.sync_engine.lock().await.execute_operation(operation).await
}

#[tauri::command]
pub async fn get_sync_queue_status(state: State<'_, AppState>) -> Result<QueueStatus, String> {
    let pending_operations = state.sync_engine.lock().await.operation_queue.len();
    Ok(QueueStatus {
        pending_operations,
    })
}

#[tauri::command]
pub async fn create_album(name: String, state: State<'_, AppState>) -> Result<(), String> {
    let operation = Operation::CreateAlbum { name };
    state.sync_engine.lock().await.execute_operation(operation).await
}

#[tauri::command]
pub async fn add_photos_to_album(
    photo_ids: Vec<i64>,
    album_id: i64,
    state: State<'_, AppState>,
) -> Result<(), String> {
    for photo_id in photo_ids {
        let operation = Operation::AddToAlbum { photo_id, album_id };
        state.sync_engine.lock().await.execute_operation(operation).await?;
    }
    Ok(())
}

#[tauri::command]
pub async fn get_albums(state: State<'_, AppState>) -> Result<Vec<Album>, String> {
    let pool = state.sync_engine.lock().await.primary_db.clone();
    let albums = sqlx::query_as::<_, Album>("SELECT * FROM albums")
        .fetch_all(&pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(albums)
}

#[tauri::command]
pub async fn delete_album(album_id: i64, state: State<'_, AppState>) -> Result<(), String> {
    // This should also be an operation, but for simplicity, we'll just delete it directly.
    let pool = state.sync_engine.lock().await.primary_db.clone();
    sqlx::query("DELETE FROM photo_album WHERE album_id = ?")
        .bind(album_id)
        .execute(&pool)
        .await
        .map_err(|e| e.to_string())?;
    sqlx::query("DELETE FROM albums WHERE id = ?")
        .bind(album_id)
        .execute(&pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn add_tag(photo_id: i64, tag_name: String, state: State<'_, AppState>) -> Result<(), String> {
    let pool = state.sync_engine.lock().await.primary_db.clone();
    let tag_service = TagService::new(pool);
    tag_service.add_tag(photo_id, tag_name).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_all_tags(state: State<'_, AppState>) -> Result<Vec<Tag>, String> {
    let pool = state.sync_engine.lock().await.primary_db.clone();
    let tags = sqlx::query_as::<_, Tag>("SELECT * FROM tags")
        .fetch_all(&pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(tags)
}

#[tauri::command]
pub async fn filter_photos(
    criteria: FilterCriteria,
    state: State<'_, AppState>,
) -> Result<Vec<Photo>, String> {
    let pool = state.sync_engine.lock().await.primary_db.clone();
    let filter_service = FilterService::new(pool);
    filter_service.filter_photos(criteria).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn search_photos(query: String, state: State<'_, AppState>) -> Result<Vec<Photo>, String> {
    let pool = state.sync_engine.lock().await.primary_db.clone();
    let criteria = FilterCriteria {
        query: Some(query),
        ..Default::default()
    };
    let filter_service = FilterService::new(pool);
    filter_service.filter_photos(criteria).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn find_duplicates() -> Result<Vec<DuplicateGroup>, String> {
    DuplicateDetector::find_duplicates(0.9).await
}

#[tauri::command]
pub async fn delete_duplicates(photo_ids: Vec<i64>) -> Result<i64, String> {
    // returns space freed
    Ok(0)
}

#[tauri::command]
pub async fn bulk_rename(
    photo_ids: Vec<i64>,
    pattern: String,
) -> Result<Vec<RenameResult>, String> {
    RenameService::bulk_rename(photo_ids, pattern).await
}

#[tauri::command]
pub async fn preview_bulk_rename(
    photo_ids: Vec<i64>,
    pattern: String,
) -> Result<Vec<RenamePreview>, String> {
    RenameService::preview_bulk_rename(photo_ids, pattern).await
}

#[tauri::command]
pub async fn detect_backup_differences() -> Result<RestoreSummary, String> {
    RestoreService::detect_differences().await
}

#[tauri::command]
pub async fn restore_backup_to_primary() -> Result<(), String> {
    RestoreService::restore_backup_to_primary().await
}
