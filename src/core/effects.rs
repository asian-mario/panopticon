use bevy::prelude::*;
use std::collections::HashMap;
use anyhow::Result;
use serde_json::Value;
use crate::core::types::CountryTag;

pub type EffectFn = fn(&mut World, CountryTag, &Value) -> Result<()>;

#[derive(Resource, Default)]
pub struct EffectRegistry {
    effects: HashMap<String, EffectFn>
}

impl EffectRegistry {
    pub fn register(&mut self, name: &str, effect: EffectFn) {
        self.effects.insert(name.to_string(), effect);
    }

    pub fn execute(&self, world: &mut World, country: CountryTag, effect_type: &str, params: &Value) -> Result<()> {
        if let Some(effect) = self.effects.get(effect_type) {
            effect(world, country, params)
        } else {
            Err(anyhow::anyhow!("Unknown effect type: {}", effect_type))
        }
    }
}

pub fn setup_effect_registry(mut commands: Commands) {
    let mut registry = EffectRegistry::default();
    
            // Register core effects
            registry.register("add_civ_factories", |_world, _country, params| {
                let _amount = params["amount"].as_i64().unwrap_or(0);
                // TODO: Implement effect
                Ok(())
            });

            registry.register("add_mil_factories", |_world, _country, params| {
                let _amount = params["amount"].as_i64().unwrap_or(0);
                // TODO: Implement effect  
                Ok(())
            });

            registry.register("add_pp", |_world, _country, params| {
                let _amount = params["amount"].as_i64().unwrap_or(0);
                // TODO: Implement effect
                Ok(())
            });

            registry.register("unlock_battalion", |_world, _country, params| {
                let _battalion = params["battalion"].as_str().unwrap_or("");
                // TODO: Implement effect
                Ok(())
            });

            registry.register("unit_stat_mod", |_world, _country, params| {
                let _unit = params["unit"].as_str().unwrap_or("");
                // TODO: Implement effect with stat modifications
                Ok(())
            });    commands.insert_resource(registry);
}