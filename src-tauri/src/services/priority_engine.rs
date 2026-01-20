use crate::models::Task;
use chrono::Utc;

/// Engine for calculating and updating task priorities
pub struct PriorityEngine {
    decay_rate: f64,
}

impl PriorityEngine {
    pub fn new(decay_rate: f64) -> Self {
        Self { decay_rate }
    }

    /// Calculate current priority with exponential decay
    /// P(t) = P0 * decay_rate^hours
    pub fn calculate_priority(&self, initial_priority: f64, hours_elapsed: f64) -> f64 {
        (initial_priority * self.decay_rate.powf(hours_elapsed)).max(0.1)
    }

    /// Update a task's current priority based on time elapsed
    pub fn update_task_priority(&self, task: &mut Task) {
        let hours_elapsed = (Utc::now() - task.created_at).num_minutes() as f64 / 60.0;
        task.current_priority = self.calculate_priority(task.initial_priority, hours_elapsed);
    }

    /// Sort tasks by current priority (highest first)
    pub fn sort_by_priority(&self, tasks: &mut [Task]) {
        for task in tasks.iter_mut() {
            self.update_task_priority(task);
        }
        tasks.sort_by(|a, b| {
            b.current_priority
                .partial_cmp(&a.current_priority)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
    }
}

impl Default for PriorityEngine {
    fn default() -> Self {
        Self::new(0.95) // 5% decay per hour
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority_decay() {
        let engine = PriorityEngine::new(0.95);

        // At t=0, priority should be unchanged
        let p0 = engine.calculate_priority(1.0, 0.0);
        assert!((p0 - 1.0).abs() < 0.001);

        // After 1 hour, should be 0.95
        let p1 = engine.calculate_priority(1.0, 1.0);
        assert!((p1 - 0.95).abs() < 0.001);

        // After 10 hours, should be ~0.60
        let p10 = engine.calculate_priority(1.0, 10.0);
        assert!((p10 - 0.5987).abs() < 0.01);

        // Should never go below 0.1
        let p_min = engine.calculate_priority(0.05, 100.0);
        assert!((p_min - 0.1).abs() < 0.001);
    }
}
