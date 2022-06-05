use rand::Rng;
//use console::Term;
use crossterm::{cursor, ExecutableCommand};
use std::io::{self, stdout};

#[derive(Debug)]
pub struct Apple {
    pos: (usize, usize),
}

impl Apple {
    pub fn new(keepout: &[(u16, u16)], field_size: (u16, u16)) -> Apple {
        loop {
            let xpos = rand::thread_rng().gen_range(0..field_size.0 as usize);
            let ypos = rand::thread_rng().gen_range(0..field_size.1 as usize);

            if !keepout.contains(&(xpos as u16, ypos as u16)) {
                return Apple { pos: (xpos, ypos) };
            }
        }
    }

    pub fn draw(&self) -> io::Result<()> {
        stdout().execute(cursor::MoveTo(self.pos.0 as u16, self.pos.1 as u16))?;

        print!("a");

        Ok(())
    }
}
