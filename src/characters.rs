use std::{fmt::{Debug, Display}, path::Path};
use serde::{Deserialize, Serialize};
use rusqlite::Connection;

use crate::modifiers::Modifier;

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)] 
pub struct CharacterID(usize);
impl From<usize> for CharacterID {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl Debug for CharacterID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.0,f)
    }
}
impl Display for CharacterID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

pub struct CharacterTable(Connection);
impl CharacterTable {
    pub fn connect<P:AsRef<Path>>(path:P) -> Option<Self> {
        let connection = Connection::open(path).unwrap();
        connection.execute("CREATE TABLE IF NOT EXISTS Characters(id INTEGER PRIMARY KEY AUTOINCREMENT, name, modifiers,
        violence INTEGER, bloodthirst INTEGER, realism INTEGER, perpetuity INTEGER, buoyancy INTEGER, maverickism INTEGER, run INTEGER)",
        ())
            .ok()?;
        Some(CharacterTable(connection))
    }
    pub fn get(&self, id:CharacterID) -> Result<BaseCharacter, rusqlite::Error> {
        let mut stmnt = self.0.prepare(&format!("SELECT name, modifiers,
         violence, bloodthirst, realism, perpetuity, buoyancy, maverickism, run
         from Characters WHERE id = {}", id.0))?;
        let result = stmnt.query_row([], |row| {
            let modifiers_string:String = row.get(1)?;
            Ok(BaseCharacter {
                id: id,
                name: row.get(0)?,
                modifiers: ron::from_str(&modifiers_string).unwrap(),
                stats: Stats { violence: row.get(2)?, bloodthirst: row.get(3)?, realism: row.get(4)?, perpetuity: row.get(5)?, buoyancy: row.get(6)?, maverickism: row.get(7)?, run: row.get(8)? },
            })
        })?;
        Ok(result)
    }
    pub fn put(&self, name:String, stats:Stats) -> Result<CharacterID, rusqlite::Error> {
        let insert = format!("INSERT INTO Characters(name, modifiers, violence, bloodthirst, realism, perpetuity, buoyancy, maverickism, run)");
        let values = format!(" VALUES ('{}', '{}', {}, {}, {}, {}, {}, {}, {})", name, "[]", stats.violence, stats.bloodthirst, stats.realism, stats.perpetuity, stats.buoyancy, stats.maverickism, stats.run);
        let sql = insert + &values;
        self.0.execute(&sql, [])?;
        let id = self.0.last_insert_rowid();
        Ok(CharacterID(id as usize))
    }
    fn set_field(&self, field:String, value:String, id:CharacterID) -> Result<(), rusqlite::Error> {
        self.0.execute(&format!("UPDATE Characters SET {}={} WHERE id = {}", field, value, id.0), [])?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct BaseCharacter {
    id:CharacterID,
    name: String,
    stats: Stats,
    modifiers: Vec<Modifier>
}
impl BaseCharacter {
    pub fn stat(&self, stat:Stat) -> i8 {
        self.stats.get(stat)
    }
}

#[derive(Debug)]
pub struct Stats {
    violence:i8,
    bloodthirst:i8,
    realism:i8,
    perpetuity:i8,
    buoyancy:i8,
    maverickism:i8,
    run:i8,
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
}

#[derive(Clone, Copy, Debug)]
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

pub enum CharacterAlteration {
    ChangeStat(Stat, i8),
    AddModifier(Modifier)
}
impl CharacterTable {
    pub fn handle_alteration(&self, id:CharacterID, alteration:CharacterAlteration) -> Result<(), rusqlite::Error>{
        match alteration {
            CharacterAlteration::ChangeStat(stat, delta) => {
                let old_stat = self.get(id)?.stat(stat);
                let new_stat = old_stat + delta;
                self.set_field(stat.to_string(), new_stat.to_string(), id)?;
            }
            CharacterAlteration::AddModifier(modifier) => {
                let mut modifiers = self.get(id)?.modifiers;
                modifiers.push(modifier);
                self.set_field("modifiers".to_string(), format!("'{}'",ron::to_string(&modifiers).unwrap()), id)?;
            }
        }
        Ok(())
    }
}