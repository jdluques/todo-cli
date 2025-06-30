mod models;
mod table;
mod utils;
mod commands;
mod config;
mod cli;

use clap::Parser;

fn main() {
    let args = cli::CliArgs::parse();

    let mut config = config::Config::load();

    if let Some(config_params) = args.config {
        let (key, value) = if let Some((key, value)) = config_params.split_once('=') {
            (key.trim(), value.trim())
        } else if let Some((key, value)) = config_params.split_once(':') {
            (key.trim(), value.trim())
        } else {
            eprintln!("Invalid format for --config. Use `key=value` or `key:value`.");
            return;
        };

        match key {
            "tasks_file_path" => config.tasks_file_path = value.trim_matches(|c| c == '"' || c == '\'').to_string(),
            _ => {
                eprintln!("Unknown configuration key: {}", key);
                return;
            }
        }

        config.save();
        println!("Configuration updated successfully!");
        return;
    }

    let tasks_file_path = &config.tasks_file_path;
    
    utils::terminal::init_screen();

    let result = std::panic::catch_unwind(|| {
        let mut tasks = utils::storage::load_tasks(tasks_file_path);

        loop {
            table::render_table(&tasks);

            let options = vec!["Add Task", "Delete Task", "Edit Task", "Exit"];
            match utils::prompt::select_option("Choose an action:", &options) {
                Some("Add Task") => commands::add::add_task(&mut tasks),
                Some("Delete Task") => commands::delete::delete_task(&mut tasks),
                Some("Edit Task") => commands::edit::edit_task(&mut tasks),
                Some("Exit") | None => break,
                _ => (),
            }

            utils::storage::save_tasks(tasks_file_path, &tasks);
        }
    });

    utils::terminal::quit_screen();

    if result.is_err() {
        eprintln!("The application encountered an error and exited.");
    }
}
