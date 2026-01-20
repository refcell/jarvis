use crate::models::{Task, TaskStatus};
use anyhow::Result;
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

pub struct TaskRepository {
    conn: Arc<Mutex<Connection>>,
}

impl TaskRepository {
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self { conn }
    }

    pub fn insert(&self, task: &Task) -> Result<()> {
        let conn = self.conn.lock().map_err(|_| anyhow::anyhow!("Lock poisoned"))?;

        conn.execute(
            "INSERT INTO tasks (id, title, description, context, initial_priority, current_priority, status, source_window, created_at, updated_at, snoozed_until)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            params![
                task.id.to_string(),
                task.title,
                task.description,
                task.context,
                task.initial_priority,
                task.current_priority,
                task.status.as_str(),
                task.source_window,
                task.created_at.to_rfc3339(),
                task.updated_at.to_rfc3339(),
                task.snoozed_until.map(|dt| dt.to_rfc3339()),
            ],
        )?;

        Ok(())
    }

    pub fn update(&self, task: &Task) -> Result<()> {
        let conn = self.conn.lock().map_err(|_| anyhow::anyhow!("Lock poisoned"))?;

        conn.execute(
            "UPDATE tasks SET title = ?2, description = ?3, context = ?4, initial_priority = ?5, current_priority = ?6, status = ?7, source_window = ?8, updated_at = ?9, snoozed_until = ?10
             WHERE id = ?1",
            params![
                task.id.to_string(),
                task.title,
                task.description,
                task.context,
                task.initial_priority,
                task.current_priority,
                task.status.as_str(),
                task.source_window,
                Utc::now().to_rfc3339(),
                task.snoozed_until.map(|dt| dt.to_rfc3339()),
            ],
        )?;

        Ok(())
    }

    pub fn get_by_id(&self, id: Uuid) -> Result<Option<Task>> {
        let conn = self.conn.lock().map_err(|_| anyhow::anyhow!("Lock poisoned"))?;

        let mut stmt = conn.prepare(
            "SELECT id, title, description, context, initial_priority, current_priority, status, source_window, created_at, updated_at, snoozed_until
             FROM tasks WHERE id = ?1",
        )?;

        let mut rows = stmt.query(params![id.to_string()])?;

        if let Some(row) = rows.next()? {
            Ok(Some(Self::row_to_task(row)?))
        } else {
            Ok(None)
        }
    }

    pub fn get_active(&self) -> Result<Vec<Task>> {
        let conn = self.conn.lock().map_err(|_| anyhow::anyhow!("Lock poisoned"))?;

        let mut stmt = conn.prepare(
            "SELECT id, title, description, context, initial_priority, current_priority, status, source_window, created_at, updated_at, snoozed_until
             FROM tasks
             WHERE status IN ('pending', 'in_progress', 'snoozed')
             ORDER BY current_priority DESC",
        )?;

        let rows = stmt.query_map([], |row| Ok(Self::row_to_task(row)))?;

        let mut tasks = vec![];
        for row in rows {
            tasks.push(row??);
        }

        Ok(tasks)
    }

    pub fn get_all(&self) -> Result<Vec<Task>> {
        let conn = self.conn.lock().map_err(|_| anyhow::anyhow!("Lock poisoned"))?;

        let mut stmt = conn.prepare(
            "SELECT id, title, description, context, initial_priority, current_priority, status, source_window, created_at, updated_at, snoozed_until
             FROM tasks
             ORDER BY created_at DESC",
        )?;

        let rows = stmt.query_map([], |row| Ok(Self::row_to_task(row)))?;

        let mut tasks = vec![];
        for row in rows {
            tasks.push(row??);
        }

        Ok(tasks)
    }

    pub fn delete(&self, id: Uuid) -> Result<()> {
        let conn = self.conn.lock().map_err(|_| anyhow::anyhow!("Lock poisoned"))?;
        conn.execute("DELETE FROM tasks WHERE id = ?1", params![id.to_string()])?;
        Ok(())
    }

    fn row_to_task(row: &rusqlite::Row) -> Result<Task> {
        let id_str: String = row.get(0)?;
        let status_str: String = row.get(6)?;
        let created_at_str: String = row.get(8)?;
        let updated_at_str: String = row.get(9)?;
        let snoozed_until_str: Option<String> = row.get(10)?;

        Ok(Task {
            id: Uuid::parse_str(&id_str)?,
            title: row.get(1)?,
            description: row.get(2)?,
            context: row.get(3)?,
            initial_priority: row.get(4)?,
            current_priority: row.get(5)?,
            status: match status_str.as_str() {
                "pending" => TaskStatus::Pending,
                "in_progress" => TaskStatus::InProgress,
                "completed" => TaskStatus::Completed,
                "dismissed" => TaskStatus::Dismissed,
                "snoozed" => TaskStatus::Snoozed,
                _ => TaskStatus::Pending,
            },
            source_window: row.get(7)?,
            created_at: DateTime::parse_from_rfc3339(&created_at_str)?.with_timezone(&Utc),
            updated_at: DateTime::parse_from_rfc3339(&updated_at_str)?.with_timezone(&Utc),
            snoozed_until: snoozed_until_str
                .map(|s| DateTime::parse_from_rfc3339(&s).map(|dt| dt.with_timezone(&Utc)))
                .transpose()?,
        })
    }
}
