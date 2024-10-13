use crate::characters::{BaseCharacter, Stat};
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
    pub fn resolve(&self, proactor:BaseCharacter, reactor:BaseCharacter, rng:&mut SimRng) -> i8 {
        let modifier = 
        proactor.stat(self.proactive.primary) + proactor.stat(self.proactive.secondary) 
        - reactor.stat(self.reactive.primary) - reactor.stat(   self.reactive.secondary);
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