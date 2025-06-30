use comfy_table::{Cell, Row, Table};

use crate::{
    models,
    utils::terminal
};

pub fn render_table(tasks: &[models::Task]) {
    terminal::clear_screen();
    terminal::move_cursor_to_top();

    let mut table = Table::new();
    table.set_header(vec!["Not Started", "In Progress", "Completed"]);

    let (mut not_started, mut in_progress, mut completed) = (vec![], vec![], vec![]);

    for task in tasks {
        match task.status {
            models::TaskStatus::NotStarted => not_started.push(task.name.clone()),
            models::TaskStatus::InProgress => in_progress.push(task.name.clone()),
            models::TaskStatus::Completed => completed.push(task.name.clone()),
        }
    }

    let row = Row::from(vec![
        Cell::new(&not_started.join("\n")),
        Cell::new(&in_progress.join("\n")),
        Cell::new(&completed.join("\n")),
    ]);

    table.add_row(row);
    
    let mut table_content = table.to_string();
    table_content.push_str("\n");
    terminal::write_at_position(0, 0, &table_content);
}
