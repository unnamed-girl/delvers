use chronobase::EntityID;
use serde::{Deserialize, Serialize};

use crate::{entities::{BaseCharacter, Character}, events::Event};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum BaseModifier {
    Team(Vec<EntityID<BaseCharacter>>),
    Dungeon(Vec<EntityID<BaseCharacter>>)
}

impl BaseModifier {
    pub fn on_enter(&self, character:&Character) -> Vec<Event> {
        match self {
            BaseModifier::Dungeon(roster) | BaseModifier::Team(roster) => {
                vec![Event::Summon { summonees: roster.clone(), team: character.team() }, Event::SwitchTeam { target: character.id(), destination: None }]
            }
        }
    }
    pub fn start_turn(&self, character:&Character) -> Vec<Event> {
        match self {
            BaseModifier::Team(_) => {
                vec![Event::RotateRoster { amount: 1, team: character.team() }]
            }
            _ => vec![]
        }
    }
    pub fn event_react(&self, _event:&Event) -> Vec<Event> {
        match self {
            _ => vec![]
        }
    }
}