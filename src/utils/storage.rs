use std::{
    fs::{
        File,
        read_to_string
    },
    io::Write
};
use serde_json::to_string_pretty;

use crate::task_model;

pub fn load_tasks(json_file_path: &str) -> Vec<task_model::Task> {
    if let Ok(data) = read_to_string(json_file_path) {
        serde_json::from_str(&data).unwrap_or_default()
    } else {
        vec![]
    }
}

pub fn save_tasks(json_file_path: &str, tasks: &[task_model::Task]) {
    let data = to_string_pretty(tasks).unwrap();
    let mut file = File::create(json_file_path).unwrap();
    file.write_all(data.as_bytes()).unwrap();
}
