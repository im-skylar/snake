use crossterm::{cursor, ExecutableCommand};
use std::io::{self, stdout};

pub struct Snake {
    /// Positions of body parts as x y
    pos: Vec<(u16, u16)>,
    /// Direction of snake as +-1 for x or y, negative is up
    pub dir: (i16, i16),
}

impl Snake {
    pub fn new(field_size: (u16, u16)) -> Snake {
        let pos = vec![(field_size.0 / 2, field_size.1 / 2)];

        Snake { pos, dir: (0, 0) }
    }

    pub fn draw(&self) -> io::Result<()> {
        for (x, y) in &self.pos {
            stdout().execute(cursor::MoveTo(*x as u16, *y as u16))?;

            print!("#");
        }

        Ok(())
    }

    pub fn update_pos(&mut self, size: (u16, u16)) {
        let head = *self.pos.last().unwrap();

        // Remove the Tail
        self.pos.remove(0);

        // Create a new head in the direction the snake was going
        let xpos: u16 = (head.0 as i16 + self.dir.0).rem_euclid(size.0 as i16) as u16;
        let ypos: u16 = (head.1 as i16 + self.dir.1).rem_euclid(size.1 as i16) as u16;
        self.pos.push((xpos, ypos));
    }
}
