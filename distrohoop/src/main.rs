use crossterm::{
    cursor,
    execute,
    terminal::{self, ClearType},
    ExecutableCommand,
};
use std::io::{stdout, Write};

mod distros;
mod utils;

fn main() -> std::io::Result<()> {
    let mut stdout = stdout();
    let original_mode = terminal::enable_raw_mode()?;
    stdout.execute(terminal::EnterAlternateScreen)?;
    stdout.execute(cursor::Hide)?;
    stdout.flush()?;
    utils::play_animation();
    stdout.execute(terminal::LeaveAlternateScreen)?;
    stdout.execute(cursor::Show)?; 
    terminal::disable_raw_mode()?;
    Ok(())
}