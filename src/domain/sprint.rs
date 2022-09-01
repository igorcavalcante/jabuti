use std::cmp::{max, min};
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use ticker::Ticker as external_ticker;

const POMODORO_TIME: i16 = 25 * 60;
const SHORT_BREAK_TIME: i16 = 5 * 60;
const LONG_BREAK_TIME: i16 = 15 * 60;

pub trait Sprint {
  fn start(&mut self);
  fn pause_toggle(&mut self);
  fn stop(&mut self);
  fn progress(&mut self) -> i8;
  fn remaining(&self) -> i16;
}

#[derive(PartialEq)]
enum Status {
  Started,
  Stopped,
  Paused,
}

pub enum SprintType {
  Pomodoro,
  ShortBreak,
  LongBreak,
}

pub struct SprintImpl {
  elapsed: Arc<Mutex<i16>>,
  status: Arc<Mutex<Status>>,
  sprint_type: SprintType,
  notification: fn(),
}

impl SprintImpl {
  pub fn new(t: SprintType, n: fn()) -> Self {
    Self {
      elapsed: Arc::new(Mutex::new(0)),
      status: Arc::new(Mutex::new(Status::Stopped)),
      sprint_type: t,
      notification: n,
    }
  }

  fn tick(&mut self) {
    let elapsed = Arc::clone(&self.elapsed);
    let status = Arc::clone(&self.status);
    let ticker = external_ticker::new(*elapsed.lock().unwrap()+1..POMODORO_TIME, Duration::from_secs(1));

    thread::spawn(move || {
      for i in ticker {
        if *status.lock().unwrap() == Status::Started {
          let mut secs = elapsed.lock().unwrap();
          *secs = i;
        } else {
          break
        }
      }
    });
  }

  fn total_time(&self) -> i16 {
    match self.sprint_type {
      SprintType::Pomodoro => POMODORO_TIME,
      SprintType::ShortBreak => SHORT_BREAK_TIME,
      SprintType::LongBreak => LONG_BREAK_TIME,
    }
  }
}

impl Sprint for SprintImpl {
  fn start(&mut self) {
    let status = Arc::clone(&self.status);
    *status.lock().unwrap() = Status::Started;
    self.tick();
  }

  //TODO refact plz
  fn pause_toggle(&mut self) {
    let status_clone = Arc::clone(&self.status);
    let mut status = status_clone.lock().unwrap();

    if *status != Status::Paused {
      *status = Status::Paused;
    } else {
      drop(status);
      self.start();
    }
  }

  fn stop(&mut self) {
    let status = Arc::clone(&self.status);
    *status.lock().unwrap() = Status::Stopped;
  }

  fn progress(&mut self) -> i8 {
    let elapsed = *self.elapsed.lock().unwrap() as f64;
    let status = Arc::clone(&self.status);
    let percentage = (elapsed / self.total_time() as f64 * 100.0) as i8;

    if percentage >= 100 && *status.lock().unwrap() == Status::Started {
      self.stop();
      let n = self.notification;
      n();
    }

    min(percentage, 100)
  }

  fn remaining(&self) -> i16 {
    let elapsed = *self.elapsed.lock().unwrap();
    max(self.total_time() - elapsed, 0)
  }
}
