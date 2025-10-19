use sqlx::SqlitePool;
use crate::models::operation::Operation;
use uuid::Uuid;
use crate::services::album::AlbumService;

pub struct SyncEngine {
    pub primary_db: SqlitePool,
    pub backup_db: Option<SqlitePool>,
    pub operation_queue: Vec<Operation>,
}

impl SyncEngine {
    pub fn new(primary_db: SqlitePool, backup_db: Option<SqlitePool>) -> Self {
        Self {
            primary_db,
            backup_db,
            operation_queue: Vec::new(),
        }
    }

    pub async fn log_operation(&self, op: &Operation) -> Result<String, sqlx::Error> {
        let op_id = Uuid::new_v4().to_string();
        let op_type = match op {
            Operation::Move { .. } => "move",
            Operation::Delete { .. } => "delete",
            Operation::Rename { .. } => "rename",
            Operation::CreateAlbum { .. } => "create_album",
            Operation::AddToAlbum { .. } => "add_to_album",
            Operation::AddTag { .. } => "add_tag",
        };
        let params = serde_json::to_string(op).unwrap_or_default();

        sqlx::query(
            "INSERT INTO sync_operations (id, operation_type, params) VALUES (?, ?, ?)",
        )
        .bind(&op_id)
        .bind(op_type)
        .bind(params)
        .execute(&self.primary_db)
        .await?;

        Ok(op_id)
    }

    pub async fn execute_operation(&mut self, op: Operation) -> Result<(), String> {
        let op_id = self.log_operation(&op).await.map_err(|e| e.to_string())?;
        
        if self.backup_db.is_some() {
            self.execute_on_both(&op).await?;
        } else {
            self.handle_backup_disconnected(op).await?;
        }

        // Here we would update the status of the operation in the database
        sqlx::query("UPDATE sync_operations SET status = 'completed' WHERE id = ?")
            .bind(op_id)
            .execute(&self.primary_db)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn execute_on_primary(&self, op: &Operation) -> Result<(), sqlx::Error> {
        let album_service = AlbumService::new(self.primary_db.clone());
        match op {
            Operation::CreateAlbum { name } => {
                album_service.create_album(name.clone()).await?;
            }
            Operation::AddToAlbum { photo_id, album_id } => {
                album_service.add_photos_to_album(vec![*photo_id], *album_id).await?;
            }
            // ... other operations
            _ => {
                println!("Executing on primary: {:?}", op);
            }
        }
        Ok(())
    }

    async fn execute_on_backup(&self, op: &Operation) -> Result<(), sqlx::Error> {
        if let Some(backup_db) = &self.backup_db {
            let album_service = AlbumService::new(backup_db.clone());
            match op {
                Operation::CreateAlbum { name } => {
                    album_service.create_album(name.clone()).await?;
                }
                Operation::AddToAlbum { photo_id, album_id } => {
                    album_service.add_photos_to_album(vec![*photo_id], *album_id).await?;
                }
                // ... other operations
                _ => {
                    println!("Executing on backup: {:?}", op);
                }
            }
        }
        Ok(())
    }

    pub async fn execute_on_both(&mut self, op: &Operation) -> Result<(), String> {
        if let Err(e) = self.execute_on_primary(op).await {
            return Err(format!("Failed to execute on primary: {}", e));
        }

        if let Some(backup_db) = &self.backup_db {
            if let Err(e) = self.execute_on_backup(op).await {
                println!("Failed to execute on backup: {}. Queuing operation.", e);
                self.operation_queue.push(op.clone());
            }
        } else {
            self.operation_queue.push(op.clone());
        }

        Ok(())
    }

    pub async fn handle_backup_disconnected(&mut self, op: Operation) -> Result<(), String> {
        println!("Backup disconnected. Queuing operation: {:?}", op);
        self.operation_queue.push(op);
        Ok(())
    }

    pub async fn flush_queue(&mut self) -> Result<(), String> {
        println!("Flushing operation queue...");
        let ops: Vec<Operation> = self.operation_queue.drain(..).collect();
        for op in ops {
            self.execute_on_both(&op).await?;
        }
        println!("Operation queue flushed.");
        Ok(())
    }
}