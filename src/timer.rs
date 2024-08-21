use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Timer {
	start_time: Instant,
	duration: Duration,
}

impl Timer {
	pub fn new(seconds: u64) -> Self {
		Timer {
			start_time: Instant::now(),
			duration: Duration::from_secs(seconds),
		}
	}

	pub fn is_elapsed(&self) -> bool {
		self.start_time.elapsed() > self.duration
	}

	pub fn remaining(&self) -> Duration {
		if self.is_elapsed() {
			return Duration::from_secs(0)
		}
		else {
			self.duration - self.start_time.elapsed()
		}
	}
}