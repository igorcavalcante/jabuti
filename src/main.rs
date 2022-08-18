mod domain;
mod ui;

use crate::ui::run;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
  run()?;
  Ok(())
}
