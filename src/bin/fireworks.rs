use crossterm::{
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    ExecutableCommand,
};
use std::io::stdout;

fn main() -> std::io::Result<()> {
    stdout()
        .execute(SetForegroundColor(Color::Blue))?
        .execute(SetBackgroundColor(Color::Red))?
        .execute(Print("Styled text here."))?
        .execute(ResetColor)?;
    Ok(())
}
