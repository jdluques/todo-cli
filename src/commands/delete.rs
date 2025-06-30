use crate::{
    task_model,
    utils::prompts
};

pub fn delete_task(tasks: &mut Vec<task_model::Task>) {
    if tasks.is_empty() {
        return;
    }

    let task_names: Vec<&str> = tasks.iter().map(|task| task.name.as_str()).collect();

    match prompts::select_option("Choose a task to delete:", &task_names) {
        Some(selected_task) => {
            let selected_task = selected_task.to_string();
            tasks.retain(|task| task.name != selected_task);
        },
        None => (),
    }
}
