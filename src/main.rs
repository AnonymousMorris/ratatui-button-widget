use std::io::Stdout;
use std::io;

use hex::Hexagon;
use ratatui::prelude::*;
use ratatui::crossterm::event::{self, Event};

mod hex;

fn main() {
    let terminal = ratatui::init();

    let _ = run(terminal);

    ratatui::restore();

}

fn run (mut terminal: Terminal<CrosstermBackend<Stdout>>) -> Result<(), io::Error> {
    loop {
        let hex = Hexagon::new();
        let _ = terminal.draw(|frame| {
            frame.render_widget(hex, frame.area());
        });
        if matches!(event::read()?, Event::Key(_)) {
            break Ok(());
        }
    }
}
