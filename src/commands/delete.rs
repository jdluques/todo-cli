use crate::{
    models,
    utils::prompt
};

pub fn delete_task(tasks: &mut Vec<models::Task>) {
    if tasks.is_empty() {
        println!("No tasks to delete.");
        return;
    }

    let task_names: Vec<&str> = tasks.iter().map(|task| task.name.as_str()).collect();

    match prompt::select_option("Choose a task to delete:", &task_names) {
        Some(selected_task) => {
            let selected_task = selected_task.to_string();
            tasks.retain(|task| task.name != selected_task);
        },
        None => println!("No task selected."),
    }
}
