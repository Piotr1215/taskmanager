use crate::domain::Task; // <-- Make sure to import Task
use std::io::Result;

pub struct TaskManager<T: TaskRepository> {
    pub repository: T,
}

pub trait TaskRepository {
    fn save(&self, tasks: Vec<Task>) -> Result<()>; // <-- Corrected Result typeport
    fn retrieve(&self, file_path: &str) -> Result<Vec<Task>>; // <-- Corrected Result type
}

pub trait TaskOperations {
    fn add_task(&self, description: &str) -> Result<Task>; // <-- Corrected Result type
    fn list_tasks(&self) -> Result<Vec<Task>>; // <-- Corrected Result type
    fn delete_task(&self, task_id: &str, tasks: &mut Vec<Task>) -> Result<Task>; // <-- Corrected Result type
}
