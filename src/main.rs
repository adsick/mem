use mem::common::*;
use std::env::{args, current_dir};

use mem::App;

pub fn main() -> Result<()> {
    let args = args();
    let mut app = App::new(current_dir()?);
    app.run(args);
    Ok(())
}
