mod models;
mod table;
mod utils;
mod commands;
mod config;
mod cli;

use clap::Parser;

fn main() {
    let args = cli::args::CliArgs::parse();

    let mut config = config::Config::load();

    if let Some(config_params) = args.config {
        cli::flags_actions::handle_config::handle_config(config_params, &mut config);
    }
    else {
        let tasks_file_path = &config.tasks_file_path;
        
        if args.visual {
            cli::flags_actions::handle_visual::handle_visual(tasks_file_path);
        }
        else {
            execute_program(tasks_file_path);
        }
    }

    
}

fn execute_program(tasks_file_path: &String) {
    utils::terminal::init_screen();

    let result = std::panic::catch_unwind(|| {
        let mut tasks = utils::storage::load_tasks(tasks_file_path);

        loop {
            table::render_table(&tasks, table::RenderType::Interactive);

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
