use evolution_wire as wire;
use interact::*;

impl wire::ToWire<wire::FeedChoice> for FeedChoice {
    fn to_wire(&self) -> wire::FeedChoice {
        match *self {
            FeedChoice::Abstain => {
                wire::FeedChoice::Abstain
            },
            FeedChoice::Feed(ref s) => {
                wire::FeedChoice::Feed(s.to_wire())
            },
            FeedChoice::Store(ref s, ref f) => {
                wire::FeedChoice::Store(s.to_wire(), f.to_wire())
            },
            FeedChoice::Attack(ref a, ref p, ref d) => {
                wire::FeedChoice::Attack(a.to_wire(), p.to_wire(), d.to_wire())
            },
        }
    }
}

impl wire::FromWire<wire::FeedChoice> for FeedChoice {
    fn from_wire(wire: wire::FeedChoice) -> Result<Self, ()> {
        let feed_choice = match wire {
            wire::FeedChoice::Abstain => FeedChoice::Abstain,
            wire::FeedChoice::Feed(species) => FeedChoice::Feed(*species as usize),
            wire::FeedChoice::Store(species, fat_food) => {
                FeedChoice::Store(*species as usize, *fat_food)
            },
            wire::FeedChoice::Attack(attacker, target, defender) => {
                FeedChoice::Attack(*attacker as usize, *target as usize, *defender as usize)
            }
        };
        Ok(feed_choice)
    }
}
