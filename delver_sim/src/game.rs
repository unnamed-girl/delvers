use std::{collections::{HashMap, VecDeque}, ops::{Index, IndexMut}};

use chronobase::{EntityID, SavableEntity};
use colored::Colorize;
use rand::seq::{IteratorRandom, SliceRandom};
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{database::{ActiveCharacterManager, DatabaseManager}, delver_display::ToDisplayConstruct, entities::{Character, Team}, events::{CompletedEvent, Event}, progress_bars::{health_bar, GameProgressBarLocation, ProgressBar, ProgressBarName}};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub struct ActiveCharacterID(Uuid);
impl ActiveCharacterID {
    pub fn roll() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct ActiveCharacter {
    pub id: ActiveCharacterID,
    pub character: EntityID<Character>,
    pub progress_bars: HashMap<ProgressBarName, ProgressBar>
}
impl ActiveCharacter {
    pub fn new(character: EntityID<Character>) -> Self {
        Self { id: ActiveCharacterID::roll(), character, progress_bars: HashMap::default() }
    }
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Roster {
    pub characters: [Option<ActiveCharacterID>; 6],
    pub previous_turn: Position
}
impl Roster {
    pub fn new() -> Self {
        Self {
            characters: [None; 6],
            previous_turn: Position::Bottom
        }
    }
    pub fn get(&self, position: Position) -> Option<ActiveCharacterID> {
        self.characters[position]
    }

    pub fn next_filled(&mut self) -> Option<ActiveCharacterID> {
        for position in self.previous_turn.single_canonical_cycle() {
            if let Some(character) = self.get(position) {
                self.previous_turn = position;
                return Some(character)
            }
        }
        None
    }

    pub fn random_filled(&mut self, rng: &mut ChaCha8Rng) -> Option<ActiveCharacterID> {
        let mut order = Position::enter_order();
        order.shuffle(rng);
        order.into_iter().flat_map(|p| self.get(p))
            .next()
    }

    /// Adds the given character to the next available roster slot according to the canonical order
    pub fn add_character(&mut self, character: ActiveCharacterID) -> Option<Position> {
        for i in Position::enter_order() {
            if self.characters[i].is_none() {
                self.characters[i] = Some(character);
                return Some(i)
            }
        }
        None
    }
}
impl Default for Roster {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Position {
    Top, // (increase your mass??)
    Charm,
    Up,
    Down,
    Strange,
    Bottom, // (Increase your mass ?)
}
impl Position {
    pub fn canonical_order() -> [Position; 6] {
        [Position::Top, Position::Charm, Position::Up, Position::Down, Position::Strange, Position::Bottom]
    }
    pub fn enter_order() -> [Position; 6] {
        [Position::Top, Position::Up, Position::Down, Position::Bottom, Position::Strange, Position::Charm]
    }
    pub fn single_canonical_cycle(self) -> impl Iterator<Item = Position> {
        IterPosition(self).take(6)
    }
}

struct IterPosition(Position);
impl Iterator for IterPosition {
    type Item = Position;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.0 = match self.0 {
            Position::Top => Position::Charm,
            Position::Charm => Position::Up,
            Position::Up => Position::Down,
            Position::Down => Position::Strange,
            Position::Strange => Position::Bottom,
            Position::Bottom => Position::Top,
        };
        Some(self.0)
    }
}
impl<T> Index<Position> for [T] where [T]: Index<usize> {
    type Output = <[T] as Index<usize>>::Output;
    fn index(&self, index: Position) -> &Self::Output {
        self.index(index.into())
    }
}
impl<T> IndexMut<Position> for [T] where [T]: IndexMut<usize> {
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        self.index_mut(index.into())
    }
}
impl From<Position> for usize {
    fn from(value: Position) -> usize {
        match value {
            Position::Top=> 2,
            Position::Charm=> 4,
            Position::Up=> 0,
            Position::Down=> 1,
            Position::Strange=> 5,
            Position::Bottom=> 3,
        }
    }
}
impl TryFrom<usize> for Position {
    type Error = ();
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            2 => Ok(Self::Top),
            4 => Ok(Self::Charm),
            0 => Ok(Self::Up),
            1 => Ok(Self::Down),
            5 => Ok(Self::Strange),
            3 => Ok(Self::Bottom),
            _ => Err(())
        }
    }
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Game {
    pub game_id: EntityID<Game>,
    pub active_characters: ActiveCharacterManager,
    pub turn_order: VecDeque<EntityID<Team>>,
    pub rosters: HashMap<EntityID<Team>, Roster>,

    pub latest_events: Vec<CompletedEvent>
}
impl Game {
    pub fn insert_progress_bar(&mut self, location: GameProgressBarLocation, bar: ProgressBar) -> Option<ProgressBar> {
        match location {
            GameProgressBarLocation::Character(character, name) => {
                let character = self.active_characters.get_mut(character);
                character.progress_bars.insert(name, bar)
            }
        }
    }
    pub fn get_progress_bar(&self, location: GameProgressBarLocation) -> &ProgressBar {
        match location {
            GameProgressBarLocation::Character(character, name) => {
                let character = self.active_characters.get(character);
                character.progress_bars.get(&name).unwrap()
            }
        }
    }
    pub fn get_progress_bar_mut(&mut self, location: GameProgressBarLocation) -> &mut ProgressBar {
        match location {
            GameProgressBarLocation::Character(character, name) => {
                let character = self.active_characters.get_mut(character);
                character.progress_bars.get_mut(&name).unwrap()
            }
        }
    }
}

pub struct Sim<'a> {
    pub database: DatabaseManager,
    pub rng: &'a mut ChaCha8Rng,

    pub world: Game
}
impl<'a> Sim<'a> {
    pub fn new(database: DatabaseManager, rng: &'a mut ChaCha8Rng, delve_team: EntityID<Team>, defender_team: EntityID<Team>) -> Self {
        let mut result = Sim {
            database,
            rng,
            world: Game::default(),
        };
        result.add_team(delve_team);
        result.add_team(defender_team);

        result
    }
    pub fn turn(&mut self) {
        self.world.latest_events.clear();

        let attacking_team = *self.world.turn_order.front().expect("There is always a team");
        let defending_team = 
            *(1..self.world.turn_order.len()).choose(&mut self.rng)
                .and_then(|index| self.world.turn_order.get(index))
                .expect("There is always a second team");
        self.world.turn_order.rotate_right(1);

        let attacker = self.world.rosters.get_mut(&attacking_team).unwrap().next_filled().expect("Roster should not be empty");
        let defender = self.world.rosters.get_mut(&defending_team).unwrap().random_filled(self.rng).expect("Roster should not be empty");
        
        let attacking_character = self.database.load(self.world.active_characters.get(attacker).character);
        let events = attacking_character.modifiers().flat_map(|modifier| modifier.start_turn(self.world.active_characters.get(attacker))).collect::<Vec<_>>();
        self.complete_events(events);

        let event = Event::Attack { attacker, target: defender };
        self.complete_events(vec![event]);

        self.database.save(self.world.clone());
    }
    fn complete_events(&mut self, events:Vec<Event>) {
        for event in events {
            let completed_event = event.complete(self);
            println!("{}", completed_event.longform(&self.world, &self.database));
            self.world.latest_events.push(completed_event);
        }
    }
    pub fn add_team(&mut self, team:EntityID<Team>) -> EntityID<Team> {
        let team = self.database.load(team);
        self.world.turn_order.push_front(team.id);
        self.world.rosters.insert(team.id, Roster::new());

        team.roster.into_iter().take(6).for_each(|c| { self.add_character(c, team.id); });

        team.id
    }
    pub fn add_character(&mut self, id:EntityID<Character>, team:EntityID<Team>) -> ActiveCharacterID {
        let active_character = ActiveCharacter::new(id);
        let active_id = active_character.id;
        self.world.rosters.get_mut(&team).unwrap().add_character(active_character.id).unwrap();
        self.world.active_characters.add_active_character(active_character);

        let character = self.database.load(id);
        self.complete_events(vec![Event::CreateProgressBar { location: GameProgressBarLocation::Character(active_id, ProgressBarName::HP), bar: health_bar() }]);
        let events = character.modifiers().flat_map(|modifier| modifier.on_enter(self.world.active_characters.get(active_id))).collect::<Vec<_>>();
        self.complete_events(events);
        active_id
    }
    pub fn display(&self, delve_team: EntityID<Team>, defender_team: EntityID<Team>) {
        for team in [delve_team, defender_team] {
            let team = self.database.load(team);
            println!("{}", team.name.color(team.colour));
            let roster = self.world.rosters.get(&team.id).unwrap();
            for i in Position::canonical_order() {
                if let Some(character) = roster.characters[i] {
                    let character = self.world.active_characters.get(character);
                    println!("{}", character.longform(&self.world, &self.database));
                }
            }
        }
    }
}

impl SavableEntity for Game {
    const TABLE_NAME: &'static str = "game";
}