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

	// pub fn move_cursor_down(&mut self) {
	// 	if !self.todo_list.is_empty() {
	// 		let new_index = self.todo_list_index.saturating_add(1);
	// 		self.todo_list_index = new_index.min(self.todo_list.len() - 1);
	// 	}
	// }

	// pub fn move_cursor_up(&mut self) {
	// 	if !self.todo_list.is_empty() {
	// 		self.todo_list_index = self.todo_list_index.saturating_sub(1);
	// 	}
	// }

	pub fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
		if index < MAX_CHARS {
			self.input.insert(index, new_char);
			self.move_cursor_right();
		}
    }
	
	pub fn byte_index(&mut self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.cursor_index)
            .unwrap_or(self.input.len())
    }

	pub fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
		new_cursor_pos.clamp(0, self.input.chars().count())
	}

	pub fn clamp_todo_list_index(&self, idx: usize) -> usize {
		idx.min(self.todo_list.len().saturating_sub(1))
	}

	pub fn delete_char(&mut self) {
		let is_not_cursor_leftmost = self.cursor_index != 0;
		if is_not_cursor_leftmost {
			let current_index = self.cursor_index;
			let from_left_to_current_index = current_index - 1;

			let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
			let after_char_to_delete = self.input.chars().skip(current_index);

			self.input = before_char_to_delete.chain(after_char_to_delete).collect();
			self.move_cursor_left();
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
