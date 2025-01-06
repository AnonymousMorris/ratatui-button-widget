use std::io::Stdout;
use std::io;

use ratatui::prelude::*;
use ratatui::crossterm::event::{self, Event};

use hex::Hexagon;
use play::Play;

mod hex;
mod play_button;
mod play;

fn main() {
    let terminal = ratatui::init();

    let _ = run(terminal);

    ratatui::restore();

}

fn run (mut terminal: Terminal<CrosstermBackend<Stdout>>) -> Result<(), io::Error> {
    loop {
        let hex = Hexagon::new();
        let play = Play::new();
        // play.render();
        let _ = terminal.draw(|frame| {
            let inner = hex.inner(frame.area());
            frame.render_widget(hex, frame.area());
            // play.render(frame.area(), frame.buffer_mut());
            frame.render_widget(play, inner);
        });
        if matches!(event::read()?, Event::Key(_)) {
            break Ok(());
        }
    }
}
