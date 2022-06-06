use crossterm::{cursor, terminal, QueueableCommand};
use std::io::{stdout, Write, Result};

pub fn q_draw_at(x: u16, y: u16, c: char) -> Result<()> {
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
