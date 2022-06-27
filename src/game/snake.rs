use crate::utils::q_draw_at;
use crossterm::style::{Color, Stylize};
use std::io;

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

    pub fn draw(&self, trans: (i16, i16), size: (u16, u16)) -> io::Result<()> {
        let mut last_x: i16 = -2; // Any number < -1 works

        for (x, y) in &self.pos {
            q_draw_at(
                (*x as i16 + trans.0).rem_euclid(size.0 as i16) as u16 * 2 + 1,
                (*y as i16 + trans.1).rem_euclid(size.1 as i16) as u16 + 1,
                '#'.on(Color::Green),
            )?;

            // This part is used to fill the gaps when the snake is
            // moving horizontally
            let diff_x = last_x - *x as i16;

            if diff_x.abs() == 1 {
                q_draw_at(
                    // This is safe, because diff_x is never < -1 in this if block
                    ((*x as i16 + trans.0).rem_euclid(size.0 as i16) * 2 + (1 + diff_x)) as u16,
                    (*y as i16 + trans.1).rem_euclid(size.1 as i16) as u16 + 1,
                    '#'.on(Color::Green),
                    //'#'.on(Color::Red),
                )?;
            } else if last_x == size.0 as i16 - 1 && *x == 0 {
                q_draw_at(
                    ((*x as i16 + trans.0).rem_euclid(size.0 as i16) * 2) as u16,
                    (*y as i16 + trans.1).rem_euclid(size.1 as i16) as u16 + 1,
                    '#'.on(Color::Green),
                    //'#'.on(Color::White),
                )?;
            } else if last_x == 0 && *x == size.0 - 1 {
                q_draw_at(
                    ((*x as i16 + trans.0).rem_euclid(size.0 as i16) * 2 + 2) as u16,
                    (*y as i16 + trans.1).rem_euclid(size.1 as i16) as u16 + 1,
                    '#'.on(Color::Green),
                    //'#'.on(Color::Blue),
                )?;
            }

            last_x = *x as i16;
        }

        Ok(())
    }

    pub fn update_pos(&mut self) {
        let head = *self.pos.last().unwrap();

        let xpos = head.0 as i16 + self.dir.0;
        let ypos = head.1 as i16 + self.dir.1;

        self.pos.push((xpos as u16, ypos as u16));
    }

    pub fn update_dir(&mut self, dir: (i16, i16)) {
        if self.dir.0 == -dir.0 || self.dir.1 == -dir.1 {
            return;
        }

        self.dir = dir;
    }

    pub fn rm_tail(&mut self) {
        self.pos.remove(0);
    }
}

#[cfg(test)]
mod test {
    use crate::game::snake::*;

    #[test]
    fn update_pos_test() {
        let mut s = Snake::new((10, 10));
        s.dir = (1, 0);
        assert_eq!(s.pos[0], (5, 5));

        s.update_pos();

        assert_eq!(s.pos.len(), 2);
        assert_eq!(s.pos[0], (5, 5));
        assert_eq!(s.pos[1], (6, 5));

        s.rm_tail();

        assert_eq!(s.pos.len(), 1);
        assert_eq!(s.pos[0], (6, 5));
    }

    #[test]
    fn update_dir_test() {
        let mut s = Snake::new((10, 10));
        s.dir = (1, 0);
        s.update_dir((-1, 0));
        assert_eq!(s.dir, (1, 0));
        s.update_dir((0, 1));
        assert_eq!(s.dir, (0, 1));
    }
}
