use std::{cmp::min, fmt::Display};

use colored::Colorize;
use serde::{Deserialize, Serialize};

use crate::game::ActiveCharacterID;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProgressBarName {
    XP,
    HP
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameProgressBarLocation {
    Character(ActiveCharacterID, ProgressBarName),
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum ProgressBarStyle {
    Fill,
    Drain
}
impl ProgressBarStyle {
    pub fn change_verb(&self) -> &'static str {
        match self {
            Self::Fill => "increase",
            Self::Drain => "decrease",
        }
    }
}

/// Colours that are printable to discord
#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Colour {
    Gray,
    Red,
    Green,
    Yellow,
    Blue,
    Pink,
    Cyan,
    White
}
impl From<Colour> for colored::Color {
    fn from(value: Colour) -> colored::Color {
        match value {
            Colour::Gray => colored::Color::Black,
            Colour::Red => colored::Color::Red,
            Colour::Green => colored::Color::Green,
            Colour::Yellow => colored::Color::Yellow,
            Colour::Blue => colored::Color::Blue,
            Colour::Pink => colored::Color::Magenta,
            Colour::Cyan => colored::Color::Cyan,
            Colour::White => colored::Color::White
        }
    }
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct ProgressBar {
    pub max: u16,
    pub name: ProgressBarName,
    pub progress: u16,
    pub colour: Colour,
    pub style: ProgressBarStyle
}
impl ProgressBar {
    pub fn new(max:u16, name: ProgressBarName, colour: Colour, style: ProgressBarStyle) -> Self {
        Self { max, name, progress: 0, colour, style}
    }
    pub fn increment(&mut self, delta:u16) {
        self.progress = min(self.max, self.progress + delta);
    }
    pub fn complete(&self) -> bool {
        self.max <= self.progress
    }
}

impl Display for ProgressBar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let completed = "o".color(self.colour);
        let uncompleted = "o".color(Colour::White);
        let mut result = String::new();

        match self.style {
            ProgressBarStyle::Fill => {
                for _ in 0..self.progress {
                    result += &completed.to_string();
                }
                for _ in self.progress..self.max {
                    result += &uncompleted.to_string();
                }
            }
            ProgressBarStyle::Drain => {
                for _ in self.progress..self.max {
                    result += &completed.to_string();
                }
                for _ in 0..self.progress {
                    result += &uncompleted.to_string();
                }
            }
        }

        result.fmt(f)
    }
}

pub fn health_bar() -> ProgressBar {
    ProgressBar::new(4,  ProgressBarName::HP, Colour::Red, ProgressBarStyle::Drain)
}
pub fn xp_bar() -> ProgressBar {
    ProgressBar::new(4, ProgressBarName::XP, Colour::Blue, ProgressBarStyle::Fill)
}