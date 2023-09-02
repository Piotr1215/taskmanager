use crate::domain::Task;
use crate::ports::TaskRepository;
use std::fs::{File, OpenOptions};
use std::io::Result;
use std::io::{self, BufRead, Write};
use std::path::Path;

impl TaskRepository for Vec<Task> {
    fn save(&mut self, file_path: String, task: Task) -> Result<()> {
        println!("Saving task to {}", file_path);

        // Open the file in append mode, or create it if it doesn't exist
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(file_path)?;

        // Write the single task to file
        let task_str = format!(
            "ID: {}, Description: {}, Status: {}\n",
            task.id, task.description, task.status
        );
        file.write_all(task_str.as_bytes())?;

        // Optionally, add the task to self.tasks
        self.push(task);

        Ok(())
    }

    fn list(&self, file_path: &str) -> Result<Vec<Task>> {
        let path = Path::new(file_path);
        let file = File::open(path)?;
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            match line {
                Ok(task) => {
                    println!("Task: {}", task);
                }
                Err(e) => {
                    eprintln!("Error reading line: {}", e);
                }
            }
        }

        Ok(self.clone())
    }
}
