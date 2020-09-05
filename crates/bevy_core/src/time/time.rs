use bevy_ecs::ResMut;
use std::time::{Duration, Instant};

/// Tracks elapsed time since the last update and since the App has started
pub struct Time {
    pub delta: Duration,
    pub instant: Option<Instant>,
    pub startup: Instant,
    pub delta_seconds_f64: f64,
    pub delta_seconds: f32,
    pub frame_count: u32,
    pub frame_count_u64: u64,
    pub seconds_since_startup: f64,
}

impl Default for Time {
    fn default() -> Time {
        Time {
            delta: Duration::from_secs(0),
            instant: None,
            startup: Instant::now(),
            delta_seconds_f64: 0.0,
            delta_seconds: 0.0,
            frame_count: 0,
            frame_count_u64: 0,
            seconds_since_startup: 0.0,
        }
    }
}

impl Time {
    pub fn update(&mut self) {
        self.frame_count_u64 += 1;
        self.frame_count = self.frame_count_u64 as u32;

        let now = Instant::now();
        if let Some(instant) = self.instant {
            self.delta = now - instant;
            self.delta_seconds_f64 = self.delta.as_secs_f64();
            self.delta_seconds = self.delta.as_secs_f32();
        }

        let duration_since_startup = now - self.startup;
        self.seconds_since_startup = duration_since_startup.as_secs_f64();
        self.instant = Some(now);
    }

    pub fn time_since_startup(&self) -> Duration {
        Instant::now() - self.startup
    }
}

pub(crate) fn time_system(mut time: ResMut<Time>) {
    time.update();
}
