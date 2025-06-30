use crate::{
    models,
    utils::prompt
};

pub fn edit_task(tasks: &mut Vec<models::Task>) {
    if tasks.is_empty() {
        return;
    }

    let task_names: Vec<&str> = tasks.iter().map(|task| task.name.as_str()).collect();

    match prompt::select_option("Choose a task to edit", &task_names) {
        Some(selected_task) => {
            let selected_task = selected_task.to_string();
            if let Some(task) = tasks.iter_mut().find(|task| task.name == selected_task) {
                let options = vec!["Name", "Status"];
                match prompt::select_option("Choose an attribute to edit:", &options) {
                    Some("Name") => {
                        let new_name = prompt::text_input("New name:").unwrap_or(task.name.clone());
                        task.name = new_name;
                    },
                    Some("Status") => {
                        let new_status = prompt::select_option("Select new task status:", &["Not Started", "In Progress", "Completed"])
                            .unwrap_or("Not Started");

                        let task_status = match new_status {
                            "Not Started" => models::TaskStatus::NotStarted,
                            "In Progress" => models::TaskStatus::InProgress,
                            "Completed" => models::TaskStatus::Completed,
                            _ => models::TaskStatus::NotStarted,
                        };

                        task.status = task_status;
                    },
                    _ => (),
                }
            }
        },
        None => (),
    }
}
