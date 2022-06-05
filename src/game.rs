mod apple;
mod snake;

use crossterm::{self, event, terminal, QueueableCommand};
use std::io;
use std::io::Write;

pub struct Game {
    snake: snake::Snake,
    apple: apple::Apple,
    size: (u16, u16),
}

impl Game {
    pub fn new(size: (u16, u16)) -> Game {
        Game {
            snake: snake::Snake::new(size),
            apple: apple::Apple::new(&[], size),
            size,
        }
    }

    pub fn process_input(&mut self) {
        if event::poll(std::time::Duration::ZERO).unwrap() {
            if let event::Event::Key(e) = event::read().unwrap() {
                match e {
                    event::KeyEvent {
                        code: event::KeyCode::Char(c),
                        ..
                    } => match c {
                        'q' => std::process::exit(0),
                        'w' => self.snake.dir = (0, -1),
                        's' => self.snake.dir = (0, 1),
                        'a' => self.snake.dir = (-1, 0),
                        'd' => self.snake.dir = (1, 0),
                        _ => (),
                    },
                    _ => println!("dada"),
                }
            }
        }
    }

    pub fn tick(&mut self) {
        self.snake.update_pos(self.size);
    }

    pub fn draw(&mut self) -> io::Result<()> {
        let mut stdout = std::io::stdout();
        stdout.queue(terminal::Clear(terminal::ClearType::All))?;

        self.snake.draw()?;
        self.apple.draw()?;

        stdout.flush()?;

        Ok(())
    }
}
