mod models;
mod table;
mod utils;
mod commands;

fn main() {
    utils::terminal::init_screen();

    let result = std::panic::catch_unwind(|| {
        let mut tasks = vec![
            models::Task{ name: String::from("Task 1"), status: models::TaskStatus::NotStarted },
            models::Task{ name: String::from("Task 2"), status: models::TaskStatus::InProgress },
            models::Task{ name: String::from("Task 3"), status: models::TaskStatus::Completed },
            models::Task{ name: String::from("Task 4"), status: models::TaskStatus::Completed },
            models::Task{ name: String::from("Task 5"), status: models::TaskStatus::NotStarted },
            models::Task{ name: String::from("Task 6"), status: models::TaskStatus::NotStarted },
        ];

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
        }
    });

    utils::terminal::quit_screen();

    if result.is_err() {
        eprintln!("The application encountered an error and exited.");
    }
}
