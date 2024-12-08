use crate::tui::App;
use std::io;

mod cli;
mod handlers;
mod models;
mod selected_tab;
mod tab_widget;
mod tables;
mod tui;
mod validators;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::new().unwrap().run(&mut terminal);
    ratatui::restore();
    app_result
}
