use std::time::{Duration, Instant};

use ratatui::widgets::ListState;
use crate::todo::TodoList;

const MAX_CHARS: usize = 180;

#[derive(PartialEq, Debug)]
pub enum State {
	Startup,
	Display,
	Exit,
}

#[derive(PartialEq, Debug)]
pub enum InputMode {
	Visual,
	Input,
	Select,
	Popup,
	PopupInput,
}

#[derive(Debug)]
pub struct App {
	pub state: State,
	pub start_time: Instant,
	pub should_quit: bool,
	pub cursor_index: usize,
	pub input_mode: InputMode,
	pub input: String,
	pub todo_list: TodoList,
	pub todo_list_state: ListState,
	pub todo_list_index: usize,
	pub show_todo_popup: bool,
	pub popup_input: String,
}

impl App {
	pub fn new() -> Self {
		App {
			state: State::Startup,
			start_time: Instant::now(),
			should_quit: false,
			cursor_index: 0,
			input_mode: InputMode::Visual,
			input: String::new(),
			todo_list: TodoList::new(),
			todo_list_state: ListState::default(),
			todo_list_index: 0,
			show_todo_popup: false,
			popup_input: String::new(),
		}
	}

	pub fn update(&mut self) {
		if let State::Startup = self.state {
			if self.start_time.elapsed() > Duration::from_secs(2) {
				self.state = State::Display;
			}
		}
	}

	pub fn move_cursor_left(&mut self) {
		let cursor_moved_left = self.cursor_index.saturating_sub(1);
        self.cursor_index = self.clamp_cursor(cursor_moved_left);
	}

	pub fn move_cursor_right(&mut self) {
		let cursor_moved_right = self.cursor_index.saturating_add(1);
		self.cursor_index = self.clamp_cursor(cursor_moved_right);
	}

	pub fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
		if index < MAX_CHARS {
			match self.input_mode {
				InputMode::Input => {
					self.input.insert(index, new_char);
					self.move_cursor_right();
				}
				InputMode::PopupInput => {
					self.popup_input.insert(index, new_char);
					self.move_cursor_right();
				}
				_ => {}
			}
		}
    }
	
	pub fn byte_index(&mut self) -> usize {
		match self.input_mode {
			InputMode::Input => {
				self.input
					.char_indices()
					.map(|(i, _)| i)
					.nth(self.cursor_index)
					.unwrap_or(self.input.len())
			}
			InputMode::PopupInput => {
				self.popup_input
					.char_indices()
					.map(|(i, _)| i)
					.nth(self.cursor_index)
					.unwrap_or(self.popup_input.len())
			}
			_ => 0
		}
    }

	pub fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
		if self.input_mode == InputMode::Input {
			new_cursor_pos.clamp(0, self.input.chars().count())
		}
		else if self.input_mode == InputMode::PopupInput {
			new_cursor_pos.clamp(0, self.popup_input.chars().count())
		}
		else {0}
	}

	pub fn clamp_todo_list_index(&self, idx: usize) -> usize {
		idx.min(self.todo_list.len().saturating_sub(1))
	}

	pub fn delete_char(&mut self) {
		let is_not_cursor_leftmost = self.cursor_index != 0;
		if is_not_cursor_leftmost {
			let current_index = self.cursor_index;
			let from_left_to_current_index = current_index - 1;
			if self.input_mode == InputMode::Input {
				let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
				let after_char_to_delete = self.input.chars().skip(current_index);
				self.input = before_char_to_delete.chain(after_char_to_delete).collect();
				self.move_cursor_left();
			}
			else if self.input_mode == InputMode::PopupInput {
				let before_char_to_delete = self.popup_input.chars().take(from_left_to_current_index);
				let after_char_to_delete = self.popup_input.chars().skip(current_index);
				self.popup_input = before_char_to_delete.chain(after_char_to_delete).collect();
				self.move_cursor_left();
			}
			else {}
		}
	}

	pub fn reset_cursor(&mut self) {
		self.cursor_index = 0;
	}

	pub fn submit_new_todo(&mut self) {
		self.todo_list.add_todo(self.input.clone(), None);
	}

	pub fn next_todo(&mut self) {
		let i = match self.todo_list_state.selected() {
			Some(i) => {
				if i >= self.todo_list.len() - 1 {
					0
				} else {
					i + 1
				}
			}
			None => 0,
		};
		self.todo_list_state.select(Some(i));
		self.todo_list_index = i;
	}

	pub fn previous_todo(&mut self) {
		let i = match self.todo_list_state.selected() {
			Some(i) => {
				if i == 0 {
					self.todo_list.len() - 1
				} else {
					i - 1
				}
			}
			None => 0,
		};
		self.todo_list_state.select(Some(i));
		self.todo_list_index = i;
	}
}
