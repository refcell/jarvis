use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A task detected from screen context by the LLM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub context: String,
    pub initial_priority: f64,
    pub current_priority: f64,
    pub status: TaskStatus,
    pub source_window: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub snoozed_until: Option<DateTime<Utc>>,
}

impl Task {
    pub fn new(title: String, description: String, context: String, priority: f64) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            title,
            description,
            context,
            initial_priority: priority,
            current_priority: priority,
            status: TaskStatus::Pending,
            source_window: None,
            created_at: now,
            updated_at: now,
            snoozed_until: None,
        }
    }

    /// Calculate current priority with exponential decay
    /// P(t) = P0 * 0.95^hours
    pub fn calculate_priority(&self) -> f64 {
        let hours_elapsed = (Utc::now() - self.created_at).num_minutes() as f64 / 60.0;
        (self.initial_priority * 0.95_f64.powf(hours_elapsed)).max(0.1)
    }

    pub fn with_source_window(mut self, window: String) -> Self {
        self.source_window = Some(window);
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Dismissed,
    Snoozed,
}

impl TaskStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            TaskStatus::Pending => "pending",
            TaskStatus::InProgress => "in_progress",
            TaskStatus::Completed => "completed",
            TaskStatus::Dismissed => "dismissed",
            TaskStatus::Snoozed => "snoozed",
        }
    }
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// A task detected by the LLM before being saved
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedTask {
    pub title: String,
    pub description: String,
    pub priority: f64,
}

impl DetectedTask {
    pub fn into_task(self, context: String) -> Task {
        Task::new(self.title, self.description, context, self.priority)
    }
}
