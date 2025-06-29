mod models;
mod table;

fn main() {
    let tasks = vec![
        models::Task{ name: String::from("Task 1"), status: models::TaskStatus::NotStarted },
        models::Task{ name: String::from("Task 2"), status: models::TaskStatus::InProgress },
        models::Task{ name: String::from("Task 3"), status: models::TaskStatus::Completed },
        models::Task{ name: String::from("Task 4"), status: models::TaskStatus::Completed },
        models::Task{ name: String::from("Task 5"), status: models::TaskStatus::NotStarted },
        models::Task{ name: String::from("Task 6"), status: models::TaskStatus::NotStarted },
    ];

    table::render_table(&tasks);
}
