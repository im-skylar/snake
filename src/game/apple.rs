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

    pub fn draw(&self) -> io::Result<()> {
        q_draw_at(self.pos.0 * 2 + 1, self.pos.1 + 1, 'a'.with(Color::Red))?;
        Ok(())
    }
}
