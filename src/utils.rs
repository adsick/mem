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
