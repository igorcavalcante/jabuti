use std::cmp::*;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::{Duration, Instant};
use ticker::Ticker;

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
  pub seconds: Arc<Mutex<i16>>,
}

impl PomodoroImpl {
  pub fn new() -> Self {
    Self {
      createdAt: Instant::now(),
      sprintType: SprintType::POMODORO,
      seconds: Arc::new(Mutex::new(0)),
    }
  }

  fn tick(&self) {
    let se_clone = Arc::clone(&self.seconds);
    let duration = match self.sprintType {
      SprintType::POMODORO => POMODORO_TIME,
      SprintType::LONG => LONG_BREAK_TIME,
      SprintType::SHORT => SHORT_BREAK_TIME,
    };
    let ticker = Ticker::new(1..duration + 1, Duration::from_secs(1));
    thread::spawn(move || {
      for i in ticker {
        let mut secs = se_clone.lock().unwrap();
        send -> mesg
        *secs = i;
      }
    });
  }
}

impl Pomodoro for PomodoroImpl {
  fn init(&mut self) {
    self.sprintType = SprintType::POMODORO;
    self.createdAt = Instant::now();
    self.tick();
  }

  fn short(&mut self) {
    self.sprintType = SprintType::SHORT;
    self.createdAt = Instant::now();
    self.tick();
  }

  fn long(&mut self) {
    self.sprintType = SprintType::LONG;
    self.createdAt = Instant::now();
    self.tick()
  }

  fn progress(&self) -> f64 {
    let seconds = *self.seconds.lock().unwrap() as f64;
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
    let enlapsed = self.seconds.lock().unwrap();
    let remaining = match self.sprintType {
      SprintType::POMODORO => POMODORO_TIME - *enlapsed,
      SprintType::LONG => LONG_BREAK_TIME - *enlapsed,
      SprintType::SHORT => SHORT_BREAK_TIME - *enlapsed,
    };

    max(remaining, 0)
  }
}
