#![allow(dead_code)]
use delver_sim::{database::DatabaseManager, delver_display::ToDisplayConstruct, entities::{Character, Stats, Team}, game::Sim, modifiers::{Modifier, ModifierType}, progress_bars::Colour};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use chronobase::{DirectConnection, HTTPConnection};

fn main() {
    // Initialise Database
    let database = DirectConnection::new("temp.db".to_string());
    let database= DatabaseManager::new(Box::new(database));
    // let database = HTTPConnection::new("http://localhost:8000");

    let mut crabs = Team::new("Baltimore Crabs".to_string(), Colour::Red);
    let crab_team_id = crabs.id;
    let mut pirates = Team::new("Antalya Pirates".to_string(), Colour::Cyan);
    let pirate_team_id = pirates.id;
    
    let clawed_one = Character::roll("Baltimore Crabs".to_string(), Stats::example(), crabs.id);
    database.save(clawed_one.clone());
    
    let unnamed_pirate_divinity = Character::roll("Unnamed Pirate Divinity".to_string(), Stats::example(), pirates.id);
    database.save(unnamed_pirate_divinity);

    let mut crabs_roster:Vec<_> = (0..4).map(|i| Character::roll(format!("Crab {}", i), Stats::example(), crabs.id)).collect();
    let crabs_roster_ids: Vec<_> = crabs_roster.iter().map(|character| character.id).collect();
    crabs_roster[0].modifiers.push(Modifier::new(crabs_roster_ids[0], ModifierType::Grinder));
    crabs_roster[1].modifiers.push(Modifier::new(crabs_roster_ids[1], ModifierType::Resilient));
    crabs_roster.into_iter().for_each(|character| database.save(character));


    crabs.roster = crabs_roster_ids;
    database.save(crabs);


    let defenders:Vec<_> = (0..4).map(|i| Character::roll(format!("Pirate {}", i), Stats::example(), pirates.id)).collect();
    let defender_ids = defenders.iter().map(|character| character.id).collect();
    defenders.into_iter().for_each(|character| database.save(character));


    pirates.roster = defender_ids;
    database.save(pirates);


    // Sim begins
    // let sim_start_time = api_helper::reserve_temporal_index().unwrap();

    // Initialisation
    let mut rng = rand::thread_rng();
    let mut rng = ChaCha8Rng::seed_from_u64(rng.gen());


    let mut game = Sim::new(database, &mut rng, crab_team_id, pirate_team_id);
    game.display(crab_team_id, pirate_team_id);
    for _ in 0..10 {
        // game.display(crab_team_id, pirate_team_id);
        game.turn();
    }
    println!("{}", game.world.game_id);
    println!("{}", clawed_one.longform(&game.world, &game.database));
}