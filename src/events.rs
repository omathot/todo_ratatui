use std::io;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crate::app::{App, InputMode};

pub fn handle_events(app: &mut App) -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
			match app.input_mode {
				InputMode::Visual => match key.code {
					KeyCode::Char('q') => {
						match app.todo_list.save_to_file(".todo_temp.json") {
							Ok(()) => {
								println!("Successfuly saved todos in temp file");
							}
							Err(e) => {
								match e.kind() {
									io::ErrorKind::PermissionDenied => println!("Permission denied, check permissions"),
									_ => println!("Error saving todos: {}", e),
								};
							}
						}
						return Ok(true);
					}
					KeyCode::Char('n') => app.input_mode = InputMode::Input,
					KeyCode::Up | KeyCode::Down => {
						if !app.todo_list.is_empty() {
							app.input_mode = InputMode::Select;
						}
					}
					_ => {}
				},
				InputMode::Select => match key.code {
					KeyCode::Up => app.previous_todo(),
					KeyCode::Down => app.next_todo(),
					KeyCode::Esc | KeyCode::Char('q') => app.input_mode = InputMode::Visual,
					KeyCode::Enter => {
						if !app.show_todo_popup {app.show_todo_popup = true; app.input_mode = InputMode::Popup}
					}
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
						if !app.input.is_empty() && !app.todo_list.contains_title(&app.input) {
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
				InputMode::Popup => match key.code {
					KeyCode::Esc => {app.input_mode = InputMode::Select; app.show_todo_popup = false}
					KeyCode::Char('i') => app.input_mode = InputMode::PopupInput,
					_ => {}
				}
				InputMode::PopupInput if key.kind == KeyEventKind::Press => match key.code {
					KeyCode::Char(to_insert) => {
						app.enter_char(to_insert);
					}
					KeyCode::Enter => {
						if !app.popup_input.is_empty() {
							app.todo_list.update_todo(app.todo_list_index, None, Some(app.popup_input.clone()));
							app.popup_input.clear();
							app.reset_cursor();
							app.input_mode = InputMode::Select;
							app.show_todo_popup = false;
						}
					}
					KeyCode::Backspace => app.delete_char(),
					KeyCode::Left => app.move_cursor_left(),
					KeyCode::Right => app.move_cursor_right(),
					KeyCode::Esc => app.input_mode = InputMode::Popup,
					_ => {}
				}
				InputMode::PopupInput => {}
			}
        }
    }
	app.update();
    Ok(false)
}