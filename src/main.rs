mod app;
mod tui;

use crate::app::App;
use crate::tui::run;
use color_eyre::Result;

fn main() -> Result<()> {
    let terminal = ratatui::init();
    let mut app = App::default();
    let result = run(terminal, &mut app);
    ratatui::restore();
    result
}
