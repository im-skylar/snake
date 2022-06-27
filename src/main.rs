mod game;
mod highscore;
mod utils;

use bincode::{Decode, Encode};
use clap::Parser;
use crossterm::{self, cursor, terminal, ExecutableCommand};
use std::thread::sleep;
use std::time::Duration;

#[derive(Parser, Debug, Encode, Decode, PartialEq, Copy, Clone)]
#[clap(version, about, long_about = None)]
pub struct Args {
    /// Display Debug information
    #[clap(short = 'D', long)]
    debug: bool,

    /// Wrap around instead of losing, when hitting walls
    #[clap(short = 'W', long)]
    wrap_around: bool,

    /// Move the camera with the snake, so the head is always in the center (implicitly turns on -W
    #[clap(short = 'C', long)]
    camera_move: bool,

    /// Height of the game field
    #[clap(short, long, default_value_t = 10)]
    height: u16,

    /// Width of the game field
    #[clap(short, long, default_value_t = 10)]
    width: u16,

    /// Number of apples
    #[clap(short, long, default_value_t = 1)]
    apples: u8,
}

fn main() {
    let mut args = Args::parse();

    if args.camera_move {
        args.wrap_around = true;
    }

    let mut term = std::io::stdout();
    terminal::enable_raw_mode().unwrap();

    term.execute(cursor::Hide).unwrap();

    let mut g = game::Game::new(args);

    loop {
        g.process_input();
        g.tick();
        g.draw().expect("Failed to write to screen");
        sleep(Duration::from_millis(100));
    }
}
