use std::fmt::{Debug, Display};
use chronobase::{entities::PatchableEntity, ChronobasePatch, EntityID, SavableEntity};
use serde::{Deserialize, Serialize};

use crate::modifiers::BaseModifier;

#[derive(Debug, Deserialize)]
pub struct Character {
    id: EntityID<Character>,
    name: String,
    base_id: EntityID<BaseCharacter>,
    team: EntityID<Team>,
    stats: Stats,
    modifiers: Vec<BaseModifier>,
    damage_taken: i32,
}

impl Character {
    pub fn new(base:&BaseCharacter, team:EntityID<Team>) -> Self {
        let modifiers = base.modifiers().clone();
        Character {
            id: EntityID::roll(),
            name: base.name.clone(),
            base_id: base.id(),
            team,
            stats: base.stats().clone(),
            modifiers,
            damage_taken: 0
        }
    }
    pub fn team(&self) -> EntityID<Team> {
        self.team
    }
    pub fn modifiers(&self) -> impl Iterator<Item = &BaseModifier> {
        self.modifiers.iter()
    }
    pub fn alter_health(&mut self, delta: i32) {
        self.damage_taken -= delta;
    }
    
    pub fn id(&self) -> EntityID<Character> {
        self.id
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }
    
    pub fn damage_taken(&self) -> i32 {
        self.damage_taken
    }
    
    pub fn team_mut(&mut self) -> &mut EntityID<Team> {
        &mut self.team
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BaseCharacter {
    id: EntityID<BaseCharacter>,
    name: String,
    stats: Stats,
    modifiers: Vec<BaseModifier>
}
impl BaseCharacter {
    pub fn new(name: String, stats:Stats, modifiers: Vec<BaseModifier>) -> Self {
        BaseCharacter {
            id: EntityID::roll(),
            name,
            stats,
            modifiers
        }
    }
    pub fn roll(name: String, stats:Stats) -> Self {
        Self::new(name, stats, Vec::new())
    }
    pub fn stats(&self) -> &Stats {
        &self.stats
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }
    
    pub fn modifiers(&self) -> &Vec<BaseModifier> {
        &self.modifiers
    }
    pub fn id(&self) -> EntityID<BaseCharacter> {
        self.id
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Stats {
    pub violence:i8,
    pub bloodthirst:i8,
    pub realism:i8,
    pub perpetuity:i8,
    pub buoyancy:i8,
    pub maverickism:i8,
    pub run:i8,
}
impl Stats {
    pub fn example() -> Self {
        Stats { violence: 1, bloodthirst: 2, realism: 3, perpetuity: 4, buoyancy: 5, maverickism: 6, run: 7 }
    }
    pub fn get(&self, stat:Stat) -> i8 {
        match stat {
            Stat::Bloodthirst => self.bloodthirst,
            Stat::Violence => self.violence,
            Stat::Realism => self.realism,
            Stat::Perpetuity => self.perpetuity,
            Stat::Buoyancy => self.buoyancy,
            Stat::Maverickism => self.maverickism,
            Stat::Run => self.run
        }
    }
    pub fn get_mut(&mut self, stat:Stat) -> &mut i8 {
        match stat {
            Stat::Bloodthirst => &mut self.bloodthirst,
            Stat::Violence => &mut self.violence,
            Stat::Realism => &mut self.realism,
            Stat::Perpetuity => &mut self.perpetuity,
            Stat::Buoyancy => &mut self.buoyancy,
            Stat::Maverickism => &mut self.maverickism,
            Stat::Run => &mut self.run
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Stat {
    Violence,
    Bloodthirst,
    Realism,
    Perpetuity,
    Buoyancy,
    Maverickism,
    Run
}
impl Display for Stat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(match self {
            Stat::Bloodthirst => "bloodthirst",
            Stat::Violence => "violence",
            Stat::Realism => "realism",
            Stat::Perpetuity => "perpetuity",
            Stat::Buoyancy => "buoyancy",
            Stat::Maverickism => "maverickism",
            Stat::Run => "run",
        }, f)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum CharacterAlteration {
    ChangeStat(Stat, i8),
    AddModifier(BaseModifier)
}
impl CharacterAlteration {
    pub fn apply(self, mut character: BaseCharacter) -> BaseCharacter {
        match self {
            Self::AddModifier(modifier) => {
                character.modifiers.push(modifier);
            }
            Self::ChangeStat(stat, delta) => {
                *character.stats.get_mut(stat) += delta;
            }
        }
        character
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Team {
    id:EntityID<Team>,
    base:EntityID<BaseTeam>,
    name: String,
    colour:TeamColour
}
impl Team {
    pub fn new(base:BaseTeam) -> Self {
        Team {
            id: EntityID::roll(),
            base: base.id,
            name: base.name,
            colour: base.colour
        }
    }
    
    pub fn id(&self) -> EntityID<Team> {
        self.id
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn base(&self) -> EntityID<BaseTeam> {
        self.base
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BaseTeam {
    id:EntityID<BaseTeam>,
    incarnation_id:EntityID<BaseCharacter>,
    name: String,
    colour:TeamColour
}

impl BaseTeam {
    pub fn new(incarnation_id:EntityID<BaseCharacter>, name: String, colour: TeamColour) -> Self {
        BaseTeam {
            id:EntityID::roll(), incarnation_id, name, colour
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn incarnation_id(&self) -> EntityID<BaseCharacter> {
        self.incarnation_id
    }
    pub fn colour(&self) -> TeamColour {
        self.colour
    }
    
    pub fn id(&self) -> EntityID<BaseTeam> {
        self.id
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(transparent)] 
pub struct TeamColour([u8;3]);
impl From<[u8;3]> for TeamColour {
    fn from(value: [u8;3]) -> Self {
        Self(value)
    }
}
impl Display for TeamColour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&ron::to_string(self).unwrap(), f)
    }
}



impl SavableEntity for BaseCharacter {
    const TABLE_NAME: &'static str = "characters";
}
impl PatchableEntity for BaseCharacter {
    type Alteration = CharacterAlteration;
}
impl SavableEntity for BaseTeam {
    const TABLE_NAME: &'static str = "teams";
}
impl ChronobasePatch<BaseCharacter> for CharacterAlteration {
    fn apply(self, entity:BaseCharacter) -> BaseCharacter {
        CharacterAlteration::apply(self, entity)
    }
}