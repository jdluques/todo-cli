# todo-cli

`todo-cli` is a command-line tool for managing your to-do tasks efficiently. Built in Rust, `todo-cli` offers features like adding, editing, deleting, and managing tasks directly from your terminal. 

## Features

- **Add Tasks:** Quickly add tasks to your todo list.
- **Edit Tasks:** Update task name, status, or priority seamlessly.
- **Delete Tasks:** Remove tasks you no longer need with ease.
- **Dynamic Display:** Tasks are displayed dynamically using `crossterm` for an engaging user interface.
- **Configuration Management:** Persist configuration across sessions using `confy`.
- **JSON-based Storage:** Tasks are stored in JSON format, ensuring compatibility and simplicity, using `serde` and `serde-json`.
- **Table Display:** Task lists are presented in a visually appealing table format with `comfy-table`.
- **Priority-Based Ordering:** Tasks are automatically ordered by priority for optimal task management.
- **User-Friendly Interaction:** Interactive prompts powered by `inquire`.
- **CLI Flags Management:** Command-line arguments and options managed with `clap`.

# Installation Guide

## Prerequisites
Ensure you have [Rust and Cargo](https://www.rust-lang.org/tools/install) installed. Cargo, the Rust package manager, comes bundled with the Rust installation.

You can verify the installation by running:
```bash
rustc --version
cargo --version
```
## Option 1: Build from Source
1. Clone the repository:
   ```bash
   git clone https://github.com/jdluques/todo-cli.git
   cd todo-cli
2. Build with cargo:
   ```bash
   cargo build --release
3. Locate the executable in the `target/release` directory:
   ```bash
   cd target/release

## Option 2: Download Precompiled Binary
1. Navigate to the [Releases](https://github.com/jdluques/todo-cli/releases) page in the repository.
2. Download the latest stable binary for your platform (e.g., Linux, macOS, Windows).

Move the executable to a directory in your PATH for easier access
```bash
mv todo /usr/local/bin/
```

Verify the installation by running
```bash
todo --version
```
You should see the version number.

To further confirm run:
```bash
todo --help
```
