#![allow(dead_code)]

use actions::attack;
use characters::{CharacterID, CharacterTable};
use modifiers::{Modifier, TeamColour};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

mod actions;
mod characters;
mod modifiers;

fn main() {
    let mut rng = rand::thread_rng();
    let mut rng = ChaCha8Rng::seed_from_u64(rng.gen());

    let table = CharacterTable::connect("DelverBase.db").unwrap();
    // table.handle_alteration(CharacterID::from(1), characters::CharacterAlteration::AddModifier(Modifier::Team(vec![CharacterID::from(2)], TeamColour::from([252,0,0])))).unwrap();
    // table.handle_alteration(CharacterID::from(1), characters::CharacterAlteration::AddModifier(Modifier::NPC)).unwrap();
    
    // println!("{}",table.put(String::from("Example"), Stats::example()).unwrap());
    // println!("{}",table.put(String::from("Example2"), Stats::example()).unwrap());
    let c_one = table.get(CharacterID::from(1)).unwrap();
    let c_two = table.get(CharacterID::from(2)).unwrap();

    println!("{:?}", c_one);

    let action = attack();

    println!("{}", action.resolve(c_one, c_two, &mut rng));
}
