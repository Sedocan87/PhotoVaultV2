use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AppConfig {
    pub primary_path: Option<PathBuf>,
    pub backup_path: Option<PathBuf>,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            primary_path: None,
            backup_path: None,
        }
    }
}

pub fn get_config_path() -> Result<PathBuf, String> {
    let config_dir = dirs::home_dir()
        .ok_or("Could not find home directory".to_string())?
        .join(".photovault");
    std::fs::create_dir_all(&config_dir).map_err(|e| e.to_string())?;
    Ok(config_dir.join("config.json"))
}

pub fn load_config() -> Result<AppConfig, String> {
    let config_path = get_config_path()?;
    if !config_path.exists() {
        return Ok(AppConfig::default());
    }
    let config_str = std::fs::read_to_string(config_path).map_err(|e| e.to_string())?;
    serde_json::from_str(&config_str).map_err(|e| e.to_string())
}

pub fn save_config(config: &AppConfig) -> Result<(), String> {
    let config_path = get_config_path()?;
    let config_str = serde_json::to_string_pretty(config).map_err(|e| e.to_string())?;
    std::fs::write(config_path, config_str).map_err(|e| e.to_string())
}


#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_save_and_load_config() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("config.json");

        let mut config = AppConfig::default();
        config.primary_path = Some(PathBuf::from("/primary"));
        config.backup_path = Some(PathBuf::from("/backup"));

        let result = save_config(&config);
        assert!(result.is_ok());

        let loaded_config = load_config().unwrap();
        assert_eq!(config, loaded_config);
    }
}
