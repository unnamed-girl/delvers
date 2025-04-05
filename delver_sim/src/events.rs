use chronobase::EntityID;

use crate::{entities::{BaseCharacter, Character, Team}, game::{Game, LoadableID}};

impl<'a> Game<'a> {
    fn alter(&mut self) {

    }
    fn get_responses(&self, _event: &Event) -> Vec<Event> {
        vec![]
    }
}
#[derive(Debug, Clone)]
pub enum Event {
    Summon {
        summonees:Vec<EntityID<BaseCharacter>>,
        team:EntityID<Team>
    },
    Damage {
        source: EntityID<Character>,
        target: EntityID<Character>,
        amount: i32
    },
    SwitchTeam {
        target: EntityID<Character>,
        destination: Option<EntityID<Team>>
    },
    RotateRoster {
        amount: usize,
        team: EntityID<Team>
    }
}
impl Event {
    pub fn complete(self, world: &mut Game) -> CompletedEvent {
        self.execute(world).complete(world)
    }
    fn execute(self, world: &mut Game) -> ExecutedEvent {
        match &self {
            Event::Summon { summonees, team } => {
                for character in summonees {
                    world.add_character(*character, *team);
                }
            }
            Event::Damage { source: _, target, amount } => {
                target.get_mut(world).alter_health(-*amount);
            },
            Event::SwitchTeam { target, destination } => {
                // if let Some(current_team) = target.get(world).team() {
                let current_team = target.get(world).team();
                    let roster = world.rosters.get_mut(&current_team).unwrap();
                    let index = roster.iter().position(|id| id == target).unwrap();
                    roster.remove(index);
                // }

                if let Some(destination) = destination {
                    world.rosters.get_mut(destination).unwrap().push(*target);
                    *target.get_mut(world).team_mut() = *destination;
                }
            }
            Event::RotateRoster { amount, team } => {
                let roster = world.rosters.get_mut(&team).unwrap();
                roster.rotate_right(*amount);
            }
        }
        ExecutedEvent(self)
    }
}

#[derive(Debug, Clone)]
struct ExecutedEvent(Event);
impl ExecutedEvent {
    fn complete(self, world: &mut Game) -> CompletedEvent {
        let mut responses = Vec::new();

        for event in world.get_responses(&self.0) {
            let event = event.execute(world);
            let event = event.complete(world);
            responses.push(event);
        }

        CompletedEvent { event_type: self.0, responses }
    }
}

#[derive(Debug, Clone)]
pub struct CompletedEvent {
    event_type: Event,
    responses: Vec<CompletedEvent>
}
impl CompletedEvent {
    pub fn display(&self, world: &Game) -> String {
        match &self.event_type {
            Event::Damage { source, target, amount } => {
                format!("{} deals {} to {}", source.get(world).name(), amount, target.get(world).name())
            }
            _ => format!("{:?}", &self.event_type)
        }
    }
}