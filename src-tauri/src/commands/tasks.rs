use crate::models::{Task, TaskStatus};
use crate::services::PriorityEngine;
use crate::state::AppState;
use chrono::{Duration, Utc};
use tauri::State;
use uuid::Uuid;

#[tauri::command]
pub fn get_active_tasks(state: State<'_, AppState>) -> Result<Vec<Task>, String> {
    let mut tasks = state
        .task_repository()
        .get_active()
        .map_err(|e| e.to_string())?;

    // Update priorities
    let engine = PriorityEngine::default();
    engine.sort_by_priority(&mut tasks);

    // Filter out snoozed tasks that are still snoozed
    let now = Utc::now();
    tasks.retain(|t| {
        if t.status == TaskStatus::Snoozed {
            t.snoozed_until.map(|until| until <= now).unwrap_or(true)
        } else {
            true
        }
    });

    Ok(tasks)
}

#[tauri::command]
pub fn get_all_tasks(state: State<'_, AppState>) -> Result<Vec<Task>, String> {
    state
        .task_repository()
        .get_all()
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_task(state: State<'_, AppState>, id: String) -> Result<Option<Task>, String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    state
        .task_repository()
        .get_by_id(uuid)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn create_task(
    state: State<'_, AppState>,
    title: String,
    description: String,
    context: String,
    priority: f64,
) -> Result<Task, String> {
    let task = Task::new(title, description, context, priority);
    state
        .task_repository()
        .insert(&task)
        .map_err(|e| e.to_string())?;
    Ok(task)
}

#[tauri::command]
pub fn update_task_status(
    state: State<'_, AppState>,
    id: String,
    status: String,
) -> Result<Task, String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;

    let mut task = state
        .task_repository()
        .get_by_id(uuid)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Task not found".to_string())?;

    task.status = match status.as_str() {
        "pending" => TaskStatus::Pending,
        "in_progress" => TaskStatus::InProgress,
        "completed" => TaskStatus::Completed,
        "dismissed" => TaskStatus::Dismissed,
        "snoozed" => TaskStatus::Snoozed,
        _ => return Err("Invalid status".to_string()),
    };

    state
        .task_repository()
        .update(&task)
        .map_err(|e| e.to_string())?;

    Ok(task)
}

#[tauri::command]
pub fn snooze_task(
    state: State<'_, AppState>,
    id: String,
    hours: i64,
) -> Result<Task, String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;

    let mut task = state
        .task_repository()
        .get_by_id(uuid)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Task not found".to_string())?;

    task.status = TaskStatus::Snoozed;
    task.snoozed_until = Some(Utc::now() + Duration::hours(hours));

    state
        .task_repository()
        .update(&task)
        .map_err(|e| e.to_string())?;

    Ok(task)
}

#[tauri::command]
pub fn dismiss_task(state: State<'_, AppState>, id: String) -> Result<(), String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;

    let mut task = state
        .task_repository()
        .get_by_id(uuid)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Task not found".to_string())?;

    task.status = TaskStatus::Dismissed;

    state
        .task_repository()
        .update(&task)
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn complete_task(state: State<'_, AppState>, id: String) -> Result<(), String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;

    let mut task = state
        .task_repository()
        .get_by_id(uuid)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Task not found".to_string())?;

    task.status = TaskStatus::Completed;

    state
        .task_repository()
        .update(&task)
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn delete_task(state: State<'_, AppState>, id: String) -> Result<(), String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    state
        .task_repository()
        .delete(uuid)
        .map_err(|e| e.to_string())
}
