use crate::models::{Settings, WatchStatus};
use crate::storage::{ContextRepository, Database, TaskRepository};
use anyhow::Result;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

pub struct AppState {
    database: Database,
    task_repository: TaskRepository,
    context_repository: ContextRepository,
    watch_status: Arc<RwLock<WatchStatus>>,
    settings: Arc<RwLock<Settings>>,
    app_data_dir: PathBuf,
}

impl AppState {
    pub fn new(app_data_dir: PathBuf) -> Result<Self> {
        let database = Database::new(app_data_dir.clone())?;
        let task_repository = TaskRepository::new(database.connection());
        let context_repository = ContextRepository::new(database.connection());

        // Load settings from disk or use defaults
        let settings = Self::load_settings(&app_data_dir).unwrap_or_default();

        Ok(Self {
            database,
            task_repository,
            context_repository,
            watch_status: Arc::new(RwLock::new(WatchStatus::default())),
            settings: Arc::new(RwLock::new(settings)),
            app_data_dir,
        })
    }

    pub fn task_repository(&self) -> &TaskRepository {
        &self.task_repository
    }

    pub fn context_repository(&self) -> &ContextRepository {
        &self.context_repository
    }

    pub fn get_watch_status(&self) -> WatchStatus {
        self.watch_status.read().unwrap().clone()
    }

    pub fn set_watching(&self, enabled: bool) {
        let mut status = self.watch_status.write().unwrap();
        status.is_watching = enabled;
        if !enabled {
            status.captures_since_start = 0;
            status.tasks_detected_since_start = 0;
        }
    }

    pub fn increment_captures(&self) {
        let mut status = self.watch_status.write().unwrap();
        status.captures_since_start += 1;
        status.last_capture_at = Some(chrono::Utc::now());
    }

    pub fn increment_tasks_detected(&self, count: u64) {
        let mut status = self.watch_status.write().unwrap();
        status.tasks_detected_since_start += count;
    }

    pub fn get_settings(&self) -> Result<Settings> {
        Ok(self.settings.read().unwrap().clone())
    }

    pub fn save_settings(&self, settings: &Settings) -> Result<()> {
        // Update in memory
        {
            let mut current = self.settings.write().unwrap();
            *current = settings.clone();
        }

        // Persist to disk
        let settings_path = self.app_data_dir.join("settings.json");
        let json = serde_json::to_string_pretty(settings)?;
        std::fs::write(settings_path, json)?;

        Ok(())
    }

    fn load_settings(app_data_dir: &PathBuf) -> Result<Settings> {
        let settings_path = app_data_dir.join("settings.json");
        if settings_path.exists() {
            let json = std::fs::read_to_string(settings_path)?;
            let settings: Settings = serde_json::from_str(&json)?;
            Ok(settings)
        } else {
            Ok(Settings::default())
        }
    }
}
