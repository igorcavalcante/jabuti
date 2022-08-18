use crate::domain::sprint::*;

pub trait PomodoroTimer {
  fn startPomodoro(&mut self);
  fn startShortInterval(&mut self);
  fn startLongInterval(&mut self);
  fn pause(&mut self);
  fn loadProgress(&mut self) -> i8;
  fn loadRemainingTime(&self) -> i16;
}

pub struct PomodoroTimerImpl {
  sprints: Vec<Box<dyn Sprint>>,
  active: Box<dyn Sprint>,
  notification: fn(),
}

impl PomodoroTimerImpl {
  pub fn new(n: fn()) -> Self {
    Self {
      sprints: Vec::new(),
      active: Box::new(SprintImpl::new(SprintType::Pomodoro, n)),
      notification: n,
    }
  }

  fn start(&mut self, t: SprintType) {
    let mut sprint = SprintImpl::new(t, self.notification);
    sprint.start();
    self.active = Box::new(sprint);
  }
}

impl PomodoroTimer for PomodoroTimerImpl {
  fn startPomodoro(&mut self) {
    self.start(SprintType::Pomodoro);
  }

  fn startShortInterval(&mut self) {
    self.start(SprintType::ShortBreak);
  }

  fn startLongInterval(&mut self) {
    self.start(SprintType::LongBreak);
  }

  fn loadProgress(&mut self) -> i8 {
    self.active.progress()
  }

  fn loadRemainingTime(&self) -> i16 {
    self.active.remaining()
  }

  fn pause(&mut self) {
    self.active.pause()
  }
}