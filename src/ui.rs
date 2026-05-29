use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::{Color, Stylize},
    symbols::block,
    widgets::{Block, Borders, Paragraph},
};

use crate::app::App;

pub fn render(frame: &mut Frame, app: &App) {
    let outer_layout = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .margin(1)
        .constraints(vec![
            Constraint::Length(60),
            Constraint::Percentage(3),
            Constraint::Length(3),
        ])
        .split(frame.area());
    frame.render_widget(
        Paragraph::new("Nothing to see here")
            .block(Block::new().borders(Borders::ALL).title("Result Window"))
            .fg(Color::Green),
        outer_layout[0],
    );

    frame.render_widget(render_command_bar(app), outer_layout[1]);
}

fn render_command_bar<'a>(app: &'a App) -> Paragraph<'a> {
    let text = format!("> {}", app.command_bar.text_field.clone());
    Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("Command"))
}
