use crate::app::App;
use color_eyre::{Result, eyre::Ok};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style, Stylize},
    widgets::{BarChart, Block, BorderType, Borders, Paragraph},
};

pub fn run(mut terminal: DefaultTerminal, app: &mut App) -> Result<()> {
    app.update_all();
    loop {
        terminal.draw(|frame| render(frame, &app))?;
        let result = input_handle(app)?;
        if !result {
            break Ok(());
        }
    }
}
fn input_handle(app: &mut App) -> Result<bool> {
    if let Event::Key(key) = event::read()? {
        if key.kind == KeyEventKind::Press {
            match key.code {
                KeyCode::Char('q') => {
                    return Ok(false);
                }
                KeyCode::Up => {
                    if !app.data.is_empty() {
                        app.update_all();
                    }
                }
                _ => {}
            }
        }
    }
    Ok(true)
}

pub fn render(frame: &mut Frame, app: &App) {
    //layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(3),
        ])
        .split(frame.area());

    // Header
    let header = Paragraph::new(" System Infos ")
        .bold()
        .fg(Color::Cyan)
        .centered()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::DarkGray)),
        );
    frame.render_widget(header, chunks[0]);

    //Infos
    let bar_chart = BarChart::default()
        .block(
            Block::default()
                .title(" Usage and Free ")
                .title_style(
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .data(&app.data)
        .bar_width(7)
        .bar_gap(2)
        .bar_style(Style::default().fg(Color::Magenta))
        .value_style(
            Style::default()
                .fg(Color::White)
                .bg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        );

    frame.render_widget(bar_chart, chunks[1]);

    //Footer
    let footer_text = " [↑] Update | [q] Quit ";
    let footer = Paragraph::new(footer_text)
        .fg(Color::Gray)
        .centered()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::DarkGray)),
        );
    frame.render_widget(footer, chunks[2]);
}
