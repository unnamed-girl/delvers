use std::{fmt::{Debug, Display}, path::Path};

use rusqlite::{types::FromSql, Connection};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CharacterID(pub usize);
impl Display for CharacterID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

pub struct CharacterTable(Connection);
impl CharacterTable {
    pub fn connect<P:AsRef<Path>>(path:P) -> Option<Self> {
        let connection = Connection::open("DelverBase.db").unwrap();
        connection.execute("CREATE TABLE IF NOT EXISTS Characters(id INTEGER PRIMARY KEY AUTOINCREMENT, name,
        violence INTEGER, bloodthirst INTEGER, realism INTEGER, perpetuity INTEGER, buoyancy INTEGER, maverickism INTEGER, run INTEGER)",
        ())
            .ok()?;
        Some(CharacterTable(connection))
    }
    pub fn get(&self, id:CharacterID) -> Result<BaseCharacter, rusqlite::Error> {
        let mut stmnt = self.0.prepare(&format!("SELECT name,
         violence, bloodthirst, realism, perpetuity, buoyancy, maverickism, run
         from Characters WHERE id = {}", id.0))?;
        let result = stmnt.query_row([], |row| {
            Ok(BaseCharacter {
                id: id,
                name: row.get(0)?,
                stats: Stats { violence: row.get(1)?, bloodthirst: row.get(2)?, realism: row.get(3)?, perpetuity: row.get(4)?, buoyancy: row.get(5)?, maverickism: row.get(6)?, run: row.get(7)? }
            })
        })?;
        Ok(result)
    }
    pub fn put(&self, name:String, stats:Stats) -> Result<CharacterID, rusqlite::Error> {
        let insert = format!("INSERT INTO Characters(name, violence, bloodthirst, realism, perpetuity, buoyancy, maverickism, run)");
        let values = format!(" VALUES ('{}', {}, {}, {}, {}, {}, {}, {})", name, stats.violence, stats.bloodthirst, stats.realism, stats.perpetuity, stats.buoyancy, stats.maverickism, stats.run);
        let sql = insert + &values;
        self.0.execute(&sql, [])?;
        let id = self.0.last_insert_rowid();
        Ok(CharacterID(id as usize))
    }
}

#[derive(Debug)]
pub struct BaseCharacter {
    id:CharacterID,
    name: String,
    pub stats: Stats,
    //modifiers
}

#[derive(Serialize, Deserialize, Debug)]
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
    pub fn get(&self, stat:&Stat) -> i8 {
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
pub enum Stat {
    Violence,
    Bloodthirst,
    Realism,
    Perpetuity,
    Buoyancy,
    Maverickism,
    Run
}