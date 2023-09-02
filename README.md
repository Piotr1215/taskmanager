# Rust CLI Todo App 🦀

## About

This is a simple CLI Todo application written in Rust. The purpose of this repository is educational. Feel free to clone, explore, and expand upon this project.

## Features

- Add tasks
- List tasks
- Save tasks to a file
- Mark tasks as done (WIP)

## Architecture

The project follows the Ports and Adapters (Hexagonal) architecture. This allows for easy testing and future expansions of the project.

- `domain.rs`: Contains the core business logic and entities.
- `ports.rs`: Defines the primary and secondary ports (interfaces) for the application.
- `adapters.rs`: Implements the secondary ports, like file storage.

## Why Rust?

Rust is a systems programming language that is safe, concurrent, and practical. It's a great choice for building reliable and efficient software. This project is a great way to explore some of the features and paradigms that Rust offers.

## Getting Started

1. Clone the repository

   ```bash
   git clone https://github.com/yourusername/rust-cli-todo.git

    Navigate to the project directory

    bash
   ```

`cd rust-cli-todo`

2. Build the project

`cargo build`

3. Run the project

`cargo run -- [COMMANDS]`

4. Contributing

This is an open-source project. Contributions are welcome!

5. License

MIT
