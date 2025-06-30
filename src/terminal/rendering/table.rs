use comfy_table::{Cell, Row, Table};

use crate::{
    task_model,
    terminal::rendering::screen
};

#[derive(Debug, Clone)]
pub enum RenderType {
    Interactive,
    Visual,
}

pub fn render_table(tasks: &mut [task_model::Task], render_type: RenderType) {
    match render_type {
        RenderType::Interactive => {
            screen::clear_screen();
            screen::move_cursor_to_top();
        },
        RenderType::Visual => (),
    }
    
    let mut table = Table::new();
    table.set_header(vec!["Not Started", "In Progress", "Completed"]);

    let (mut not_started, mut in_progress, mut completed) = (vec![], vec![], vec![]);

    tasks.sort_by(|a, b| b.priority.cmp(&a.priority));
    for task in tasks {
        let task_entry =  format!("{} {}", priority_emoji(&task.priority), task.name.clone());
        match task.status {
            task_model::TaskStatus::NotStarted => not_started.push(task_entry),
            task_model::TaskStatus::InProgress => in_progress.push(task_entry),
            task_model::TaskStatus::Completed => completed.push(task_entry),
        }
    }

    let row = Row::from(vec![
        Cell::new(&not_started.join("\n")),
        Cell::new(&in_progress.join("\n")),
        Cell::new(&completed.join("\n")),
    ]);

    table.add_row(row);
    
    match render_type {
        RenderType::Interactive => {
            let table_content = format!("{}\n", table.to_string());
            screen::write_at_position(0, 0, &table_content);
        },
        RenderType::Visual => println!("{table}"),
    }
}

fn priority_emoji(priority: &task_model::TaskPriority) -> &str {
    match priority {
        task_model::TaskPriority::Low => "\u{1F7E2}",
        task_model::TaskPriority::Medium => "\u{26A0}",
        task_model::TaskPriority::High => "\u{1F6A8}",
        task_model::TaskPriority::Complete => "\u{2705}",
    }
}