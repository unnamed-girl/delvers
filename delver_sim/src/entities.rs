use std::{fmt::{Debug, Display}, ops::{Index, IndexMut}};
use chronobase::{EntityID, SavableEntity};
use serde::{Deserialize, Serialize};

use crate::{modifiers::Modifier, progress_bars::Colour};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Character {
    pub id: EntityID<Character>,
    pub name: String,
    pub team: EntityID<Team>,
    pub stats: Stats,
    pub modifiers: Vec<Modifier>
}

impl Character {
    pub fn new(name: String, stats: Stats, team: EntityID<Team>) -> Self {
        Character {
            id: EntityID::roll(),
            name,
            team,
            stats,
            modifiers: Vec::new(),
        }
    }
    pub fn modifiers(&self) -> impl Iterator<Item = &Modifier> {
        self.modifiers.iter()
    }
    pub fn roll(name: String, stats:Stats, team: EntityID<Team>) -> Self {
        Self::new(name, stats, team)
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
impl Index<Stat> for Stats {
    type Output = i8;
    fn index(&self, index: Stat) -> &Self::Output {
        match index {
            Stat::Bloodthirst => &self.bloodthirst,
            Stat::Violence => &self.violence,
            Stat::Realism => &self.realism,
            Stat::Perpetuity => &self.perpetuity,
            Stat::Buoyancy => &self.buoyancy,
            Stat::Maverickism => &self.maverickism,
            Stat::Run => &self.run
        }
    }
}

impl IndexMut<Stat> for Stats {
    fn index_mut(&mut self, index: Stat) -> &mut Self::Output {
        match index {
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
impl Stats {
    pub fn example() -> Self {
        Stats { violence: 1, bloodthirst: 2, realism: 3, perpetuity: 4, buoyancy: 5, maverickism: 6, run: 7 }
    }
    pub fn get(&self, stat:Stat) -> i8 {
        self[stat]
    }
    pub fn get_mut(&mut self, stat:Stat) -> &mut i8 {
        &mut self[stat]
    }
    pub fn canonical_order() -> [Stat; 7] {
        [Stat::Bloodthirst, Stat::Violence, Stat::Realism, Stat::Perpetuity, Stat::Buoyancy, Stat::Maverickism, Stat::Run]
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


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Team {
    pub id: EntityID<Team>,
    pub name: String,
    pub colour:Colour,
    pub roster: Vec<EntityID<Character>>
}
impl Team {
    pub fn new(name: String, colour: Colour) -> Self {
        Team {
            id: EntityID::roll(),
            name,
            colour,
            roster: Vec::new()
        }
    }
}



impl SavableEntity for Character {
    const TABLE_NAME: &'static str = "characters";
}
impl SavableEntity for Team {
    const TABLE_NAME: &'static str = "teams";
}