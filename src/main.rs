use color_eyre::Result;
use crossterm::event::{self, Event};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    widgets::{BarChart, Block, Borders, Paragraph},
};

struct App {
    data: Vec<(&'static str, u64)>,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app = App {
        data: vec![("A", 10), ("B", 20), ("C", 12), ("D", 50), ("E", 40)],
    };
    let result = run(terminal, &app);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal, app: &App) -> Result<()> {
    loop {
        terminal.draw(|frame| render(frame, &app))?;
        if matches!(event::read()?, Event::Key(_)) {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame, app: &App) {
    let out_layer = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(vec![Constraint::Fill(60), Constraint::Fill(40)])
        .split(frame.area());

    frame.render_widget(
        Paragraph::new("outer 0").block(Block::new().bold().fg(Color::Red).borders(Borders::ALL)),
        out_layer[0],
    );
    frame.render_widget(
        Paragraph::new("outer 1").block(Block::new().bold().fg(Color::Blue).borders(Borders::ALL)),
        out_layer[1],
    );
    let bar_chart = BarChart::default()
        .block(Block::default().title("BarChart").borders(Borders::ALL))
        .data(&app.data)
        .bar_width(5)
        .bar_style(Style::default().fg(Color::Red))
        .value_style(Style::default().fg(Color::Black).bg(Color::Blue));
    frame.render_widget(bar_chart, out_layer[0]);
}
