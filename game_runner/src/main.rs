#![allow(dead_code)]
use delver_sim::{entities::{BaseCharacter, BaseTeam, CharacterAlteration, Stats}, game::{Game, LoadableID}, modifiers::BaseModifier};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use chronobase::{database_connection::Typebase, HTTPConnection};

fn main() {
    // Initialise Database
    // let database = DirectConnection::new("temp.db".to_string());
    let database = HTTPConnection::new("http://localhost:8000");
    let incarnation = BaseCharacter::roll("Baltimore Crabs".to_string(), Stats::example());
    database.save(incarnation.id(), &incarnation).unwrap();
    let dungeon = BaseCharacter::roll("Dungeon".to_string(), Stats::example());
    database.save(dungeon.id(), &dungeon).unwrap();

    let delvers:Vec<_> = (0..4).map(|i| BaseCharacter::roll(format!("Delver {}", i), Stats::example())).collect();
    delvers.iter().for_each(|character| {database.save(character.id(), character).unwrap();});
    let delvers = delvers.iter().map(|character| character.id()).collect();
    database.alter(incarnation.id(), CharacterAlteration::AddModifier(BaseModifier::Team(delvers))).unwrap();

    let defenders:Vec<_> = (0..4).map(|i| BaseCharacter::roll(format!("Defender {}", i), Stats::example())).collect();
    defenders.iter().for_each(|character| {database.save(character.id(), character).unwrap();});
    let defenders = defenders.iter().map(|character| character.id()).collect();
    database.alter(dungeon.id(), CharacterAlteration::AddModifier(BaseModifier::Dungeon(defenders))).unwrap();

    let defender_team = BaseTeam::new(dungeon.id(), "Defenders".to_string(), [0,0,0].into());
    let defender_team_id = defender_team.id();
    
    let delver_team = BaseTeam::new(incarnation.id(), "Delvers".to_string(), [252,0,0].into());
    let delver_team_id = delver_team.id();

    database.save(defender_team_id, &defender_team).unwrap();
    database.save(delver_team_id, &delver_team).unwrap();

    // Sim begins
    // let sim_start_time = api_helper::reserve_temporal_index().unwrap();

    // Initialisation
    let mut rng = rand::thread_rng();
    let mut rng = ChaCha8Rng::seed_from_u64(rng.gen());


    let mut game = Game::new(&database, &mut rng, delver_team_id, defender_team_id);
    let active_defender_team = *game.turn_order.iter().filter(|team| team.get(&game).base() == defender_team_id).next().unwrap();
    let active_delver_team = *game.turn_order.iter().filter(|team| team.get(&game).base() == delver_team_id).next().unwrap();
    
    for _ in 0..5 {
        game.display(active_delver_team, active_defender_team);
        game.turn();
    }
}