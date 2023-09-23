use bevy::prelude::*;

use std::time::{Instant, Duration};

enum TimerState {
    Stopped { elapsed: Duration },
    Running { start: Instant, elapsed: Duration },
}

#[derive(Resource)]
pub struct Timer {
    state: TimerState,
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            state: TimerState::Stopped { elapsed: Duration::new(0, 0) },
        }
    }

    pub fn start(&mut self) {
        match &self.state {
            TimerState::Stopped { elapsed } => {
                self.state = TimerState::Running {
                    start: Instant::now(),
                    elapsed: *elapsed,
                };
            }
            TimerState::Running { .. } => {} 
        }
    }

    pub fn get(&self) -> Duration {
        match &self.state {
            TimerState::Stopped { elapsed } => *elapsed,
            TimerState::Running { start, elapsed } => *elapsed + start.elapsed(),
        }
    }

    pub fn stop(&mut self) {
        if let TimerState::Running { start, elapsed } = &self.state {
            self.state = TimerState::Stopped {
                elapsed: *elapsed + start.elapsed(),
            };
        }
    }

    pub fn reset(&mut self) {
        self.state = TimerState::Stopped { elapsed: Duration::new(0, 0) };
    }
}