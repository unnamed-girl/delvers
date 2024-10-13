use actions::attack;
use characters::{CharacterID, CharacterTable, Stats};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

mod actions;
mod characters;

struct RollOutcomes(Vec<OutcomeBucket>);
struct OutcomeBucket {
    lower:Option<u8>,
    upper:Option<u8>
    //AsociatedEvent
}

impl OutcomeBucket {
    fn crit_fail() -> Self {
        OutcomeBucket { lower:None, upper:Some(4) }
    }
    fn fail() -> Self {
        OutcomeBucket { lower:Some(5), upper:Some(10) }
    }
    fn success() -> Self {
        OutcomeBucket { lower: Some(11), upper: Some(17) }
    }
    fn crit_success() -> Self {
        OutcomeBucket { lower: Some(18), upper: None }
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut rng = ChaCha8Rng::seed_from_u64(rng.gen());

    let table = CharacterTable::connect("DelverBase.db").unwrap();
    // println!("{}",table.put(String::from("Example2"), Stats::example()).unwrap());
    let c_one = table.get(CharacterID(1)).unwrap();
    let c_two = table.get(CharacterID(2)).unwrap();

    let action = attack();

    println!("{}", action.resolve(c_one.stats, c_two.stats, &mut rng));
}
