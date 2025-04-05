use std::{collections::{HashMap, VecDeque}, fmt::Debug};

use chronobase::{EntityID, database_connection::Typebase};
use rand::seq::IteratorRandom;
use rand_chacha::ChaCha8Rng;

use crate::{entities::{BaseCharacter, Character, BaseTeam, Team}, events::Event};


pub trait CharacterDatabase: Typebase<BaseCharacter> + Typebase<BaseTeam> {}
impl<T: Typebase<BaseCharacter> + Typebase<BaseTeam>> CharacterDatabase for T {}
impl Debug for Box<dyn CharacterDatabase> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "Character Database".fmt(f)
    }
}

pub struct Game<'a> {
    database: &'a dyn CharacterDatabase,
    rng: &'a mut ChaCha8Rng,

    pub turn_order: VecDeque<EntityID<Team>>,

    pub characters: HashMap<EntityID<Character>, Character>,

    pub teams: HashMap<EntityID<Team>, Team>,
    pub incarnations: HashMap<EntityID<Team>, EntityID<Character>>,
    pub rosters: HashMap<EntityID<Team>, Vec<EntityID<Character>>>,
}
impl<'a> Game<'a> {
    pub fn new(database:&'a dyn CharacterDatabase, rng: &'a mut ChaCha8Rng, delve_team: EntityID<BaseTeam>, defender_team: EntityID<BaseTeam>) -> Self {
        let mut result = Game {
            database,
            rng,
            characters: HashMap::new(),
            rosters: HashMap::new(),
            turn_order: VecDeque::new(),
            incarnations: HashMap::new(),

            teams: HashMap::new(),
        };
        result.add_team(delve_team);
        result.add_team(defender_team);

        result
    }
    pub fn turn(&mut self) {
        let attacking_team = *self.turn_order.get(0).expect("There is always a team");
        let defending_team = 
            *(1..self.turn_order.len()).choose(&mut self.rng)
                .map(|index| self.turn_order.get(index))
                .flatten()
                .expect("There is always a second team");
        self.turn_order.rotate_right(1);

        let incarnation = self.incarnations.get(&attacking_team).unwrap().get(self);
        self.complete_events(incarnation.modifiers().flat_map(|modifier| modifier.start_turn(incarnation)).collect());

        let attacker = *self.rosters.get(&attacking_team).unwrap().get(0).expect("Roster should not be empty");
        let defender = *self.rosters.get(&defending_team).unwrap().get(0).expect("Roster should not be empty");
    
        let event = Event::Damage { source: attacker, target: defender, amount: 2 };
        self.complete_events(vec![event]);
    }
    fn complete_events(&mut self, events:Vec<Event>) {
        for event in events {
            println!("{}", event.complete(self).display(self));
        }
    }
    pub fn add_team(&mut self, id:EntityID<BaseTeam>) -> EntityID<Team> {
        let base = self.database.load_latest(id, None).unwrap();
        let incarnation = base.incarnation_id();
        let team = Team::new(base);
        let id = team.id();

        self.turn_order.push_front(id);
        self.rosters.insert(id, Vec::new());
        self.teams.insert(id, team);

        let incarnation = self.add_character(incarnation, id);
        self.incarnations.insert(id, incarnation);
        id
    }
    pub fn add_character(&mut self, id:EntityID<BaseCharacter>, team:EntityID<Team>) -> EntityID<Character> {
        let base = self.database.load_latest(id, None).unwrap();
        let character = Character::new(&base, team);
        let id = character.id();
        self.characters.insert(id, character);
        self.rosters.get_mut(&team).unwrap().push(id);

        let character = id.get(self);
        let events = character.modifiers().flat_map(|modifier| modifier.on_enter(character)).collect::<Vec<_>>();
        self.complete_events(events);
        id
    }
    pub fn display(&self, delve_team: EntityID<Team>, defender_team: EntityID<Team>) {
        for team in [delve_team, defender_team] {
            println!("{}", team.get(self).name());
            for character in self.rosters.get(&team).unwrap() {
                println!("{}: {}hp", character.get(self).name(), character.get(self).damage_taken())
            }
        }
    }
}

pub trait LoadableID {
    type Entity;
    fn get<'a>(self, game: &'a Game) -> &'a Self::Entity;
    fn get_mut<'a>(self, game: &'a mut Game) -> &'a mut Self::Entity;
}
impl<'a> Game<'a> {
    pub fn get<T, ID:LoadableID<Entity = T>>(&'a self, id:ID) -> &'a T {
        id.get(self)
    }
    pub fn get_mut<T, ID:LoadableID<Entity = T>>(&'a mut self, id:ID) -> &'a mut T {
        id.get_mut(self)
    }
}

impl LoadableID for EntityID<Character> {
    type Entity = Character;
    fn get<'a>(self, game: &'a Game) -> &'a Character {
        game.characters.get(&self).expect("Ids should be valid")
    }
    fn get_mut<'a>(self, game: &'a mut Game) -> &'a mut Character {
        game.characters.get_mut(&self).expect("Ids should be valid")
    }
}
impl LoadableID for EntityID<Team> {
    type Entity = Team;
    fn get<'a>(self, game: &'a Game) -> &'a Self::Entity {
        game.teams.get(&self).expect("Ids should be valid")
    }
    fn get_mut<'a>(self, game: &'a mut Game) -> &'a mut Self::Entity {
        game.teams.get_mut(&self).expect("Ids should be valid")
    }
}