use std::fs::read_dir;

use crate::common::*;
use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
};
pub fn println_colored<T: std::fmt::Display>(
    value: &T,
    fg: Color,
    bg: Color,
) -> Result<(), std::io::Error> {
    use std::io::stdout;
    execute!(
        stdout(),
        SetForegroundColor(fg),
        SetBackgroundColor(bg),
        Print(value),
        ResetColor,
        Print("\n")
    )
}

pub fn print_colored<T: std::fmt::Display>(
    value: &T,
    fg: Color,
    bg: Color,
) -> Result<(), std::io::Error> {
    use std::io::stdout;
    execute!(
        stdout(),
        SetForegroundColor(fg),
        SetBackgroundColor(bg),
        Print(value),
        ResetColor,
    )
}

pub fn traverse_dir(path: &Path) -> Result<()> {
    for entry in read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            traverse_dir(&path);
        }
    }
    Ok(())
}
