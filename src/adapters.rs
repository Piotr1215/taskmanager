use crate::domain::Task;
use crate::ports::{TaskManager, TaskOperations, TaskRepository};
use crate::FILE_PATH;
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::io::{Read, Result};
use std::path::Path;

#[derive(Clone)]
pub struct FileRepository {
    pub file_path: String,
}

impl TaskRepository for FileRepository {
    fn save(&self, tasks: Vec<Task>) -> io::Result<()> {
        let file_path = Path::new(&self.file_path);
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(file_path)?;

        let data: String = serde_json::to_string(&tasks)?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }

    fn retrieve(&self, _file_path: &str) -> Result<Vec<Task>> {
        let file_path = Path::new(&self.file_path);
        let mut file = File::open(file_path)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;
        let tasks: Vec<Task> = serde_json::from_str(&data)?;
        Ok(tasks)
    }
}
impl<T: TaskRepository> TaskOperations for TaskManager<T> {
    fn delete_task(&self, task_id: &str, tasks: &mut Vec<Task>) -> io::Result<Task> {
        // No need to load tasks from the repository now, as we're passing it in

        // Find the task by ID and mark it as done
        if let Some(index) = tasks.iter().position(|t| t.id == task_id) {
            let task = tasks[index].clone();
            let done_task = task.done();
            tasks[index] = done_task.clone();

            // Save the updated tasks back to the repository
            self.repository.save(tasks.to_vec())?;

            Ok(done_task)
        } else {
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Task not found".to_string(),
            ))
        }
    }

    fn add_task(&self, _description: &str) -> Result<Task> {
        Ok(Task::new(_description))
    }

    fn list_tasks(&self) -> Result<Vec<Task>> {
        self.repository.retrieve("")
    }
}
