mod game;

use crossterm::{self, cursor, terminal, ExecutableCommand};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut term = std::io::stdout();
    terminal::enable_raw_mode().unwrap();

    term.execute(cursor::Hide).unwrap();

    let mut g = game::Game::new((20, 20));

    loop {
        g.process_input();
        g.tick();
        g.draw().expect("Failed to write to screen");
        sleep(Duration::from_millis(100));
    }
}
