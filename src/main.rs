// Import only what's necessary to keep the code clean.
use clap::{Parser, Subcommand};
use std::{
    fs::{File, OpenOptions},
    io::{self, BufRead, Result, Write},
    path::Path,
};
use uuid::Uuid;
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
// Domain {{{
// Using #[derive(Debug, Clone)]
#[derive(Debug, Clone)]
struct Task {
    id: String,
    description: String,
    status: String,
}
impl Task {
    pub fn new(description: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            description: description.to_string(),
            status: "pending".to_string(),
        }
    }
}
//}}}

// Constants are usually in SCREAMING_SNAKE_CASE.
const FILE_PATH: &str = "/home/decoder/.tasks.txt";

// Use Result for error handling.

fn save_tasks(file_path: &str, tasks: &[Task]) -> Result<()> {
    println!("Saving tasks to {}", file_path);

    // Open the file in append mode, or create it if it doesn't exist
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(file_path)?;

    // Iterate over tasks and write to file
    for task in tasks {
        let task_str = format!(
            "ID: {}, Description: {}, Status: {}\n",
            task.id, task.description, task.status
        );
        file.write_all(task_str.as_bytes())?;
    }

    Ok(())
}

// Use slices instead of Vec when you don't need to modify the vector.
fn list_tasks(file_path: &str) -> io::Result<()> {
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

    Ok(())
}

// Use `&mut Vec<Task>` to modify the vector in place.
fn add_task(description: &str, tasks: &mut Vec<Task>) {
    let task = Task::new(description);
    println!("Task {} created successfully", task.description);
    tasks.push(task);

    // Handle errors using `if let`.
    if let Err(e) = save_tasks(FILE_PATH, tasks) {
        eprintln!("Failed to save tasks: {}", e);
    }
}

fn main() {
    let cli = Cli::parse();
    // Initialize an empty vector of Task.
    let mut tasks: Vec<Task> = Vec::new();

    // Get the home directory dynamically.
    // let home_dir = env::home_dir().expect("Unable to get home directory");
    // let default_file_path = format!("{}/.tasks.txt", home_dir.display());
    match &cli.command {
        Some(Commands::Add { description }) => {
            println!("Adding task: {}", description);
            add_task(description, &mut tasks);
        }
        Some(Commands::List {}) => {
            println!("Listing all tasks");
            if let Err(e) = list_tasks(FILE_PATH) {
                eprintln!("Failed to list tasks: {}", e);
            }
        }
        Some(Commands::Save { file_path }) => {
            println!("Saving tasks to {}", file_path);
            if let Err(e) = save_tasks(file_path, &tasks) {
                eprintln!("Failed to save tasks: {}", e);
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
mod tests {
    use super::*;

    #[test]
    fn test_add_task() {
        let mut tasks: Vec<Task> = Vec::new();
        add_task("Learn Rust", &mut tasks);
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].description, "Learn Rust");
    }

    #[test]
    fn test_save_tasks() {
        let mut tasks: Vec<Task> = Vec::new();
        add_task("Learn Rust", &mut tasks);
        let result = save_tasks(FILE_PATH, &tasks);
        assert!(result.is_ok());
    }

    #[test]
    fn test_list_tasks() {
        let result = list_tasks(FILE_PATH);
        assert!(result.is_ok());
    }

    // Add more tests as needed
}
