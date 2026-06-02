use std::fmt::Alignment;

use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    text::Line,
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
};

use crate::app::App;

pub fn render(frame: &mut Frame, app: &App) {
    let outer_layout = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .margin(1)
        .constraints(vec![
            Constraint::Percentage(60),
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let result_layout = Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .margin(1)
        .constraints(vec![Constraint::Percentage(65), Constraint::Percentage(35)])
        .split(outer_layout[0]);

    frame.render_widget(render_last_roll(app), result_layout[0]);
    frame.render_widget(render_roll_history(app), result_layout[1]);
    frame.render_widget(render_command_bar(app), outer_layout[1]);
}

fn render_roll_history<'a>(app: &'a App) -> Table<'a> {
    let rolls: Vec<(String, String, String)> = app
        .app_state
        .roll_history
        .iter()
        .map(|b| (b.input.clone(), "=".into(), b.result.to_string()))
        .collect();

    let rows = rolls.into_iter().map(|roll| {
        Row::new(vec![
            Cell::from(Line::from(roll.0).alignment(ratatui::layout::HorizontalAlignment::Right)),
            Cell::from(Line::from(roll.1)),
            Cell::from(Line::from(roll.2)),
        ])
    });

    let widths = [
        Constraint::Percentage(40),
        Constraint::Length(1),
        Constraint::Length(5),
    ];

    let table =
        Table::new(rows, widths).block(Block::new().title("Roll History").borders(Borders::ALL));

    table
}

fn render_last_roll<'a>(app: &'a App) -> Paragraph<'a> {
    let mut rolls: Vec<String> = app
        .app_state
        .roll_history
        .iter()
        .map(|b| b.to_string())
        .collect();

    let last_roll = rolls.pop().unwrap_or("".into());

    Paragraph::new(last_roll)
        .alignment(ratatui::layout::Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("Last Roll"))
}

fn render_command_bar<'a>(app: &'a App) -> Paragraph<'a> {
    let text = format!("> {}", app.command_bar.text_field.clone());
    Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("Command"))
}
