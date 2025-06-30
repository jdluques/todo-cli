use crate::{
    terminal::rendering::table,
    storage,
};

pub fn handle_visual(tasks_file_path: &String) {
    let mut tasks = storage::load_tasks(tasks_file_path);
    table::render_table(&mut tasks, table::RenderType::Visual);
}
