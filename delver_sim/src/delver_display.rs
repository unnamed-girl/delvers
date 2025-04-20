use std::fmt::Display;

use colored::Colorize;
use itertools::Itertools;

use crate::{database::DatabaseManager, entities::{Character, Stats}, events::{CompletedEvent, Event, ExecutedEvent}, game::{ActiveCharacter, ActiveCharacterID, Game}, progress_bars::{GameProgressBarLocation, ProgressBar}};

pub trait ToDisplayConstruct {
    fn shortform(&self, game: &Game, database: &DatabaseManager) -> String;
    fn longform(&self, game: &Game, database: &DatabaseManager) -> DisplayConstruct;
}
impl ToDisplayConstruct for ProgressBar {
    fn shortform(&self, _game: &Game, _database: &DatabaseManager) -> String {
        format!("{:?}: {} ", self.name, self)
    }
    fn longform(&self, game: &Game, database: &DatabaseManager) -> DisplayConstruct {
        DisplayConstruct::Single(self.shortform(game, database))
    }
}
impl ToDisplayConstruct for Stats {
    fn shortform(&self, _game: &Game, _database: &DatabaseManager) -> String {
        format!("[{}]", Stats::canonical_order().into_iter().map(|stat| 
            self.get(stat).to_string()
        ).join(", "))
    }
    fn longform(&self, _game: &Game, _database: &DatabaseManager) -> DisplayConstruct {
        DisplayConstruct::List(Stats::canonical_order().into_iter().map(|stat| 
            format!("{stat}: {}", self.get(stat))
        ).collect())
    }
}
impl ToDisplayConstruct for Character {
    fn shortform(&self, _game: &Game, database: &DatabaseManager) -> String {
        let team = database.load(self.team);
        self.name.color(team.colour).to_string()
    }
    fn longform(&self, game: &Game, database: &DatabaseManager) -> DisplayConstruct {
        let name = self.shortform(game, database);
        DisplayConstruct::Multi(
            vec![
                DisplayConstruct::Single(name),
                self.stats.longform(game, database)
            ]
        )
    }
}
impl ToDisplayConstruct for ActiveCharacterID {
    fn shortform(&self, game: &Game, database: &DatabaseManager) -> String {
        game.active_characters.get(*self).shortform(game, database)
    }
    fn longform(&self, game: &Game, database: &DatabaseManager) -> DisplayConstruct {
        game.active_characters.get(*self).longform(game, database)
    }
}
impl ToDisplayConstruct for ActiveCharacter {
    fn shortform(&self, game: &Game, database: &DatabaseManager) -> String {
        database.load(self.character).shortform(game, database)
    }
    fn longform(&self, game: &Game, database: &DatabaseManager) -> DisplayConstruct {
        DisplayConstruct::Multi(vec![
            DisplayConstruct::Single(self.shortform(game, database)),
            DisplayConstruct::List(
                self.progress_bars.values().map(|bar| bar.longform(game, database).to_string()).collect()
            )
        ])
    }
}
impl ToDisplayConstruct for GameProgressBarLocation {
    fn shortform(&self, game: &Game, database: &DatabaseManager) -> String {
        match self {
            GameProgressBarLocation::Character(character, bar_name) => {
                let character = game.active_characters.get(*character).shortform(game, database);
                format!("{character}'s {bar_name:?}")
            }
        }
    }
    fn longform(&self, game: &Game, database: &DatabaseManager) -> DisplayConstruct {
        DisplayConstruct::Single(self.shortform(game, database))
    }
}
impl ToDisplayConstruct for CompletedEvent {
    fn shortform(&self, game: &Game, database: &DatabaseManager) -> String {
        self.event.shortform(game, database)
    }
    fn longform(&self, game: &Game, database: &DatabaseManager) -> DisplayConstruct {
        DisplayConstruct::ParentChildren(Box::new(self.event.longform(game, database)),
            self.pre_responses.iter()
                .chain(self.outcomes.iter())
                .chain(self.post_responses.iter())                
                .map(|event| event.longform(game, database)).collect()
        )
    }
}
impl ToDisplayConstruct for Event {
    fn shortform(&self, game: &Game, database: &DatabaseManager) -> String {
        match self {
            Self::Attack { attacker, target } => {
                format!("{} attacks {}", attacker.shortform(game, database), target.shortform(game, database))
            }
            Self::CreateProgressBar { location, .. } => {
                    format!("Created {}", location.shortform(game, database))
            }
            Self::ProgressProgressBar { location, amount } => {
                let bar = game.get_progress_bar(*location);
                let change_verb = bar.style.change_verb();

                format!("{} {change_verb}s by {amount}", location.shortform(game, database))
            }
            Self::Say(string) => string.clone()
        }
    }
    fn longform(&self, game: &Game, database: &DatabaseManager) -> DisplayConstruct {
        match self {
            Self::Attack { .. } => {
                DisplayConstruct::Single(self.shortform(game, database))
            }
            Self::CreateProgressBar { .. } => {
                DisplayConstruct::Single(self.shortform(game, database))
            }
            Self::ProgressProgressBar { location, .. } => {
                let bar = game.get_progress_bar(*location);
                DisplayConstruct::Single(format!("{} => {bar}", self.shortform(game, database)))
            }
            Self::Say(_) => DisplayConstruct::Single(self.shortform(game, database))
        }
    }
}
impl ToDisplayConstruct for ExecutedEvent {
    fn longform(&self, game: &Game, database: &DatabaseManager) -> DisplayConstruct {
        self.0.longform(game, database)
    }
    fn shortform(&self, game: &Game, database: &DatabaseManager) -> String {
        self.0.shortform(game, database)
    }
}

pub enum DisplayConstruct {
    Single(String),
    List(Vec<String>),
    Multi(Vec<DisplayConstruct>),
    ParentChildren(Box<DisplayConstruct>, Vec<DisplayConstruct>)
}

impl Display for DisplayConstruct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Single(string) => string.fmt(f),
            Self::List(list) => {
                list.iter().map(|a| format!("- {a}")).join("\n").fmt(f)
            }
            Self::Multi(multiple) => {
                multiple.iter().join("\n").fmt(f)
            }
            Self::ParentChildren(parent, children) => {
                let children = children.iter().flat_map(|child| child.to_string().lines().map(|s| format!("- {s}")).collect::<Vec<_>>()).join("\n");
                write!(f, "{parent}\n{children}")
            }
        }
    }
}