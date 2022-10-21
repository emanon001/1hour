use crate::app::App;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use unicode_width::UnicodeWidthStr;

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(f.size());

    let filter = Paragraph::new(app.filter.as_ref())
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title("Filter"));
    f.render_widget(filter, chunks[0]);
    f.set_cursor(
        // Put cursor past the end of the input text
        chunks[0].x + app.filter.width() as u16 + 1,
        // Move one line down, from the border to the input line
        chunks[0].y + 1,
    );

    let output = Paragraph::new(app.output.as_ref())
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title("Output"));
    f.render_widget(output, chunks[1]);
}