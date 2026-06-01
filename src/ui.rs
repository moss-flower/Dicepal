use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::{Color, Stylize},
    symbols::block,
    widgets::{Block, Borders, List, ListState, Paragraph},
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

    let mut state = ListState::default();
    state.select_last();

    frame.render_stateful_widget(render_roll_history(app), outer_layout[0], &mut state);

    frame.render_widget(render_command_bar(app), outer_layout[1]);
}

fn render_roll_history<'a>(app: &'a App) -> List<'a> {
    let rolls: Vec<String> = app
        .app_state
        .roll_history
        .iter()
        .map(|b| b.to_string())
        .collect();

    let list = List::new(rolls).block(Block::default().borders(Borders::ALL).title("Rolls"));
    list
}

fn render_command_bar<'a>(app: &'a App) -> Paragraph<'a> {
    let text = format!("> {}", app.command_bar.text_field.clone());
    Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("Command"))
}
