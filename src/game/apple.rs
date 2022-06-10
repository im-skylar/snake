use crate::utils::q_draw_at;
use crossterm::style::{Color, Stylize};
use rand::Rng;
use std::io;

#[derive(Debug)]
pub struct Apple {
    pub pos: (u16, u16),
}

impl Apple {
    pub fn new(keepout: &[(u16, u16)], field_size: (u16, u16)) -> Apple {
        loop {
            let xpos = rand::thread_rng().gen_range(0..field_size.0 as usize) as u16;
            let ypos = rand::thread_rng().gen_range(0..field_size.1 as usize) as u16;

            if !keepout.contains(&(xpos, ypos)) {
                return Apple { pos: (xpos, ypos) };
            }
        }
    }

    pub fn draw(&self) -> io::Result<()> {
        q_draw_at(self.pos.0 * 2 + 1, self.pos.1 + 1, 'a'.with(Color::Red))?;
        Ok(())
    }
}
