// Core simulation primitives with optional Bevy integration
#[cfg(feature = "bevy")]
use bevy::prelude::*;

use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "bevy", derive(Resource))]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Clock {
    pub current_day: u32,
    pub paused: bool,
    pub speed_idx: usize,
    pub acc: f32,
}

pub const SPEEDS: [f32; 5] = [0.5, 0.35, 0.25, 0.18, 0.12];

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Tick;
impl Event for Tick {}

#[cfg(feature = "bevy")]
pub fn tick_system(time: Res<Time>, mut clock: ResMut<Clock>, mut ev: EventWriter<Tick>) {
    if clock.paused {
        return;
    }
    clock.acc += time.delta_seconds();
    let spd = SPEEDS[clock.speed_idx.min(4)];
    while clock.acc >= spd {
        clock.acc -= spd;
        clock.current_day = clock.current_day.wrapping_add(1);
        ev.send(Tick);
    }
}

/// Advance the clock by delta seconds and return how many ticks occurred.
/// This is the non-Bevy testable core of `tick_system`.
pub fn advance_clock(clock: &mut Clock, delta_seconds: f32) -> u32 {
    if clock.paused {
        return 0;
    }
    clock.acc += delta_seconds;
    let spd = SPEEDS[clock.speed_idx.min(4)];
    let mut ticks = 0u32;
    while clock.acc >= spd {
        clock.acc -= spd;
        clock.current_day = clock.current_day.wrapping_add(1);
        ticks += 1;
    }
    ticks
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn advance_clock_paused() {
        let mut c = Clock { current_day: 0, paused: true, speed_idx: 0, acc: 0.0 };
        let t = advance_clock(&mut c, 1.0);
        assert_eq!(t, 0);
        assert_eq!(c.current_day, 0);
        assert_eq!(c.acc, 0.0);
    }

    #[test]
    fn advance_clock_partial_accumulation() {
        let mut c = Clock { current_day: 0, paused: false, speed_idx: 0, acc: 0.0 };
        let t = advance_clock(&mut c, 0.2);
        assert_eq!(t, 0);
        assert!(c.acc > 0.19 && c.acc < 0.21);
    }

    #[test]
    fn advance_clock_multiple_ticks() {
        let mut c = Clock { current_day: 10, paused: false, speed_idx: 0, acc: 0.0 };
        // speed 0 => 0.5s per day, so 1.5s => 3 ticks
        let t = advance_clock(&mut c, 1.5);
        assert_eq!(t, 3);
        assert_eq!(c.current_day, 13);
        assert!(c.acc < 0.5);
    }

    #[test]
    fn advance_clock_speed_index() {
        let mut c = Clock { current_day: 0, paused: false, speed_idx: 2, acc: 0.0 };
        // speed_idx 2 -> 0.25s per day
        let t = advance_clock(&mut c, 0.5);
        assert_eq!(t, 2);
        assert_eq!(c.current_day, 2);
    }
}

