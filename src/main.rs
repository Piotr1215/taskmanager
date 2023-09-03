use adapters::FileRepository;
// Import only what's necessary to keep the code clean.
use clap::{Parser, Subcommand};
mod adapters;
mod domain;
mod ports;
use domain::Task;
use ports::{TaskManager, TaskOperations, TaskRepository};
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
    /// does testing things
    Add {
        /// lists test values
        description: String,
    },
    List {},
    Done {
        task_id: String,
    },
}

// Constants are usually in SCREAMING_SNAKE_CASE.
const FILE_PATH: &str = "/home/decoder/.tasks.json";

fn main() {
    let cli = Cli::parse();
    // Initialize FileRepository with the file path
    let task_repository = FileRepository {
        file_path: FILE_PATH.to_string(),
    };
    let mut manager = TaskManager {
        repository: task_repository.clone(),
    };

    // Retrieve tasks from the file
    let mut tasks = match task_repository.retrieve(FILE_PATH) {
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
                }
            }
        }
        Some(Commands::List {}) => {
            println!("Listing all tasks");
            match task_repository.retrieve(FILE_PATH) {
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

        let result = mock_repo.retrieve(FILE_PATH);
        assert!(result.is_ok());

        let tasks = result.unwrap();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0], new_task);
    }
}
