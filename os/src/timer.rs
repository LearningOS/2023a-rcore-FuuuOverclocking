//! RISC-V timer-related functionality

use crate::config::CLOCK_FREQ;
use crate::sbi::set_timer;
use riscv::register::time;

/// The number of ticks per second
const TIMER_FREQ: usize = 100;

/// Get the current time in ticks
pub fn get_time() -> usize {
    time::read()
}

/// get current time in milliseconds
#[allow(dead_code)]
pub fn get_time_ms() -> usize {
    time::read() / (CLOCK_FREQ / 1000)
}

/// get current time in microseconds
#[allow(dead_code)]
pub fn get_time_us() -> usize {
    // time::read() * 1_000_000 / CLOCK_FREQ
    time::read() * 2 / 25
}

/// Set the next timer interrupt
pub fn set_next_trigger() {
    set_timer(get_time() + CLOCK_FREQ / TIMER_FREQ);
}

/// Instant, namely timestamp.
pub struct Instant {
    pub ticks: usize,
}

impl Instant {
    pub fn now() -> Self {
        Instant { ticks: get_time() }
    }

    pub fn duration_since(&self, earlier: Instant) -> Duration {
        Duration {
            ticks: self.ticks - earlier.ticks,
        }
    }

    pub fn elapsed(&self) -> Duration {
        Duration {
            ticks: get_time() - self.ticks,
        }
    }

    pub fn as_secs(&self) -> usize {
        self.ticks / CLOCK_FREQ
    }
    pub fn as_millis(&self) -> usize {
        self.ticks / (CLOCK_FREQ / 1000)
    }
    pub fn as_micros(&self) -> usize {
        self.ticks * 2 / 25
    }
}

/// Duration, namely time-span.
pub struct Duration {
    pub ticks: usize,
}

impl Duration {
    pub fn as_secs(&self) -> usize {
        self.ticks / CLOCK_FREQ
    }
    pub fn as_millis(&self) -> usize {
        self.ticks / (CLOCK_FREQ / 1000)
    }
    pub fn as_micros(&self) -> usize {
        self.ticks * 2 / 25
    }
}
