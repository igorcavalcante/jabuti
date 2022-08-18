use std::cmp::{max, min};
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use ticker::Ticker as external_ticker;

const POMODORO_TIME: i16 = 25 * 60;
const SHORT_BREAK_TIME: i16 = 5 * 1;
const LONG_BREAK_TIME: i16 = 15 * 60;

pub trait Sprint {
  fn start(&mut self);
  fn pause(&mut self);
  fn stop(&mut self);
  fn progress(&mut self) -> i8;
  fn remaining(&self) -> i16;
}

#[derive(PartialEq)]
enum Status {
  Started,
  Stoped,
  Paused,
}

pub enum SprintType {
  Pomodoro,
  ShortBreak,
  LongBreak,
}

pub struct SprintImpl {
  enlapsed: Arc<Mutex<i16>>,
  status: Status,
  sprintType: SprintType,
  notification: fn(),
}

impl SprintImpl {
  pub fn new(t: SprintType, n: fn()) -> Self {
    Self {
      enlapsed: Arc::new(Mutex::new(0)),
      status: Status::Stoped,
      sprintType: t,
      notification: n,
    }
  }

  fn tick(&mut self) {
    let enlapsed = Arc::clone(&self.enlapsed);
    let ticker = external_ticker::new(1..POMODORO_TIME + 1, Duration::from_secs(1));

    thread::spawn(move || {
      for i in ticker {
        let mut secs = enlapsed.lock().unwrap();
        *secs = i;
      }
    });
  }

  fn totalTime(&self) -> i16 {
    match self.sprintType {
      SprintType::Pomodoro => POMODORO_TIME,
      SprintType::ShortBreak => SHORT_BREAK_TIME,
      SprintType::LongBreak => LONG_BREAK_TIME,
    }
  }
}

impl Sprint for SprintImpl {
  fn start(&mut self) {
    self.status = Status::Started;
    self.tick();
  }
  fn pause(&mut self) {
    self.status = Status::Paused;
  }
  fn stop(&mut self) {
    self.status = Status::Stoped;
  }

  fn progress(&mut self) -> i8 {
    let enlapsed = *self.enlapsed.lock().unwrap() as f64;
    let percentage = (enlapsed / self.totalTime() as f64 * 100.0) as i8;

    if percentage >= 100 && self.status == Status::Started {
      self.stop();
      let n = self.notification;
      n();
    }

    min(percentage, 100)
  }

  fn remaining(&self) -> i16 {
    let enlapsed = *self.enlapsed.lock().unwrap();
    max(self.totalTime() - enlapsed, 0)
  }
}
