use crate::{
    task_model,
    utils::prompt
};

pub fn edit_task(tasks: &mut Vec<task_model::Task>) {
    if tasks.is_empty() {
        return;
    }

    let task_names: Vec<&str> = tasks.iter().map(|task| task.name.as_str()).collect();

    match prompt::select_option("Choose a task to edit", &task_names) {
        Some(selected_task) => {
            let selected_task = selected_task.to_string();
            if let Some(task) = tasks.iter_mut().find(|task| task.name == selected_task) {
                let mut options = vec!["Name", "Status"];
                if task.status != task_model::TaskStatus::Completed {
                    options.push("Priority");
                }

                match prompt::select_option("Choose an attribute to edit:", &options) {
                    Some("Name") => edit_name(task),
                    Some("Status") => edit_status(task),
                    Some("Priority") => edit_priority(task),
                    _ => (),
                }
            }
        },
        None => (),
    }
}

fn edit_name(task: &mut task_model::Task) {
    let new_name = prompt::text_input("New name:").unwrap_or(task.name.clone());
    task.name = new_name;
}

fn edit_status(task: &mut task_model::Task) {
    let selected_status = prompt::select_option("Select new task status:", &["Not Started", "In Progress", "Completed"])
                           .unwrap_or("Not Started");

    let task_status = match selected_status {
        "Not Started" => task_model::TaskStatus::NotStarted,
        "In Progress" => task_model::TaskStatus::InProgress,
        "Completed" => task_model::TaskStatus::Completed,
        _ => task_model::TaskStatus::NotStarted,
    };

    if task.status == task_model::TaskStatus::Completed && selected_status != "Completed" {
        edit_priority(task);
    }

    task.status = task_status;
}

fn edit_priority(task: &mut task_model::Task) {
    let selected_priority = prompt::select_option("Select new task priority:", &["Low", "Medium", "High"])
                                     .unwrap_or("Low");
    
    let task_priority = match selected_priority {
        "Low" => task_model::TaskPriority::Low,
        "Medium" => task_model::TaskPriority::Medium,
        "High" => task_model::TaskPriority::High,
        _ => task_model::TaskPriority::Low,
    };

    task.priority = task_priority;
}