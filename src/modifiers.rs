use serde::{Deserialize, Serialize};

use crate::characters::CharacterID;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(transparent)] 
pub struct TeamColour([u8;3]);
impl From<[u8;3]> for TeamColour {
    fn from(value: [u8;3]) -> Self {
        Self(value)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Modifier {
    Team(Vec<CharacterID>,TeamColour),
    NPC
}