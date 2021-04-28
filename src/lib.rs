use std::{sync::atomic::{AtomicU64, Ordering}, time::Duration};
use quanta::Instant;

pub struct AtomicInstant(AtomicU64);

impl AtomicInstant {
    pub const fn empty() -> Self {
        Self(AtomicU64::new(0))
    }
    
    pub fn now() -> Self {
        Self(AtomicU64::new(Instant::now().as_unix_duration().as_millis() as u64))
    }

    pub const fn from_millis(millis: u64) -> Self {
        Self(AtomicU64::new(millis))
    }

    pub fn elapsed(&self) -> Duration {
        Duration::from_millis(Instant::now().as_unix_duration().as_millis() as u64 - self.0.load(Ordering::SeqCst))
    }

    pub fn as_millis(&self) -> u64 {
        self.0.load(Ordering::SeqCst)
    }

    pub fn set_now(&self) {
        self.0.store(Instant::now().as_unix_duration().as_millis() as u64, Ordering::SeqCst);
    }

    pub fn set_millis(&self, millis: u64) {
        self.0.store(millis, Ordering::SeqCst);
    }
}