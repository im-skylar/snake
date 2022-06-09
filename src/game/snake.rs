use crate::utils::q_draw_at;
use std::io;
use crossterm::style::{Stylize, Color};

pub struct Snake {
    /// Positions of body parts as x y
    pub pos: Vec<(u16, u16)>,
    /// Direction of snake as +-1 for x or y, negative is up
    pub dir: (i16, i16),
}

impl Snake {
    pub fn new(field_size: (u16, u16)) -> Snake {
        let pos = vec![(field_size.0 / 2, field_size.1 / 2)];

        Snake { pos, dir: (1, 0) }
    }

    pub fn draw(&self) -> io::Result<()> {
        let mut last_x: i16 = -2; // Any number < -1 works

        for (x, y) in &self.pos {
            q_draw_at(
                *x * 2 + 1,
                *y + 1,
                '#'.on(Color::Green)
            )?;
            
            // This part is used to fill the gaps when the snake is 
            // moving horizontally
            let diff_x = last_x - *x as i16;
            
            if diff_x.abs() == 1 {
                q_draw_at(
                    *x * 2 + (1 + diff_x) as u16, // This is safe, because diff_x is never < -1 in this if block
                    *y + 1,
                    '#'.on(Color::Green)
                )?;
            }
            
            last_x = *x as i16;
        }

        Ok(())
    }

    pub fn update_pos(&mut self, size: (u16, u16)) {
        let head = *self.pos.last().unwrap();

        // Remove the Tail
        //self.pos.remove(0);

        // Create a new head in the direction the snake was going
        let xpos = (head.0 as i16 + self.dir.0).rem_euclid(size.0 as i16) as u16;
        let ypos = (head.1 as i16 + self.dir.1).rem_euclid(size.1 as i16) as u16;
        self.pos.push((xpos, ypos));
    }

    pub fn update_dir(&mut self, dir: (i16, i16)) {
        if self.dir.0 == -dir.0 || self.dir.1 == -dir.1 {
            return();
        }

        self.dir = dir;
    }

    pub fn rm_tail(&mut self) {
        self.pos.remove(0);
    }
}
