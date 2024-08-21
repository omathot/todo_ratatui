use std::io;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crate::app::{App, InputMode};

pub fn handle_events(app: &mut App) -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
			match app.input_mode {
				InputMode::Visual => match key.code {
					KeyCode::Char('q') => return Ok(true),
					KeyCode::Char('n') => app.input_mode = InputMode::Input,
					KeyCode::Up | KeyCode::Down => app.input_mode = InputMode::Select,
					_ => {}
				},
				InputMode::Select => match key.code {
					KeyCode::Up => app.previous_todo(),
					KeyCode::Down => app.next_todo(),
					KeyCode::Esc | KeyCode::Char('q') => app.input_mode = InputMode::Visual,
					KeyCode::Char('d') => {
						app.todo_list.remove_todo(app.todo_list_index);
					}
					KeyCode::Char('c') => {
						app.todo_list.complete_todo(app.todo_list_index);
					}
					_ => {}
				}
				InputMode::Input if key.kind == KeyEventKind::Press => match key.code {
					KeyCode::Enter => {
						if !app.input.is_empty() {
							app.submit_new_todo();
							app.input.clear();
							app.reset_cursor();
							app.input_mode = InputMode::Visual;
						}
					} 
					KeyCode::Char(to_insert) => {
						app.enter_char(to_insert);
					}
					KeyCode::Backspace => app.delete_char(),
					KeyCode::Left => app.move_cursor_left(),
					KeyCode::Right => app.move_cursor_right(),
					KeyCode::Esc => {
						app.input_mode = InputMode::Visual;
						app.input.clear();
						app.reset_cursor();
					}
					_ => {},
				},
				InputMode::Input => {}
			}
        }
    }
	app.update();
    Ok(false)
}