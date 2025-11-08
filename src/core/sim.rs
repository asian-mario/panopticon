use serde::{Deserialize, Serialize};
use std::collections::{HashSet, HashMap};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunningFocus {
    pub id: String,
    pub remaining_days: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CountryFocusState {
    pub in_progress: Option<RunningFocus>,
    pub completed: HashSet<String>,
}

impl CountryFocusState {
    pub fn start_focus(&mut self, id: String, days: u32) -> Result<(), String> {
        if self.in_progress.is_some() { return Err("another focus already in progress".into()); }
        if self.completed.contains(&id) { return Err("focus already completed".into()); }
        self.in_progress = Some(RunningFocus { id, remaining_days: days });
        Ok(())
    }

    /// Advance focus by one day (tick). Returns Some(id) if a focus completed.
    pub fn tick(&mut self) -> Option<String> {
        if let Some(ref mut cur) = self.in_progress {
            if cur.remaining_days > 0 { cur.remaining_days = cur.remaining_days.saturating_sub(1); }
            if cur.remaining_days == 0 {
                let id = cur.id.clone();
                self.completed.insert(id.clone());
                self.in_progress = None;
                return Some(id);
            }
        }
        None
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RunningResearch {
    pub id: String,
    pub remaining_days: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CountryResearchState {
    pub in_progress: Option<RunningResearch>,
    pub completed: HashSet<String>,
}

impl CountryResearchState {
    pub fn start_research(&mut self, id: String, days: u32) -> Result<(), String> {
        if self.in_progress.is_some() { return Err("another research already in progress".into()); }
        if self.completed.contains(&id) { return Err("research already completed".into()); }
        self.in_progress = Some(RunningResearch { id, remaining_days: days });
        Ok(())
    }

    pub fn tick(&mut self) -> Option<String> {
        if let Some(ref mut cur) = self.in_progress {
            if cur.remaining_days > 0 { cur.remaining_days = cur.remaining_days.saturating_sub(1); }
            if cur.remaining_days == 0 {
                let id = cur.id.clone();
                self.completed.insert(id.clone());
                self.in_progress = None;
                return Some(id);
            }
        }
        None
    }
}

/// Minimal division movement state used by the sim core
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Moving {
    pub to: u32,
    pub days_left: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DivisionState {
    pub id: u64,
    pub location: u32,
    pub moving: Option<Moving>,
}

impl DivisionState {
    pub fn tick(&mut self) -> bool {
        if let Some(ref mut m) = self.moving {
            if m.days_left > 0 { m.days_left = m.days_left.saturating_sub(1); }
            if m.days_left == 0 {
                self.location = m.to;
                self.moving = None;
                return true; // arrived
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn focus_progression_happy_path() {
        let mut s = CountryFocusState::default();
        s.start_focus("revive_industry".to_string(), 3).unwrap();
        assert!(s.in_progress.is_some());
        assert_eq!(s.tick(), None);
        assert_eq!(s.tick(), None);
        assert_eq!(s.tick(), Some("revive_industry".to_string()));
        assert!(s.in_progress.is_none());
        assert!(s.completed.contains("revive_industry"));
    }

    #[test]
    fn research_progression_happy_path() {
        let mut s = CountryResearchState::default();
        s.start_research("basic_infantry_weapons".to_string(), 2).unwrap();
        assert_eq!(s.tick(), None);
        assert_eq!(s.tick(), Some("basic_infantry_weapons".to_string()));
    }

    #[test]
    fn division_movement_arrival() {
        let mut d = DivisionState { id: 1, location: 0, moving: Some(Moving{ to: 2, days_left: 2 }) };
        assert!(!d.tick());
        assert_eq!(d.moving.as_ref().unwrap().days_left, 1);
        assert!(d.tick());
        assert_eq!(d.location, 2);
        assert!(d.moving.is_none());
    }
}
