use std::time::Duration;
use crossbeam_utils::atomic::AtomicCell;
use quanta::Instant;

#[derive(Debug)]
pub struct AtomicInstant(AtomicCell<Instant>);

impl AtomicInstant {
    pub fn now() -> Self {
        Self(AtomicCell::new(Instant::now()))
    }

    pub fn elapsed(&self) -> Duration {
        Instant::now() - self.0.load()
    }

    pub fn duration_since(&self, earlier: Self) -> Duration {
        self.0.load().duration_since(earlier.0.load())
    }

    pub fn set_now(&self) {
        self.0.store(Instant::now());
    }
}
