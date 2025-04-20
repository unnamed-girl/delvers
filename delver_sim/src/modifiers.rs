use chronobase::EntityID;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{entities::Character, events::{Event, ExecutedEvent}, game::ActiveCharacter, progress_bars::{xp_bar, GameProgressBarLocation, ProgressBarName}};

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct Modifier(pub ModifierID, pub ModifierType);
impl Modifier {
    pub fn new(character: EntityID<Character>, type_: ModifierType) -> Self {
        Self(ModifierID::roll(character), type_)
    }
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub enum ModifierType {
    Grinder,
    Resilient
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub struct ModifierID(pub EntityID<Character>, pub Uuid);
impl ModifierID {
    pub fn roll(character: EntityID<Character>) -> Self {
        Self(character, Uuid::new_v4())
    }
}

impl Modifier {
    pub fn on_enter(&self, character:&ActiveCharacter) -> Vec<Event> {
        let mut events = Vec::new();
        match self.1 {
            ModifierType::Grinder => {
                events.push(Event::CreateProgressBar { location: GameProgressBarLocation::Character(character.id, ProgressBarName::XP), bar: xp_bar() })
            }
            _ => ()
        }
        events
    }
    pub fn start_turn(&self, character:&ActiveCharacter) -> Vec<Event> {
        let mut events = Vec::new();
        match self.1 {
            _ => ()
        }
        events
    }
    pub fn pre_event(&self, character:&ActiveCharacter, event:&mut Event) -> Vec<Event> {
        let mut events = Vec::new();
        match self.1 {
            ModifierType::Resilient => if let Event::ProgressProgressBar { location: GameProgressBarLocation::Character(target, ProgressBarName::HP), amount } = event {
                if *target == character.id && *amount > 1 {
                    *amount -= 1;
                    events.push(Event::Say(format!("{target:?}'s resilience reduces the damage they take")));
                }
            }
            _ => ()
        }
        events
    }
    pub fn post_event(&self, character:&ActiveCharacter, event:&ExecutedEvent) -> Vec<Event> {
        let mut events = Vec::new();
        match self.1 {
            ModifierType::Grinder => if let Event::Attack {target, attacker } = event.0 {
                if target == character.id || attacker == character.id {
                    events.push(Event::ProgressProgressBar { location: GameProgressBarLocation::Character(character.id, ProgressBarName::XP), amount: 1 })
                }
            }
            _ => ()
        }
        events
    }
}