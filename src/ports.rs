use crate::domain::Task; // <-- Make sure to import Task
use std::io::Result;

pub trait TaskRepository {
    fn save(&mut self, file_path: String, task: Task) -> Result<()>; // <-- Corrected Result typeport
    fn list(&self, file_path: &str) -> Result<Vec<Task>>; // <-- Corrected Result type
}

pub trait TaskOperations {
    fn add_task(&self, description: &str) -> Result<Task>; // <-- Corrected Result type
    fn list_tasks(&self) -> Result<Vec<Task>>; // <-- Corrected Result type
}
