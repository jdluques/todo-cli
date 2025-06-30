use crate::config;

pub fn handle_config(config_params: String, config: &mut config::Config) {
    let (key, value) = if let Some((key, value)) = config_params.split_once('=') {
        (key.trim(), value.trim())
    } else if let Some((key, value)) = config_params.split_once(':') {
        (key.trim(), value.trim())
    } else {
        eprintln!("Invalid format for --config. Use `key=value` or `key:value`.");
        return;
    };

    match key {
        "tasks_file_path" => config.tasks_file_path = value.trim_matches(|c| c == '"' || c == '\'').to_string(),
        _ => {
            eprintln!("Unknown configuration key: {}", key);
            return;
        }
    }

    config.save();
    println!("Configuration updated successfully!");
    return;
}
