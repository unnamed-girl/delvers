use crate::characters::{BaseCharacter, Stat, Stats};
use rand::distributions::Distribution;

struct DiceDistribution(u8, u8);
impl Distribution<u8> for DiceDistribution {
    fn sample<R: rand::prelude::Rng + ?Sized>(&self, rng: &mut R) -> u8 {
        (0..self.0).map(|_| rng.gen_range(1..=self.1)).sum()
    }
}

type SimRng = rand_chacha::ChaCha8Rng;
// pub struct SimRng(pub rand_chacha::ChaCha8Rng);

const DICE:DiceDistribution = DiceDistribution(2, 10);

pub struct Action {
    //outcomes
    proactive:Roll,
    reactive:Roll
}

impl Action {
    pub fn resolve(&self, proactor:Stats, reactor:Stats, rng:&mut SimRng) -> i8 {
        let modifier = 
        proactor.get(&self.proactive.primary) + proactor.get(&self.proactive.secondary) 
        - reactor.get(&self.reactive.primary) - reactor.get(&self.reactive.secondary);
        DICE.sample(rng) as i8 + modifier
    }
}

struct Roll {
    primary:Stat,
    secondary:Stat
}

pub fn attack() -> Action {
    Action { proactive: Roll { primary: Stat::Violence, secondary: Stat::Run }, reactive: Roll { primary: Stat::Buoyancy, secondary: Stat::Maverickism } }
}