mod apple;
mod snake;

use crossterm::{self, event};
use std::io;

use crate::utils::{q_clear, q_draw_at, q_flush};

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
                        'w' => self.snake.update_dir((0, -1)),
                        's' => self.snake.update_dir((0, 1)),
                        'a' => self.snake.update_dir((-1, 0)),
                        'd' => self.snake.update_dir((1, 0)),
                        _ => (),
                    },
                    _ => println!("dada"),
                }
            }
        }
    }

    pub fn tick(&mut self) {
        self.snake.update_pos(self.size);
        
        let head = *self.snake.pos.last().unwrap();
        
        // Check whether snake is in itself
        if self.snake.pos[..self.snake.pos.len()-2].contains(&head) {
            self.loose();
        }

        // Check whether snake is on apple
        if head == self.apple.pos {
            self.apple = apple::Apple::new(&self.snake.pos, self.size);
        } else {
            self.snake.rm_tail();
        }
    }

    pub fn draw(&mut self) -> io::Result<()> {
        q_clear()?;

        for x in 0..self.size.0*2+1 {
            q_draw_at(x, 0, '+')?;
            q_draw_at(x, self.size.1+1, '+')?;
        }

        for y in 0..self.size.1+2 {
            q_draw_at(0, y, '+')?;
            q_draw_at(self.size.0*2, y, '+')?;
        }

        self.snake.draw()?;
        self.apple.draw()?;

        q_flush()?;

        Ok(())
    }

    pub fn loose(&self) {
        q_clear().unwrap();
        q_draw_at(0, 0, ' ').unwrap();

        println!("You lost :(");
        q_draw_at(0, 1, ' ').unwrap();
        println!("Points: {}", self.snake.pos.len());

        std::process::exit(0);
    }
}
