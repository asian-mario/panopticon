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

#[cfg_attr(feature = "bevy", derive(Event))]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Tick;

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

