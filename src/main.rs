mod domain;

use domain::pomodoro::*;

fn main() {
  PomodoroImpl::new().init(printer);
}

fn printer(t: i16) {
  let minutes = t / 60;
  let seconds = t % 60;
  println!("{:}:{:02}", minutes, seconds);
}
