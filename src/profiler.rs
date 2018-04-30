use common::*;

pub struct Profiler {
  start: Instant,
  frame: u64,
}

impl Profiler {
  pub fn new() -> Profiler {
    Profiler {
      start: Instant::now(),
      frame: 0,
    }
  }

  pub fn frame(&mut self) -> Frame {
    Frame {
      profiler: self,
      _start:    Instant::now(),
    }
  }

  pub fn fps(&self) -> f64 {
    let elapsed = self.start.elapsed();
    let seconds = elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 * 1e-9;
    self.frame as f64 / seconds
  }
}

pub struct Frame<'profiler> {
  profiler: &'profiler mut Profiler,
  _start:    Instant,
}

impl<'profiler> Frame<'profiler> {
  pub fn number(&self) -> u64 {
    self.profiler.frame
  }

  pub fn first(&self) -> bool {
    self.number() == 0
  }
}

impl<'profiler> Drop for Frame<'profiler> {
  fn drop(&mut self) {
    self.profiler.frame += 1;
  }
}
