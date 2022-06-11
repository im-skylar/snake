use crossterm::{cursor, terminal, QueueableCommand};
use std::fmt::Display;
use std::io::{stdout, Result, Write};

pub fn q_draw_at<T: Display>(x: u16, y: u16, c: T) -> Result<()> {
    stdout().queue(cursor::MoveTo(x, y))?;
    print!("{}", c);
    Ok(())
}

pub fn q_clear() -> Result<()> {
    stdout().queue(terminal::Clear(terminal::ClearType::All))?;
    Ok(())
}

pub fn q_flush() -> Result<()> {
    stdout().flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::utils::*;

    #[test]
    fn q_draw_at_test() -> Result<()> {
        q_draw_at(0, 0, ' ')?;
        q_draw_at(0, 5, "Hello")?;
        q_draw_at(0, 5, 15)?;

        Ok(())
    }

    #[test]
    fn q_clear_test() -> Result<()> {
        q_clear()?;

        Ok(())
    }

    #[test]
    fn q_flush_test() -> Result<()> {
        q_flush()?;

        Ok(())
    }
}
