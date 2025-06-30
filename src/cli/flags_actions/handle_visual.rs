use crate::{
    table,
    utils,
};

pub fn handle_visual(tasks_file_path: &String) {
    let mut tasks = utils::storage::load_tasks(tasks_file_path);
    table::render_table(&mut tasks, table::RenderType::Visual);
}
