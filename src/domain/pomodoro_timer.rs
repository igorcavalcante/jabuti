use crate::domain::sprint::*;

pub trait PomodoroTimer {
  fn start_pomodoro(&mut self);
  fn start_short_interval(&mut self);
  fn start_long_interval(&mut self);
  fn pause_toggle(&mut self);
  fn load_progress(&mut self) -> i8;
  fn load_remaining_time(&self) -> i16;
}

pub struct PomodoroTimerImpl {
  //sprints: Vec<Box<dyn Sprint>>,
  active: Box<dyn Sprint>,
  notification: fn(),
}

impl PomodoroTimerImpl {
  pub fn new(n: fn()) -> Self {
    Self {
      //sprints: Vec::new(),
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
  fn start_pomodoro(&mut self) {
    self.start(SprintType::Pomodoro);
  }

  fn start_short_interval(&mut self) {
    self.start(SprintType::ShortBreak);
  }

  fn start_long_interval(&mut self) {
    self.start(SprintType::LongBreak);
  }

  fn pause_toggle(&mut self) {
    self.active.pause_toggle()
  }

  fn load_progress(&mut self) -> i8 {
    self.active.progress()
  }

  fn load_remaining_time(&self) -> i16 {
    self.active.remaining()
  }
}
