use crate::{
    task_model,
    utils::prompt
};

pub fn add_task(tasks: &mut Vec<task_model::Task>) {
    let name = prompt::text_input("Enter task name:").unwrap_or_default();

    let status_selected = prompt::select_option("Select task status:", &["Not Started", "In Progress", "Completed"])
        .unwrap_or("Not Started");
    let status = match status_selected {
        "Not Started" => task_model::TaskStatus::NotStarted,
        "In Progress" => task_model::TaskStatus::InProgress,
        "Completed" => task_model::TaskStatus::Completed,
        _ => task_model::TaskStatus::NotStarted,
    };

    let mut priority_selected = "Complete";
    if status != task_model::TaskStatus::Completed {
        priority_selected = prompt::select_option("Select task priority:", &["Low", "Medium", "High"])
        .unwrap_or("Not Started");
    }
    let priority = match priority_selected {
        "Low" => task_model::TaskPriority::Low,
        "Medium" => task_model::TaskPriority::Medium,
        "High" => task_model::TaskPriority::High,
        "Complete" => task_model::TaskPriority::Complete,
        _ => task_model::TaskPriority::Low,
    };

    tasks.push(task_model::Task { name, status, priority });
}
