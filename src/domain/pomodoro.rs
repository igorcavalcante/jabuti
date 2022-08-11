use std::time::Duration;
use ticker::Ticker;

const POMODORO_TIME: i16 = 25 * 60;
const SHORT_BREAK_TIME: i16 = 5 * 60;
const LONG_BREAK_TIME: i16 = 15 * 60;

pub trait Pomodoro {
  fn init(&mut self, f: fn(i16));
  fn short(&mut self, f: fn(i16));
  fn long(&mut self, f: fn(i16));
}

pub struct PomodoroImpl {
  sprints: i16,
  current: i16,
}

impl PomodoroImpl {
  pub fn new() -> Self {
    Self {
      sprints: 0,
      current: 0,
    }
  }

  fn interval(&mut self, seconds: i16, f: fn(i16)) {
    let ticker = Ticker::new(0..seconds, Duration::from_secs(1));
    for i in ticker {
      self.current = POMODORO_TIME - i;
      f(self.current);
    }
  }
}

impl Pomodoro for PomodoroImpl {
  fn init(&mut self, f: fn(i16)) {
    self.interval(POMODORO_TIME, f)
  }

  fn short(&mut self, f: fn(i16)) {
    self.interval(SHORT_BREAK_TIME, f)
  }

  fn long(&mut self, f: fn(i16)) {
    self.interval(LONG_BREAK_TIME, f)
  }
}
