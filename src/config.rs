use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::Path
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub tasks_file_path: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            tasks_file_path: "tasks.json".to_string(),
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let config: Config = confy::load("todo_cli", None).unwrap_or_else(|_| Config {
            tasks_file_path: "tasks.json".to_string(),
        });

        let tasks_path = Path::new(&config.tasks_file_path);
        if !tasks_path.exists() {
            println!("Creating tasks file at: {}", config.tasks_file_path);
            fs::write(&config.tasks_file_path, "[]").expect("Unable to create tasks file");
        }

        config
    }
}
