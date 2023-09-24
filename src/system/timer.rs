#[cfg(not(target_arch = "wasm32"))]
pub mod platform {
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

        pub fn get_sec(&self) -> u64 {
            match &self.state {
                TimerState::Stopped { elapsed } => elapsed.as_secs(),
                TimerState::Running { start, elapsed } => (*elapsed + start.elapsed()).as_secs(),
            }
        }
        
        pub fn get_milli_sec(&self) -> u128 {
            match &self.state {
                TimerState::Stopped { elapsed } => elapsed.as_millis(),
                TimerState::Running { start, elapsed } => (*elapsed + start.elapsed()).as_millis(),
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
}
#[cfg(target_arch = "wasm32")]
pub mod platform {
    use bevy::prelude::*;
    use js_sys::Date;
    enum TimerState {
        Stopped { elapsed: f64 },
        Running { start: f64, elapsed: f64 },
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
                state: TimerState::Stopped { elapsed: 0.0 },
            }
        }
    
        pub fn start(&mut self) {
            match &self.state {
                TimerState::Stopped { elapsed } => {
                    self.state = TimerState::Running {
                        start: Date::now(),
                        elapsed: *elapsed,
                    };
                }
                TimerState::Running { .. } => {} 
            }
        }
    
        pub fn get_sec(&self) -> u64 {
            match &self.state {
                TimerState::Stopped { elapsed } => (*elapsed as f64 / 1000.0) as u64,
                TimerState::Running { start, elapsed } => ((*elapsed as f64 + (Date::now() - *start as f64)) / 1000.0) as u64,
            }
        }

        pub fn get_milli_sec(&self) -> u128 {
            match &self.state {
                TimerState::Stopped { elapsed } => *elapsed as u128,
                TimerState::Running { start, elapsed } => (*elapsed as f64 + (Date::now() - *start as f64)) as u128,
            }
        }
    
        pub fn stop(&mut self) {
            if let TimerState::Running { start, elapsed } = &self.state {
                self.state = TimerState::Stopped {
                    elapsed: *elapsed + (Date::now() - *start),
                };
            }
        }
    
        pub fn reset(&mut self) {
            self.state = TimerState::Stopped { elapsed: 0.0 };
        }
    }
}