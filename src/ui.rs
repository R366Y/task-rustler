use crate::app::{App, InputField, InputMode};
use crate::task::{Priority, Task};
use ratatui::layout::{Constraint, Flex, Layout, Position, Rect};
use ratatui::prelude::{Color, Direction, Line, Modifier, Span, StatefulWidget, Style};
use ratatui::style::palette::tailwind::{BLUE, SLATE};
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Borders, Clear, HighlightSpacing, List, ListItem, Paragraph};
use ratatui::{symbols, Frame};
use std::vec;
const TODO_HEADER_STYLE: Style = Style::new().fg(SLATE.c100).bg(BLUE.c800);
const NORMAL_ROW_BG: Color = SLATE.c950;
const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);
const TEXT_FG_COLOR: Color = SLATE.c200;
const COMPLETED_TEXT_FG_COLOR: Color = SLATE.c500;

pub fn ui(f: &mut Frame, app: &mut App) {
    match app.input_mode {
        InputMode::View => {
            let [main_area, message_area] =
                Layout::vertical([Constraint::Min(1), Constraint::Length(1)])
                    .margin(1)
                    .areas(f.area());
            render_list(f, app, main_area);
            render_message_area(f, app, message_area);
        }
        InputMode::Adding | InputMode::EditingExisting => {
            let [main_area, input_title_area, input_description_area, input_date_area, message_area] =
                Layout::vertical([
                    Constraint::Min(1),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(1),
                ])
                .margin(1)
                .areas(f.area());

            let input_area = match app.input_field {
                InputField::Title => input_title_area,
                InputField::Description => input_description_area,
                InputField::Date => input_date_area,
            };
            let x = input_area.x
                + match app.input_field {
                    InputField::Title => app.input_title.len() as u16,
                    InputField::Description => app.input_description.len() as u16,
                    InputField::Date => app.input_date.len() as u16,
                }
                + 1;
            let y = input_area.y + 1;
            f.set_cursor_position(Position::new(x, y));

            render_list(f, app, main_area);
            render_input_title_area(f, app, input_title_area);
            render_input_description_area(f, app, input_description_area);
            render_input_date_area(f, app, input_date_area);
            render_message_area(f, app, message_area);
        }
    }
    if app.show_popup {
        let block = Block::bordered().title("Help");
        let area = render_popup(f.area(), 40, 80);
        f.render_widget(Clear, area); //this clears out the background
        f.render_widget(block, area);

        let popup_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Percentage(80)])
            .split(area);
        let t1 = Paragraph::new(vec![
            Line::raw("'q' to quit"),
            Line::raw("'a' to add a task"),
            Line::raw("'m' to modify the selected task"),
            Line::raw("'d' to delete the selected task"),
            Line::raw("'p' to change the priority"),
            Line::raw("'s' to sort by priority"),
            Line::raw("'↑↓' to select task"),
            Line::raw("'Space' to toggle status"),
        ]);
        f.render_widget(t1, popup_chunks[0]);
    }
}

fn render_list(f: &mut Frame, app: &mut App, area: Rect) {
    let block = Block::new()
        .title(Line::raw("Task Rustler").centered())
        .borders(Borders::TOP)
        .border_set(symbols::border::EMPTY)
        .border_style(TODO_HEADER_STYLE)
        .bg(NORMAL_ROW_BG);

    let items: Vec<ListItem> = app
        .task_list
        .items
        .iter()
        .enumerate()
        .map(|(_, item)| ListItem::from(item))
        .collect();

    let list = List::new(items)
        .block(block)
        .highlight_style(SELECTED_STYLE)
        .highlight_symbol(">")
        .highlight_spacing(HighlightSpacing::Always);

    StatefulWidget::render(list, area, f.buffer_mut(), &mut app.task_list.state);
}

fn render_input_title_area(f: &mut Frame, app: &mut App, area: Rect) {
    let input = create_input_paragraph(app, app.input_title.as_str(), "Title");
    f.render_widget(input, area);
}

fn render_input_description_area(f: &mut Frame, app: &mut App, area: Rect) {
    let input = create_input_paragraph(app, app.input_description.as_str(), "Description");
    f.render_widget(input, area);
}

fn render_input_date_area(f: &mut Frame, app: &mut App, area: Rect) {
    let input = create_input_paragraph(app, app.input_date.as_str(), "Date (dd-mm-yyyy)");
    f.render_widget(input, area);
}

fn render_message_area(f: &mut Frame, app: &mut App, area: Rect) {
    let (msg, style) = match app.input_mode {
        InputMode::View => (
            vec![
                Span::styled("Display tasks", Style::default().bg(Color::White).fg(Color::Black)),
                Span::raw("  Press "),
                Span::styled("h", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" for help "),
            ],
            Style::default().add_modifier(Modifier::BOLD),
        ),
        InputMode::Adding => (
            if app.error.is_none() {
                vec![
                    Span::styled("Add task", Style::default().bg(Color::White).fg(Color::Black)),
                    Span::raw("  Press "),
                    Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to stop editing, "),
                    Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to add new item"),
                ]
            } else {
                vec![Span::styled(
                    app.error.as_ref().unwrap(),
                    Style::default().red(),
                )]
            },
            Style::default(),
        ),
        InputMode::EditingExisting => (
            if app.error.is_none() {
                vec![
                    Span::styled("Edit task", Style::default().bg(Color::White).fg(Color::Black)),
                    Span::raw("  Press "),
                    Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to cancel, "),
                    Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to save changes"),
                ]
            } else {
                vec![Span::styled(
                    app.error.as_ref().unwrap(),
                    Style::default().red(),
                )]
            },
            Style::default(),
        ),
    };
    let help_message = Paragraph::new(Line::from(msg)).style(style);
    f.render_widget(help_message, area);
}

fn render_popup(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}

impl From<&Task> for ListItem<'_> {
    fn from(value: &Task) -> Self {
        let todo_line = vec![
            Span::styled(" ☐", Style::default().fg(TEXT_FG_COLOR)),
            Span::styled(
                format!(" ({})", value.priority),
                Style::default().fg(priority_to_color(&value.priority)),
            ),
            Span::styled(
                format!(
                    "{:>14}",
                    value
                        .date
                        .clone()
                        .try_into()
                        .unwrap_or(format!("{}", " ".repeat(10)))
                ),
                Style::default(),
            ),
            Span::styled(
                format!("    {} - {}", value.title, value.description),
                Style::default().fg(TEXT_FG_COLOR),
            ),
        ];
        let done_line = vec![
            Span::styled(" ✓", Style::default().fg(COMPLETED_TEXT_FG_COLOR)),
            Span::styled(
                format!(" ({})", value.priority),
                Style::default().fg(priority_to_color(&value.priority)),
            ),
            Span::styled(
                format!(
                    "{:>14}",
                    value
                        .date
                        .clone()
                        .try_into()
                        .unwrap_or(format!("{}", " ".repeat(10)))
                ),
                Style::default(),
            ),
            Span::styled(
                format!("    {} - {}", value.title, value.description),
                Style::default().fg(COMPLETED_TEXT_FG_COLOR),
            ),
        ];
        let line: Line = match value.completed {
            false => todo_line.into(),
            true => done_line.into(),
        };
        ListItem::new(line)
    }
}

fn priority_to_color(priority: &Priority) -> Color {
    match priority {
        Priority::Low => Color::Green,
        Priority::Medium => Color::Yellow,
        Priority::High => Color::Red,
    }
}

fn create_input_paragraph<'a>(app: &'a App, text: &'a str, title: &'a str) -> Paragraph<'a> {
    Paragraph::new(text)
        .style(match app.input_mode {
            InputMode::View => Style::default(),
            InputMode::Adding => Style::default().fg(Color::Green),
            InputMode::EditingExisting => Style::default().fg(Color::Yellow),
        })
        .block(Block::default().borders(Borders::BOTTOM).title(title))
}
