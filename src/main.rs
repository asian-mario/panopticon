use anyhow::Result;

#[cfg(feature = "bevy")]
use bevy::prelude::*;

#[cfg(feature = "bevy")]
use panopticon::{
    core::time::{tick_system, Clock, Tick},
    engine::EnginePlugin,
};

#[cfg(feature = "bevy")]
fn main() -> Result<()> {
    // Minimal Bevy app: inserts Clock resource and Tick event
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EnginePlugin)
        .insert_resource(Clock { current_day: 0, paused: false, speed_idx: 0, acc: 0.0 })
        .add_event::<Tick>()
        .add_systems(Update, (tick_system, print_tick))
        .run();

    // Ok(()) unreachable because Bevy takes over the main thread, but keep signature
    Ok(())
}

#[cfg(not(feature = "bevy"))]
fn main() -> Result<()> {
    println!("Bevy feature not enabled. To run the app with Bevy, enable the 'bevy' feature: \n  cargo run --features bevy");
    Ok(())
}

#[cfg(feature = "bevy")]
fn print_tick(mut ev: EventReader<Tick>, clock: Res<Clock>) {
    for _ in ev.read() {
        info!("Tick: day={}", clock.current_day);
    }
}
