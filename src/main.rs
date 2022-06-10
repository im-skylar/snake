mod game;
mod utils;

use crossterm::{self, cursor, terminal, ExecutableCommand};
use std::thread::sleep;
use std::time::Duration;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Args {
    /// Wrap around instead of losing, when hitting walls
    #[clap(short = 'W', long)]
    wrap_around: bool,

    /// Height of the game field
    #[clap(short, long, default_value_t = 10)]
    height: u16,

    /// Width of the game field
    #[clap(short, long, default_value_t = 10)]
    width: u16,
}

fn main() {
    let args = Args::parse();

    let mut term = std::io::stdout();
    terminal::enable_raw_mode().unwrap();

    term.execute(cursor::Hide).unwrap();

    let mut g = game::Game::new(
        (args.width, args.height),
        args.wrap_around
    );

    loop {
        g.process_input();
        g.tick();
        g.draw().expect("Failed to write to screen");
        sleep(Duration::from_millis(100));
    }
}
