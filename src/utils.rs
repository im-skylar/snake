use crossterm::{cursor, terminal, QueueableCommand};
use std::io::{stdout, Write, Result};
use std::fmt::Display;

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
