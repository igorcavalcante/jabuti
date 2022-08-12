mod crossterm;
mod domain;
mod ui;

use crate::crossterm::run;
use domain::pomodoro::*;
use std::{error::Error, time::Duration};

fn main() -> Result<(), Box<dyn Error>> {
  run()?;
  Ok(())
}
