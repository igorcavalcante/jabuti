use std::cmp::*;
use std::time::{Duration, Instant};

const POMODORO_TIME: i16 = 25 * 60;
const SHORT_BREAK_TIME: i16 = 5 * 1;
const LONG_BREAK_TIME: i16 = 15 * 60;

pub trait Pomodoro {
  fn init(&mut self);
  fn short(&mut self);
  fn long(&mut self);
  fn progress(&self) -> f64;
  fn remaining(&self) -> i16;
}

enum SprintType {
  LONG,
  SHORT,
  POMODORO,
}

pub struct PomodoroImpl {
  createdAt: Instant,
  sprintType: SprintType,
}

impl PomodoroImpl {
  pub fn new() -> Self {
    Self {
      createdAt: Instant::now(),
      sprintType: SprintType::POMODORO,
    }
  }
}

impl Pomodoro for PomodoroImpl {
  fn init(&mut self) {
    self.sprintType = SprintType::POMODORO;
    self.createdAt = Instant::now();
  }

  fn short(&mut self) {
    self.sprintType = SprintType::SHORT;
    self.createdAt = Instant::now();
  }

  fn long(&mut self) {
    self.sprintType = SprintType::LONG;
    self.createdAt = Instant::now();
  }

  fn progress(&self) -> f64 {
    let seconds = (self.createdAt.elapsed().as_secs()) as f64;
    let percentage = match self.sprintType {
      SprintType::POMODORO => seconds / POMODORO_TIME as f64,
      SprintType::LONG => seconds / LONG_BREAK_TIME as f64,
      SprintType::SHORT => seconds / SHORT_BREAK_TIME as f64,
      _ => 100.0,
    };

    if percentage > 1.0 {
      1.0
    } else {
      percentage
    }
  }

  fn remaining(&self) -> i16 {
    let enlapsed = (self.createdAt.elapsed().as_secs()) as i16;

    let remaining = match self.sprintType {
      SprintType::POMODORO => POMODORO_TIME - enlapsed,
      SprintType::LONG => LONG_BREAK_TIME - enlapsed,
      SprintType::SHORT => SHORT_BREAK_TIME - enlapsed,
    };

    max(remaining, 0)
  }
}
