
use serde::{Deserialize, Serialize};

use crate::{game::{ActiveCharacterID, Sim}, progress_bars::{GameProgressBarLocation, ProgressBar, ProgressBarName}};

impl Sim<'_> {
    fn get_pre_responses(&self, event: &mut Event) -> Vec<Event> {
        self.world.active_characters.0.values()
            .flat_map(|active_character| {
                let character = self.database.load(active_character.character);
                character.modifiers.iter().flat_map(|modifier| modifier.pre_event(active_character, event))
                    .collect::<Vec<_>>()
            })
            .collect()
    }
    fn get_post_responses(&self, event: &ExecutedEvent) -> Vec<Event> {
        self.world.active_characters.0.values()
            .flat_map(|active_character| {
                let character = self.database.load(active_character.character);
                character.modifiers.iter().flat_map(|modifier| modifier.post_event(active_character, event))
                    .collect::<Vec<_>>()
            })
            .collect()
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Event {
    Attack {
        attacker: ActiveCharacterID,
        target: ActiveCharacterID
    },
    CreateProgressBar {
        location: GameProgressBarLocation,
        bar: ProgressBar
    },
    ProgressProgressBar {
        location: GameProgressBarLocation,
        amount: u16
    },
    Say(String)
}
impl Event {
    fn execute(self, sim: &mut Sim) -> (ExecutedEvent, Vec<Event>) {
        let mut events = Vec::new();
        match &self {
            Event::Attack { target, .. } => {
                events.push(Event::ProgressProgressBar { location: GameProgressBarLocation::Character(*target, ProgressBarName::HP), amount: 2 });
            }
            Event::CreateProgressBar { location, bar } => {
               sim.world.insert_progress_bar(*location, *bar);
            }
            Event::ProgressProgressBar { location, amount } => {
                let bar =sim.world.get_progress_bar_mut(*location);
                bar.increment(*amount);
            }
            Event::Say(_) => ()
        }
        (ExecutedEvent(self), events)
    }
}


impl Event {
    pub fn complete(mut self, sim: &mut Sim) -> CompletedEvent {
        let pre_responses = sim.get_pre_responses(&mut self)
            .into_iter()
            .map(|event| event.complete(sim))
            .collect();
        
        let (event, outcomes) = self.execute(sim);

        let outcomes = outcomes.into_iter().map(|event| event.complete(sim)).collect();

        let post_responses = sim.get_post_responses(&event)
            .into_iter()
            .map(|event| event.complete(sim))
            .collect();

        CompletedEvent { event, pre_responses, outcomes, post_responses }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutedEvent(pub Event);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletedEvent {
    pub event: ExecutedEvent,
    pub pre_responses: Vec<CompletedEvent>,
    pub outcomes: Vec<CompletedEvent>,
    pub post_responses: Vec<CompletedEvent>
}