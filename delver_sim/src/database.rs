use std::{collections::HashMap, fmt::Debug};

use chronobase::{EntityID, SavableEntity, Typebase};
use serde::{Deserialize, Serialize};

use crate::{entities::{Character, Team}, game::{ActiveCharacter, ActiveCharacterID, Game}};

impl<> Debug for DatabaseManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "Character Database".fmt(f)
    }
}
pub trait CharacterDatabase: Typebase<Character> + Typebase<Team> + Typebase<Game> + Send + Sync {}
impl<T: Typebase<Character> + Typebase<Team> + Typebase<Game> + Send + Sync> CharacterDatabase for T {}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct ActiveCharacterManager(pub HashMap<ActiveCharacterID, ActiveCharacter>);
impl ActiveCharacterManager {
    pub fn add_active_character(&mut self, character: ActiveCharacter) {
        self.0.insert(character.id, character);
    }
    pub fn get<ID, O>(&self, id: ID) -> &O where Self: Get<ID, O> {
        _Get::get(self, id).unwrap()
    }
    pub fn get_mut<ID, O>(&mut self, id: ID) -> &mut O where Self: GetMut<ID, O> {
        _GetMut::get_mut(self, id).unwrap()
    }
}

pub struct DatabaseManager(pub Box<dyn CharacterDatabase>);
impl DatabaseManager {
    pub fn new(database: Box<dyn CharacterDatabase>) -> Self {
        Self(database)
    }
    pub fn save<T>(&self, value: T) where Self: Save<T> {
        _Save::save(self, value).unwrap();
    }
    pub fn load<ID, O>(&self, id: ID) -> O where Self: Load<ID, O> {
        _Load::load(self, id).unwrap()
    }
}

#[allow(private_bounds)]
pub trait Save<T>: _Save<T> {}
impl<A, T> Save<T> for A where A: _Save<T> {}

#[allow(private_bounds)]
pub trait Load<ID, O>: _Load<ID, Output = O> {}
impl<T, ID, O> Load<ID, O> for T where T: _Load<ID, Output = O> {}

#[allow(private_bounds)]
pub trait Get<ID, O>: _Get<ID, Output = O> {}
impl<T, ID, O> Get<ID, O> for T where T: _Get<ID, Output = O> {}

#[allow(private_bounds)]
pub trait GetMut<ID, O>: _GetMut<ID, Output = O> {}
impl<T, ID, O> GetMut<ID, O> for T where T: _GetMut<ID, Output = O> {}

trait _Save<T> {
    fn save(&self, value: T) -> Option<()>;
}
trait _Load<ID> {
    type Output;
    fn load(&self, id: ID) -> Option<Self::Output>;
}

trait _Get<ID> {
    type Output;
    fn get(&self, id: ID) -> Option<&Self::Output>;
}
trait _GetMut<ID> {
    type Output;
    fn get_mut(&mut self, id: ID) -> Option<&mut Self::Output>;
}

impl<T: SavableEntity + GetIDHelpher> _Save<T> for DatabaseManager where dyn CharacterDatabase: Typebase<T>{
    fn save(&self, value: T) -> Option<()> {   
        self.0.save(value.get_id(), &value).ok()
    }
}
impl<T: SavableEntity> _Load<EntityID<T>> for DatabaseManager where dyn CharacterDatabase: Typebase<T> {
    type Output = T;
    fn load(&self, id: EntityID<T>) -> Option<T> {
        self.0.load_latest(id, None).ok()
    }
}

// impl<'a> _Load<ModifierID> for DatabaseManager {
//     type Output = Modifier;
//     fn load(&self, id: ModifierID) -> Option<Self::Output> {
//         self.0.load_latest(id.0, None).ok()?.modifiers
//             .iter().find(|m| m.0 == id)
//             .copied()
//     }
// }

// impl _Load<ActiveCharacterID> for DatabaseManager {
//     type Output = Character;
//     fn load(&self, id: ActiveCharacterID) -> Option<Self::Output> {
//         _Load::load(self, self.get(id).character)
//     }
// }

impl _Get<ActiveCharacterID> for ActiveCharacterManager {
    type Output = ActiveCharacter;
    fn get(&self, id: ActiveCharacterID) -> Option<&Self::Output> {
        self.0.get(&id)
    }
}
impl _GetMut<ActiveCharacterID> for ActiveCharacterManager {
    type Output = ActiveCharacter;
    fn get_mut(&mut self, id: ActiveCharacterID) -> Option<&mut Self::Output> {
        self.0.get_mut(&id)
    }
}

trait GetIDHelpher:Sized {
    fn get_id(&self) -> EntityID<Self>;
}
impl GetIDHelpher for Character {
    fn get_id(&self) -> EntityID<Self> {
        self.id
    }
}
impl GetIDHelpher for Team {
    fn get_id(&self) -> EntityID<Self> {
        self.id
    }
}
impl GetIDHelpher for Game {
    fn get_id(&self) -> EntityID<Self> {
        self.game_id
    }
}