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

// credit: https://stackoverflow.com/a/54306906/18543872
pub fn expand_tilde<P: AsRef<Path>>(path_user_input: P) -> Option<PathBuf> {
    let p = path_user_input.as_ref();
    if !p.starts_with("~") {
        return Some(p.to_path_buf());
    }
    if p == Path::new("~") {
        return dirs::home_dir();
    }
    dirs::home_dir().map(|mut h| {
        if h == Path::new("/") {
            // Corner case: `h` root directory;
            // don't prepend extra `/`, just drop the tilde.
            p.strip_prefix("~").unwrap().to_path_buf()
        } else {
            h.push(p.strip_prefix("~/").unwrap());
            h
        }
    })
}
