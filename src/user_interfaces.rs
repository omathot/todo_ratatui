use itertools::izip;
use ratatui::{
    prelude::*,
    widgets::{Block, Clear, Borders, Paragraph},
};
use crate::app::{App, InputMode};
use indoc::indoc;

#[allow(clippy::many_single_char_names)]
fn logo() -> Vec<Line<'static>> {
	let todo_app = indoc! {r#"
	████████╗ ██████╗ ██████╗  ██████╗      █████╗ ██████╗ ██████╗ 
	╚══██╔══╝██╔═══██╗██╔══██╗██╔═══██╗    ██╔══██╗██╔══██╗██╔══██╗
	   ██║   ██║   ██║██║  ██║██║   ██║    ███████║██████╔╝██████╔╝
	   ██║   ██║   ██║██║  ██║██║   ██║    ██╔══██║██╔═══╝ ██╔═══╝ 
	   ██║   ╚██████╔╝██████╔╝╚██████╔╝    ██║  ██║██║     ██║     
	   ╚═╝    ╚═════╝ ╚═════╝  ╚═════╝     ╚═╝  ╚═╝╚═╝     ╚═╝     
	"#};
	izip!(todo_app.lines())
		.map(|line| {
			Line::from(vec![
				Span::styled(
					format!("{:5}", line),
					Style::default().fg(Color::Blue)
				)
			])
		})
		.collect()
}

#[allow(clippy::many_single_char_names)]
fn display_title() -> Vec<Line<'static>> {
	let display_title = indoc! {r#"
    ████████╗ ██████╗ ██████╗  ██████╗     ██╗     ██╗███████╗████████╗
    ╚══██╔══╝██╔═══██╗██╔══██╗██╔═══██╗    ██║     ██║██╔════╝╚══██╔══╝
       ██║   ██║   ██║██║  ██║██║   ██║    ██║     ██║███████╗   ██║   
       ██║   ██║   ██║██║  ██║██║   ██║    ██║     ██║╚════██║   ██║   
       ██║   ╚██████╔╝██████╔╝╚██████╔╝    ███████╗██║███████║   ██║   
       ╚═╝    ╚═════╝ ╚═════╝  ╚═════╝     ╚══════╝╚═╝╚══════╝   ╚═╝   
    "#};
    izip!(display_title.lines())
        .map(|line| {
            Line::from(vec![
                Span::styled(
                    format!("{:5}", line),
                    Style::default().fg(Color::Cyan)
                )
            ])
        })
        .collect()
}

pub fn startup_ui(frame: &mut Frame) {
	let chunks = Layout::default()
		.direction(Direction::Vertical)
		.constraints([
			Constraint::Percentage(40),
			Constraint::Length(8),
			Constraint::Percentage(40),
		])
		.split(frame.area());
	let logo_paragraph = Paragraph::new(logo())
			.alignment(Alignment::Center)
			.block(Block::default().borders(Borders::NONE));
    frame.render_widget(logo_paragraph, chunks[1]);
}

pub fn main_ui(frame: &mut Frame, app: &mut App) {
	let area = frame.area();
	let chunks = Layout::default()
		.direction(Direction::Vertical)
		.constraints([
			Constraint::Length(9),
			Constraint::Length(5),
			Constraint::Length(3),
			Constraint::Min(1),
		])
		.split(frame.area());
	let title_paragraph = Paragraph::new(display_title())
			.alignment(Alignment::Center)
			.block(Block::default().borders(Borders::ALL));

	let (input_help_text, style) = match app.input_mode {
		InputMode::Visual => (
			vec![
				Line::from(vec![
					Span::styled("• ", Style::default().fg(Color::Yellow)),
					Span::raw("Press "),
					Span::styled("'N'", Style::default().fg(Color::Green)),
					Span::raw(" to add a new todo"),
				]),
				Line::from(vec![
					Span::styled("• ", Style::default().fg(Color::Yellow)),
					Span::styled("Up ", Style::default().fg(Color::LightCyan)),
					Span::raw("or "),
					Span::styled("Down ", Style::default().fg(Color::LightCyan)),
					Span::raw("arrows to select Todo"),
				]),
				Line::from(vec![
					Span::styled("• ", Style::default().fg(Color::Yellow)),
					Span::raw("Press "),
					Span::styled("'Q'", Style::default().fg(Color::Red)),
					Span::raw(" to quit"),
				]),
				],
				Style::default().add_modifier(Modifier::RAPID_BLINK),
			),
		InputMode::Input => (
			vec![
				Line::from(vec![
					Span::styled("• ", Style::default().fg(Color::Yellow)),
					Span::raw("Press "),
					Span::styled("'Esc'", Style::default().fg(Color::Green)),
					Span::raw(" to leave Input mode"),
            	]),
				Line::from(vec![
					Span::styled("• ", Style::default().fg(Color::Yellow)),
					Span::raw("Press "),
					Span::styled("Enter ", Style::default().fg(Color::Yellow)),
					Span::raw("to save new todo"),
				]),
				],
				Style::default(),
			),
		InputMode::Select | InputMode::Popup | InputMode::PopupInput => (
			vec![
				Line::from(vec![
					Span::styled("• ", Style::default().fg(Color::Yellow)),
					Span::raw("Press "),
					Span::styled("'Esc'", Style::default().fg(Color::Red)),
					Span::raw("or "),
					Span::styled("'Q'", Style::default().fg(Color::Red)),
					Span::raw(" to leave Select mode"),
					Span::raw(" ".repeat(6)), 
					Span::styled("• ", Style::default().fg(Color::Yellow)),
					Span::raw("Press "),
					Span::styled("'D'", Style::default().fg(Color::Red)),
					Span::raw(" to delete a todo"),
				]),
				Line::from(vec![
					Span::styled("• ", Style::default().fg(Color::Yellow)),
					Span::raw("Press "),
					Span::styled("Enter ", Style::default().fg(Color::Green)),
					Span::raw("to select hovered Todo"),
				]),
				Line::from(vec![
					Span::styled("• ", Style::default().fg(Color::Yellow)),
					Span::raw("Press "),
					Span::styled("'C'", Style::default().fg(Color::Blue)),
					Span::raw(" to complete a todo"),
				])
			],
			Style::default(),
		)
	};
	let input_help = Paragraph::new(input_help_text)
			.block(Block::default().borders(Borders::ALL).title("Input Help"))
			.alignment(Alignment::Left)
			.wrap(ratatui::widgets::Wrap {trim: true});

	let user_input = Paragraph::new(app.input.as_str())
		.style(match app.input_mode {
			InputMode::Visual => Style::default(),
			InputMode::Input => Style::default().fg(Color::Yellow),
			InputMode::Select => Style::default(),
			InputMode::Popup | InputMode::PopupInput => Style::default(),
		})
		.block(Block::bordered().title("Input"))
		.add_modifier(Modifier::RAPID_BLINK);
	match app.input_mode {
		InputMode::Visual => {}
		InputMode::Input => {
			#[allow(clippy::cast_possible_truncation)]
			frame.set_cursor_position(Position {
				x: chunks[2].x + app.cursor_index as u16 + 1,
				y: chunks[2].y + 1,
			});
		}
		InputMode::Select | InputMode::Popup | InputMode::PopupInput => {}
	}
	let todo_list = app.todo_list.create_list_widget();

    frame.render_widget(title_paragraph, chunks[0]);
	frame.render_widget(input_help, chunks[1]);
	frame.render_widget(user_input, chunks[2]);
	if app.input_mode == InputMode::Select {
		frame.render_stateful_widget(todo_list, chunks[3], &mut app.todo_list_state);
	} else {
		frame.render_widget(todo_list, chunks[3]);
	}

	if app.show_todo_popup {
		match app.input_mode {
			InputMode::Popup => {
				let mut popup_text = String::new();
				if app.todo_list.get_todo_body(app.todo_list_index) == None {
					popup_text = String::from("Press i to start writing Todo's details");
				} else {
					popup_text = String::from(app.todo_list.get_todo_body(app.todo_list_index).unwrap_or_else(||"Empty body".to_string()));
				}
				let popup = Paragraph::new(popup_text)
				.style(Style::default())
				.block(Block::bordered().title("Todo Body"));
				let area = centered_rect(60, 20, area);
				frame.render_widget(Clear, area);
				frame.render_widget(popup, area);
			},
			InputMode::PopupInput => {
				let popup = Paragraph::new(app.popup_input.as_str())
				.style(Style::default().fg(Color::Yellow))
				.block(Block::bordered().title("Edit"))
				.add_modifier(Modifier::RAPID_BLINK);
				let area = centered_rect(60, 20, area);
				frame.render_widget(Clear, area);
				frame.render_widget(popup, area);
				frame.set_cursor_position(Position {
					x: area.x + app.cursor_index as u16 + 1,
					y: area.y + 1,
				});
			},
			_ => {}
		}
	}
}

pub fn leave() {}


fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
	let popup_layout = Layout::vertical([
		Constraint::Percentage((100 - percent_y) / 2),
		Constraint::Percentage(percent_y),
		Constraint::Percentage((100 - percent_y) / 2),
	])
	.split(r);

	Layout::horizontal([
		Constraint::Percentage((100 - percent_x) / 2),
		Constraint::Percentage(percent_x),
		Constraint::Percentage((100 - percent_x) / 2),
	])
	.split(popup_layout[1])[1]
}