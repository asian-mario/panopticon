#[cfg(feature = "bevy")]
use bevy::prelude::*;
use crate::core::{
    types::{CountryTag, FocusId, ProvinceId, DivisionId},
    time::Tick,
    effects::EffectRegistry,
    components::*,
};

#[cfg(not(feature = "bevy"))]
pub trait Event {}

// Focus system events
#[derive(Clone, Debug)]
pub struct FocusStarted {
    pub country: CountryTag,
    pub focus: FocusId,
}
impl Event for FocusStarted {}

#[derive(Clone, Debug)]
pub struct FocusCompleted {
    pub country: CountryTag,
    pub focus: FocusId,
}
impl Event for FocusCompleted {}

// Research events
#[derive(Clone, Debug)]
pub struct ResearchStarted {
    pub country: CountryTag,
    pub tech_id: String,
}
impl Event for ResearchStarted {}

#[derive(Clone, Debug)]
pub struct ResearchCompleted {
    pub country: CountryTag,
    pub tech_id: String,
}
impl Event for ResearchCompleted {}

// Movement events
#[derive(Clone, Debug)]
pub struct DivisionMoved {
    pub division_id: DivisionId,
    pub from: ProvinceId,
    pub to: ProvinceId,
}
impl Event for DivisionMoved {}

#[derive(Clone, Debug)]
pub struct DivisionArrived {
    pub division_id: DivisionId,
    pub province: ProvinceId,
}
impl Event for DivisionArrived {}

#[cfg(feature = "bevy")]
pub struct SimulationPlugin;

#[cfg(feature = "bevy")]
impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<FocusStarted>()
            .add_event::<FocusCompleted>()
            .add_event::<ResearchStarted>()
            .add_event::<ResearchCompleted>()
            .add_event::<DivisionMoved>()
            .add_event::<DivisionArrived>()
            .add_systems(Update, (
                process_focuses,
                process_research,
                process_movement,
            ));
    }
}

pub fn process_focuses(
    mut commands: Commands,
    mut focus_query: Query<(Entity, &CountryTag, &mut FocusProgress)>,
    mut ev_completed: EventWriter<FocusCompleted>,
    mut ev_tick: EventReader<Tick>,
    _registry: Res<EffectRegistry>,
) {
    for _tick in ev_tick.iter() {
        for (entity, country_tag, mut focus) in focus_query.iter_mut() {
            if focus.days_remaining > 0 {
                focus.days_remaining -= 1;
            }

            if focus.days_remaining == 0 {
                let completed = FocusCompleted {
                    country: *country_tag,
                    focus: focus.focus_id.clone(),
                };
                ev_completed.send(completed);
                commands.entity(entity).remove::<FocusProgress>();
            }
        }
    }
}

pub fn process_research(
    mut commands: Commands,
    mut research_query: Query<(Entity, &CountryTag, &mut ResearchProgress)>,
    mut ev_completed: EventWriter<ResearchCompleted>,
    mut ev_tick: EventReader<Tick>,
    _registry: Res<EffectRegistry>,
) {
    // Only process on tick events
    for _tick in ev_tick.iter() {
        for (entity, tag, mut research) in research_query.iter_mut() {
            if research.days_remaining > 0 {
                research.days_remaining -= 1;
            }

            if research.days_remaining == 0 {
                let completed = ResearchCompleted {
                    country: *tag,
                    tech_id: research.tech_id.clone(),
                };
                ev_completed.send(completed);
                commands.entity(entity).remove::<ResearchProgress>();
            }
        }
    }
}

pub fn process_movement(
    mut division_query: Query<(&DivisionComponent, &mut DivisionMovement)>,
    mut ev_arrived: EventWriter<DivisionArrived>,
    mut ev_tick: EventReader<Tick>,
) {
    for _tick in ev_tick.iter() {
        for (division, mut movement) in division_query.iter_mut() {
            if movement.days_left > 0 {
                movement.days_left -= 1;
            }

            if movement.days_left == 0 {
                let arrived = DivisionArrived {
                    division_id: division.id,
                    province: movement.to,
                };
                ev_arrived.send(arrived);
            }
        }
    }
}