use ratatui::{symbols, Frame};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::prelude::{Color, Line, Modifier, Span, StatefulWidget, Style};
use ratatui::style::palette::tailwind::{BLUE, SLATE};
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Borders, HighlightSpacing, List, Paragraph};
use crate::app::{App, InputMode};

const TODO_HEADER_STYLE: Style = Style::new().fg(SLATE.c100).bg(BLUE.c800);
const NORMAL_ROW_BG: Color = SLATE.c950;
const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);

pub fn ui(f: &mut Frame, app: &mut App) {
    let [main_area, input_area, message_area] = Layout::vertical([
        Constraint::Min(1),
        Constraint::Length(3),
        Constraint::Length(1),
    ])
        .margin(2)
        .areas(f.area());

    render_list(f, app, main_area);
    render_input_area(f, app, input_area);
    render_message_area(f, app, message_area);
}

fn render_list(f: &mut Frame, app: &mut App, area: Rect) {
    let block = Block::new()
        .title(Line::raw("TODO List").centered())
        .borders(Borders::TOP)
        .border_set(symbols::border::EMPTY)
        .border_style(TODO_HEADER_STYLE)
        .bg(NORMAL_ROW_BG);

    let list = List::new(vec!["item 1", "item 2"])
        .block(block)
        .highlight_style(SELECTED_STYLE)
        .highlight_symbol(">")
        .highlight_spacing(HighlightSpacing::Always);

    StatefulWidget::render(list, area, f.buffer_mut(), &mut app.task_list.state);
}

fn render_input_area(f: &mut Frame, app: &mut App, area: Rect) {
    let input = Paragraph::new(app.input.as_str())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
            InputMode::EditingExisting => Style::default().fg(Color::Cyan),
        })
        .block(Block::default().borders(Borders::ALL).title("Input"));
    f.render_widget(input, area);

    match app.input_mode {
        InputMode::Normal => {}
        InputMode::Editing | InputMode::EditingExisting => {
            f.set_cursor(area.x + app.input.len() as u16 + 1, area.y + 1)
        }
    }
}

fn render_message_area(f: &mut Frame, app: &mut App, area: Rect) {
    let (msg, style) = match app.input_mode {
        InputMode::Normal => (
            vec![
                Span::raw("Press "),
                Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to exit, "),
                Span::styled("e", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to start editing, "),
                Span::styled("m", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to modify selected, "),
                Span::styled("p", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to change priority, "),
                Span::styled("s", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to sort by priority, "),
                Span::styled("d", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to delete selected, "),
                Span::styled("↑↓", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to select, "),
                Span::styled("Space", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to toggle status."),
            ],
            Style::default().add_modifier(Modifier::BOLD),
        ),
        InputMode::Editing => (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to stop editing, "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to add new item"),
            ],
            Style::default(),
        ),
        InputMode::EditingExisting => (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to cancel, "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to save changes"),
            ],
            Style::default(),
        ),
    };
    let help_message = Paragraph::new(Line::from(msg)).style(style);
    f.render_widget(help_message, area);
}

