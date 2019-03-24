extern crate serde_json;
extern crate evolution_wire;
#[macro_use]
extern crate evolution;

use evolution_wire as wire;
use evolution::object::*;

test_harness!(situation = wire::Situation => Situation, {
    println!("{}", situation.is_valid());
});

/// An attacking situation between 2 to 4 species.
#[derive(Debug)]
struct Situation {
    attacker: Species,
    defender: Species,
    left: Option<Species>,
    right: Option<Species>,
}

impl Situation {
    fn is_valid(&self) -> bool {
        self.attacker.can_attack(&self.defender, self.left.as_ref(), self.right.as_ref())
    }
}

impl wire::FromWire<wire::Situation> for Situation {
    fn from_wire(wire: wire::Situation) -> Result<Situation, ()> {
        let attacker = try!(Species::from_wire(wire.attacker));
        let defender = try!(Species::from_wire(wire.defender));
        let left = if let Some(l) = wire.left {
            Some(try!(Species::from_wire(l)))
        } else {
            None
        };
        let right = if let Some(r) = wire.right {
            Some(try!(Species::from_wire(r)))
        } else {
            None
        };
        Ok(Situation {
            attacker: attacker,
            defender: defender,
            left: left,
            right: right,
        })
    }
}
