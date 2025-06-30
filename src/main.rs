mod commands;
mod config;
mod storage;
mod task_model;
mod terminal;
mod utils;

use clap::Parser;

fn main() {
    let args = terminal::cli::args::CliArgs::parse();

    let mut config = config::Config::load();

    if let Some(config_params) = args.config {
        terminal::cli::flags_handlers::handle_config::handle_config(config_params, &mut config);
    }
    else {
        let tasks_file_path = &config.tasks_file_path;
        
        if args.visual {
            terminal::cli::flags_handlers::handle_visual::handle_visual(tasks_file_path);
        }
        else {
            execute_interactive_mode(tasks_file_path);
        }
    }    
}

fn execute_interactive_mode(tasks_file_path: &String) {
    terminal::rendering::screen::init_screen();

    let result = std::panic::catch_unwind(|| {
        let mut tasks = storage::load_tasks(tasks_file_path);

        loop {
            terminal::rendering::table::render_table(&mut tasks, terminal::rendering::table::RenderType::Interactive);

            let options = vec!["Add Task", "Delete Task", "Edit Task", "Exit"];
            match utils::prompts::select_option("Choose an action:", &options) {
                Some("Add Task") => commands::add::add_task(&mut tasks),
                Some("Delete Task") => commands::delete::delete_task(&mut tasks),
                Some("Edit Task") => commands::edit::edit_task(&mut tasks),
                Some("Exit") | None => break,
                _ => (),
            }

            storage::save_tasks(tasks_file_path, &tasks);
        }
    });

    terminal::rendering::screen::quit_screen();

    if result.is_err() {
        eprintln!("The application encountered an error and exited.");
    }
}
