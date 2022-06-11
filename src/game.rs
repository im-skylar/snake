mod apple;
mod snake;

use crate::utils::{q_clear, q_draw_at, q_flush};
use crossterm::{self, event, ExecutableCommand};
use rand::seq::IteratorRandom;
use std::io;

pub struct Game {
    /// The snake
    snake: snake::Snake,

    /// The Apples
    apple: Vec<apple::Apple>,

    /// the size of the field in x y
    size: (u16, u16),

    /// Whether or not the walls should wrap around instead of making you loose
    wrap_around: bool,

    /// Positions, where there is no apple (but maybe snake)
    allowed_area: Vec<(u16, u16)>,
}

impl Game {
    pub fn new(size: (u16, u16), wrap_around: bool, apples: u8) -> Game {
        //! Create a new game object

        let mut g = Game {
            snake: snake::Snake::new(size),
            apple: vec![],
            size,
            wrap_around,
            allowed_area: vec![],
        };

        for x in 0..g.size.0 {
            for y in 0..g.size.1 {
                g.allowed_area.push((x, y));
            }
        }

        for _ in 0..apples {
            g.add_apple();
        }

        g
    }

    fn add_apple(&mut self) {
        //! Filter out the positions, where there is no snake from the list of
        //! positions, where theres no apple and then create a new apple on a
        //! random position from that filtered list

        let new_pos = self
            .allowed_area
            .iter()
            .filter(|x| !self.snake.pos.contains(x))
            .choose(&mut rand::thread_rng());

        if let Some(pos) = new_pos {
            if let Some(idx) = self.allowed_area.iter().position(|x| *x == *pos) {
                let pos = self.allowed_area.swap_remove(idx);
                self.apple.push(apple::Apple::new(pos));
            }
        }
    }

    pub fn process_input(&mut self) {
        //! Read keyboard for wasdq and call the according function

        if event::poll(std::time::Duration::ZERO).unwrap() {
            if let event::Event::Key(e) = event::read().unwrap() {
                match e {
                    event::KeyEvent {
                        code: event::KeyCode::Char(c),
                        ..
                    } => match c {
                        'q' => self.loose(),
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
        //! Update positions of snake and checks for Walls or apples eaten

        self.snake.update_pos();

        // Check whether snake is outside of game field
        let head = self.snake.pos.last_mut().unwrap(); // Mutable!

        if self.wrap_around {
            // Wrap around when hitting a wall
            if head.0 >= self.size.0 {
                if self.snake.dir.0 == 1 {
                    *head = (0, head.1);
                } else {
                    *head = (self.size.0 - 1, head.1);
                }
            }

            if head.1 >= self.size.1 {
                if self.snake.dir.1 == 1 {
                    *head = (head.0, 0);
                } else {
                    *head = (head.0, self.size.1 - 1);
                }
            }
        } else {
            // Loose when hitting a wall
            if head.0 >= self.size.0 || head.1 >= self.size.1 {
                self.loose();
            }
        }

        let head = self.snake.pos.last().unwrap(); // Immutable!

        // Check whether snake is in itself
        if self.snake.pos[..self.snake.pos.len() - 2].contains(head) {
            // Safe, because the Snakes len is always > 1 here and tail is only removed later in apple check
            self.snake.pos.pop();
            self.loose();
            return;
        }

        // Check whether snake is on apple
        let idx = self.apple.iter().position(|x| &x.pos == head); // Get Index of Apple the Snake is on
        if let Some(idx) = idx {
            let apple = self.apple.remove(idx);
            self.allowed_area.push(apple.pos);
            self.add_apple();
        } else {
            self.snake.rm_tail();
        }
    }

    pub fn draw(&mut self) -> io::Result<()> {
        //! Draw the field to the screen

        q_clear()?;

        // Box around the field
        for x in 0..self.size.0 * 2 + 1 {
            q_draw_at(x, 0, '+')?;
            q_draw_at(x, self.size.1 + 1, '+')?;
        }

        for y in 0..self.size.1 + 2 {
            q_draw_at(0, y, '+')?;
            q_draw_at(self.size.0 * 2, y, '+')?;
        }

        // Snake and apples
        self.snake.draw()?;
        for a in &self.apple {
            a.draw()?;
        }

        // Score
        q_draw_at(
            0,
            self.size.1 + 2,
            format!("Score: {:3}", self.snake.pos.len()),
        )?;

        q_flush()?;

        Ok(())
    }

    fn loose(&self) {
        //! Display the score, clean up and exit

        q_clear().unwrap();
        q_draw_at(0, 0, ' ').unwrap();

        println!("You lost :(");
        q_draw_at(0, 1, ' ').unwrap();
        println!("Points: {}", self.snake.pos.len());

        crossterm::terminal::disable_raw_mode().unwrap();
        std::io::stdout().execute(crossterm::cursor::Show).unwrap();

        std::process::exit(0);
    }
}

#[cfg(test)]
mod tests {
    use crate::game::*;

    #[test]
    fn wrap_around() {
        let mut g = Game::new((10, 10), true, 0);
        g.snake.pos = vec![(9, 5)];
        g.tick();
        assert_eq!(g.snake.pos[0], (0, 5));

        let mut g = Game::new((10, 10), true, 0);
        g.snake.pos = vec![(0, 5)];
        g.snake.dir = (-1, 0);
        g.tick();
        assert_eq!(g.snake.pos[0], (9, 5));

        let mut g = Game::new((10, 10), true, 0);
        g.snake.pos = vec![(3, 9)];
        g.snake.dir = (0, 1);
        g.tick();
        assert_eq!(g.snake.pos[0], (3, 0));

        let mut g = Game::new((10, 10), true, 0);
        g.snake.pos = vec![(3, 0)];
        g.snake.dir = (0, -1);
        g.tick();
        assert_eq!(g.snake.pos[0], (3, 9));
    }

    #[test]
    fn eat_apple() {
        let mut g = Game::new((10, 10), true, 1);
        g.apple[0].pos = (3, 5);
        g.snake.pos = vec![(2, 5)];
        g.tick();
        assert_eq!(g.snake.pos.len(), 2);
        assert_ne!(g.apple[0].pos, (3, 5));
        assert_eq!(g.apple.len(), 1);
    }
}
