use crate::utils::q_draw_at;
use crossterm::style::{Color, Stylize};
use std::io;

#[derive(Debug, Clone)]
pub struct Apple {
    pub pos: (u16, u16),
}

impl Apple {
    pub fn new(pos: (u16, u16)) -> Apple {
        Apple { pos }
    }

    pub fn draw(&self, trans: (i16, i16), size: (u16, u16)) -> io::Result<()> {
        q_draw_at(
            (self.pos.0 as i16 + trans.0).rem_euclid(size.0 as i16) as u16 * 2 + 1,
            (self.pos.1 as i16 + trans.1).rem_euclid(size.1 as i16) as u16 + 1,
            'a'.with(Color::Red),
        )?;
        Ok(())
    }
}
