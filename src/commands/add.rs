use crate::{
    models,
    utils::prompt
};

pub fn add_task(tasks: &mut Vec<models::Task>) {
    let name = prompt::text_input("Enter task name:").unwrap_or_default();
    let status = prompt::select_option("Select task status:", &["Not Started", "In Progress", "Completed"])
        .unwrap_or("Not Started");

    let task_status = match status {
        "Not Started" => models::TaskStatus::NotStarted,
        "In Progress" => models::TaskStatus::InProgress,
        "Completed" => models::TaskStatus::Completed,
        _ => models::TaskStatus::NotStarted,
    };

    tasks.push(models::Task { name, status: task_status });
}
