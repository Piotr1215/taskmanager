// Import only what's necessary to keep the code clean.
use clap::{Parser, Subcommand};
mod adapters;
mod domain;
mod ports;
use domain::Task;
use ports::TaskRepository;
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
    Save {
        #[arg(short, long, default_value = FILE_PATH)]
        file_path: String,
    },
    Done {
        #[arg(short, long)]
        task_id: String,
    },
}

// Constants are usually in SCREAMING_SNAKE_CASE.
const FILE_PATH: &str = "/home/decoder/.tasks.txt";

fn main() {
    let cli = Cli::parse();
    // Initialize an empty vector of Task.
    let mut task_repository: Vec<Task> = Vec::new();

    // Get the home directory dynamically.
    // let home_dir = env::home_dir().expect("Unable to get home directory");
    // let default_file_path = format!("{}/.tasks.txt", home_dir.display());
    match &cli.command {
        Some(Commands::Add { description }) => {
            println!("Adding task: {}", description);
            let new_task = Task::new(description);
            if let Err(e) = task_repository.save(FILE_PATH.to_string(), new_task) {
                eprintln!("Failed to save task: {}", e);
            }
        }
        Some(Commands::List {}) => {
            println!("Listing all tasks");
            match task_repository.list(FILE_PATH) {
                Ok(tasks) => {
                    for task in tasks {
                        println!("Task: {:?}", task);
                    }
                }
                Err(e) => eprintln!("Failed to list tasks: {}", e),
            }
        }
        Some(Commands::Save { file_path }) => {
            println!("Saving tasks to {}", file_path);
            let tasks_to_save: Vec<Task> = task_repository.clone(); // Clone the tasks to a new vector
            for task in tasks_to_save {
                if let Err(e) = task_repository.save(file_path.clone(), task) {
                    eprintln!("Failed to save task: {}", e);
                }
            }
        }
        Some(Commands::Done { task_id: _ }) => {
            println!("Mark task as done");
            //TODO: Actually mark task as done
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
    fn save(&mut self, _file_path: String, task: Task) -> Result<()> {
        self.tasks.push(task);
        Ok(())
    }

    fn list(&self, _file_path: &str) -> Result<Vec<Task>> {
        Ok(self.tasks.clone())
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_save_task() {
        let mut mock_repo = MockTaskRepository { tasks: Vec::new() };
        let new_task = Task::new("Learn Rust");
        let result = mock_repo.save(FILE_PATH.to_string(), new_task.clone());

        assert!(result.is_ok());
        assert_eq!(mock_repo.tasks.len(), 1);
        assert_eq!(mock_repo.tasks[0], new_task);
    }
    #[test]
    fn test_list_tasks() {
        let mut mock_repo = MockTaskRepository { tasks: Vec::new() };
        let new_task = Task::new("Learn Rust");
        mock_repo.tasks.push(new_task.clone());

        let result = mock_repo.list(FILE_PATH);
        assert!(result.is_ok());

        let tasks = result.unwrap();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0], new_task);
    }
}
