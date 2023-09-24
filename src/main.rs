use adapters::FileRepository;
// Import only what's necessary to keep the code clean.
use clap::{Parser, Subcommand};
mod adapters;
mod domain;
mod ports;
use domain::Task;
use ports::{TaskManager, TaskOperations, TaskRepository};
use std::env;
#[allow(unused_imports)]
use std::io::Result;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// add a task
    Add {
        /// description of the task
        description: String,
    },
    /// list all tasks
    List {},
    /// mark a task as done
    Done { task_id: String },
}

// Constants are usually in SCREAMING_SNAKE_CASE.
const TASKS_FILE: &str = ".tasks.json";

fn main() {
    // Fetch the HOME environment variable
    let home = env::var("HOME").expect("HOME variable not set");
    // combine the TASKS_FILE with the home directory
    let task_file_path = format!("{}/{}", home, TASKS_FILE);
    let cli = Cli::parse();
    // Initialize FileRepository with the file path
    let task_repository = FileRepository {
        file_path: task_file_path,
    };
    let manager = TaskManager {
        repository: task_repository.clone(),
    };

    // Retrieve tasks from the file
    let mut tasks = match task_repository.retrieve(TASKS_FILE) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Failed to retrieve tasks: {}", e);
            vec![]
        }
    };

    // Get the home directory dynamically.
    // let home_dir = env::home_dir().expect("Unable to get home directory");
    // let default_file_path = format!("{}/.tasks.txt", home_dir.display());
    match &cli.command {
        Some(Commands::Add { description }) => {
            println!("Adding task: {}", description);
            let new_task = Task::new(description);
            let _ = &tasks.push(new_task.clone());
            {
                if let Err(e) = task_repository.save(tasks) {
                    eprintln!("Failed to save task: {}", e);
                    println!("Failed to save {}", new_task.description);
                }
            }
        }
        Some(Commands::List {}) => {
            println!("Listing all tasks");
            match task_repository.retrieve(TASKS_FILE) {
                Ok(tasks) => {
                    for task in tasks {
                        println!("Task: {:?}", task);
                    }
                }
                Err(e) => eprintln!("Failed to list tasks: {}", e),
            }
        }
        Some(Commands::Done { task_id }) => {
            println!("Mark task as done");
            dbg!(&tasks);
            // Step 1: Mark the task as done using TaskManager
            match &manager.delete_task(task_id, &mut tasks) {
                Ok(done_task) => {
                    println!("Successfully marked task as done: {:?}", done_task);

                    if let Err(e) = task_repository.save(tasks.clone()) {
                        eprintln!("Failed to save updated tasks: {}", e);
                    }
                    dbg!(&tasks);
                }
                Err(e) => {
                    eprintln!("Failed to mark task as done: {}", e);
                }
            }
        }
        None => {}
    }
}
#[cfg(test)]
struct MockTaskRepository {
    tasks: Vec<Task>,
}

#[cfg(test)]
impl TaskRepository for MockTaskRepository {
    fn retrieve(&self, _file_path: &str) -> Result<Vec<Task>> {
        Ok(self.tasks.clone())
    }

    fn save(&self, _tasks: Vec<Task>) -> Result<()> {
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_tasks() {
        let mut mock_repo = MockTaskRepository { tasks: Vec::new() };
        let new_task = Task::new("Learn Rust");
        mock_repo.tasks.push(new_task.clone());

        let result = mock_repo.retrieve(TASKS_FILE);
        assert!(result.is_ok());

        let tasks = result.unwrap();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0], new_task);
    }
}
