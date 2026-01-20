use crate::models::CaptureContext;
use anyhow::Result;
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

pub struct ContextRepository {
    conn: Arc<Mutex<Connection>>,
}

impl ContextRepository {
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self { conn }
    }

    pub fn insert(&self, context: &CaptureContext) -> Result<()> {
        let conn = self.conn.lock().map_err(|_| anyhow::anyhow!("Lock poisoned"))?;

        conn.execute(
            "INSERT INTO capture_contexts (id, ocr_text, active_window_title, active_app_name, captured_at, screen_width, screen_height)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                context.id.to_string(),
                context.ocr_text,
                context.active_window_title,
                context.active_app_name,
                context.captured_at.to_rfc3339(),
                context.screen_width,
                context.screen_height,
            ],
        )?;

        Ok(())
    }

    pub fn get_recent(&self, limit: usize) -> Result<Vec<CaptureContext>> {
        let conn = self.conn.lock().map_err(|_| anyhow::anyhow!("Lock poisoned"))?;

        let mut stmt = conn.prepare(
            "SELECT id, ocr_text, active_window_title, active_app_name, captured_at, screen_width, screen_height
             FROM capture_contexts
             ORDER BY captured_at DESC
             LIMIT ?1",
        )?;

        let rows = stmt.query_map(params![limit as i64], |row| {
            Ok(Self::row_to_context(row))
        })?;

        let mut contexts = vec![];
        for row in rows {
            contexts.push(row??);
        }

        Ok(contexts)
    }

    pub fn delete_older_than(&self, cutoff: DateTime<Utc>) -> Result<usize> {
        let conn = self.conn.lock().map_err(|_| anyhow::anyhow!("Lock poisoned"))?;
        let deleted = conn.execute(
            "DELETE FROM capture_contexts WHERE captured_at < ?1",
            params![cutoff.to_rfc3339()],
        )?;
        Ok(deleted)
    }

    fn row_to_context(row: &rusqlite::Row) -> Result<CaptureContext> {
        let id_str: String = row.get(0)?;
        let captured_at_str: String = row.get(4)?;

        Ok(CaptureContext {
            id: Uuid::parse_str(&id_str)?,
            ocr_text: row.get(1)?,
            active_window_title: row.get(2)?,
            active_app_name: row.get(3)?,
            captured_at: DateTime::parse_from_rfc3339(&captured_at_str)?.with_timezone(&Utc),
            screen_width: row.get(5)?,
            screen_height: row.get(6)?,
        })
    }
}
