use anyhow::Result;

#[cfg(feature = "bevy")]
use bevy::prelude::*;

#[cfg(feature = "bevy")]
use panopticon::{
    core::{
        time::{tick_system, Clock, Tick},
        effects::setup_effect_registry,
        simulation::SimulationPlugin,
    },
    engine::EnginePlugin,
};

#[cfg(feature = "bevy")]
fn main() -> Result<()> {
    // Minimal Bevy app: inserts Clock resource and Tick event
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Panopticon".into(),
                resolution: (1280., 720.).into(),
                present_mode: bevy::window::PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_plugins((EnginePlugin, SimulationPlugin))
        .insert_resource(Clock { current_day: 0, paused: false, speed_idx: 0, acc: 0.0 })
        .add_event::<Tick>()
        .add_systems(Startup, setup_effect_registry)
        .add_systems(Update, tick_system)
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
    for _ in ev.iter() {
        info!("Tick: day={}", clock.current_day);
    }
}
