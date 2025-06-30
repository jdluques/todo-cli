use crossterm::{
    cursor::MoveTo,
    execute,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen}
};
use std::io::{stdout, Write};

pub fn init_screen() {
    enable_raw_mode().unwrap();
    execute!(stdout(), EnterAlternateScreen).unwrap();
}

pub fn quit_screen() {
    execute!(stdout(), LeaveAlternateScreen).unwrap();
    disable_raw_mode().unwrap();
}

pub fn clear_screen() {
    execute!(stdout(), Clear(ClearType::All)).unwrap();
    stdout().flush().unwrap();
}

pub fn move_cursor_to_top() {
    execute!(stdout(), MoveTo(0, 0)).unwrap();
    stdout().flush().unwrap();
}

pub fn write_at_position(x: u16, y: u16, content: &str) {
    execute!(stdout(), MoveTo(x, y), Print(content)).unwrap();
    stdout().flush().unwrap();
}
