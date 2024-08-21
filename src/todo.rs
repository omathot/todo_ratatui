use std::{fs::{File, OpenOptions}, io::{Read, Write}};
use ratatui::widgets::{List, ListItem, Block, Borders};
use ratatui::text::{Line, Span};
use ratatui::style::{Style, Color};


use chrono::{DateTime, Local, Utc};
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Debug ,Serialize, Deserialize)]
pub struct TodoItem {
	title: String,
	body: Option<String>,
	#[serde(with = "chrono::serde::ts_seconds")]
	creation_date: DateTime<Utc>,
	#[serde(with = "chrono::serde::ts_seconds")]
	last_edit_date: DateTime<Utc>,
	completed: bool,
}

impl TodoItem {
	pub fn new(title: String, body: Option<String>) -> Self {
		let now: DateTime<Utc> = Utc::now();
		TodoItem {
			title,
			body,
			creation_date: now,
			last_edit_date: now,
			completed: false,
		}
	}

	pub fn complete(&mut self) {
		if !self.completed {
			self.completed = true;
		} else {
			self.completed = false;
		}
	}

	// getters so as to not make items public
	pub fn title(&self) -> &str {
		&self.title
	}
	pub fn body(&self) -> Option<&str> {
		self.body.as_deref()
	}
	pub fn creation_date(&self) -> DateTime<Local> {
		self.creation_date.with_timezone(&chrono::Local)
	}
	pub fn last_edit_date(&self) -> DateTime<Local> {
		self.last_edit_date.with_timezone(&chrono::Local)
	}
	pub fn completed(&self) -> bool {
		self.completed
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoList {
	todos: Vec<TodoItem>
}

impl TodoList {
	pub fn new() -> Self {
		TodoList {
			todos: Vec::new(),
		}
	}

	pub fn add_todo(&mut self, title: String, body: Option<String>) {
		let new_todo = TodoItem::new(title, body);
		self.todos.insert(0, new_todo);
	}

	pub fn remove_todo(&mut self, idx: usize) -> Option<TodoItem> {
		if idx < self.todos.len() {
			Some(self.todos.remove(idx))
		}
		else {
			None
		}
	}

	pub fn complete_todo(&mut self, idx: usize) -> bool {
		if let Some(todo) = self.todos.get_mut(idx) {
			todo.complete();
			true
		}
		else {
			false
		}
	}

	pub fn get_todo(&self, idx: usize) -> Option<&TodoItem> {
		self.todos.get(idx)
	}

	pub fn get_todos(&self) -> &[TodoItem] {
		&self.todos
	}

	pub fn len(&self) -> usize {
		self.todos.len()
	}

	pub fn is_empty(&self) -> bool {
		self.todos.is_empty()
	}

	pub fn save_to_file(&self, filename: &str) -> std::io::Result<()> {
		let json = serde_json::to_string(self)?;
		let mut file = OpenOptions::new()
			.write(true)
			.create(true)
			.truncate(true)
			.open(filename)?;
		file.write_all(json.as_bytes())?;
		Ok(())
	}

	pub fn load_from_file(filename: &str) -> std::io::Result<Self> {
		let mut file = File::open(filename)?;
		let mut contents = String::new();
		file.read_to_string(&mut contents)?;
		let todolist = serde_json::from_str(&contents)?;
		Ok(todolist)
	}

	pub fn sort_by_date(&mut self) {
		self.todos.sort_by(|a, b|b.creation_date().cmp(&a.creation_date()))
	}

	pub fn filter_completed (&self) -> Vec<&TodoItem> {
		self.todos.iter().filter(|&todo|todo.completed()).collect()
	}

	pub fn filter_uncompleted(&self) -> Vec<&TodoItem> {
		self.todos.iter().filter(|&todo|!todo.completed()).collect()
	}

	pub fn update_todo(&mut self, idx: usize, title: Option<String>, body: Option<String>) -> bool {
        if let Some(todo) = self.todos.get_mut(idx) {
			if let Some(new_title) = title {
				todo.title = new_title;
			}
			if let Some(new_body) = body {
				todo.body = Some(new_body);
			}
			todo.last_edit_date = Utc::now();
			true
		} else {
			false
		}
	}

	pub fn create_list_widget(&self) -> List {
		let items: Vec<ListItem> = self.todos
			.iter()
			.enumerate()
			.map(|(i, todo)| {
				let status = if todo.completed() { "✓" } else { " " };
				let content = Line::from(vec![
					Span::styled(
                        format!("{}: [{}] ", i, status),
                        Style::default().fg(Color::Yellow)
                    ),
					Span::raw(todo.title()),
					Span::styled(
                        format!(" ({})", todo.creation_date().format("%d-%m-%Y %H:%M")),
                        Style::default().fg(Color::Gray)
                    ),
				]);
				ListItem::new(content)
			})
			.collect();
		List::new(items)
			.block(Block::default().borders(Borders::ALL).title("Todo Items"))
			.highlight_style(Style::default().bg(Color::DarkGray))
			.highlight_symbol("> ")
	}
}