use evolution_wire as wire;
use interact::*;
use object::*;

impl wire::ToWire<(wire::remote::LOB, wire::remote::LOB)> for ActionObservation {
    fn to_wire(&self) -> (wire::remote::LOB, wire::remote::LOB) {
        let before = self.before.iter().map(|d| d.to_wire()).collect();
        let after = self.after.iter().map(|d| d.to_wire()).collect();
        (before, after)
    }
}

impl wire::FromWire<(wire::remote::Start, (wire::remote::LOB, wire::remote::LOB))> for ActionObservation {
    fn from_wire(wire: (wire::remote::Start, (wire::remote::LOB, wire::remote::LOB))) -> Result<Self, ()> {
        let (start, (before, after)) = wire;
        Ok(ActionObservation {
            current_player: try!(Player::from_wire(start)),
            before: try!(Vec::from_wire(before)),
            after: try!(Vec::from_wire(after)),
        })
    }
}

// NOTE: These impls are based on old data, and are being left in for old tests.

impl wire::ToWire<wire::Choice> for ActionObservation {
    fn to_wire(&self) -> wire::Choice {
        wire::Choice {
            current_player: self.current_player.to_wire(),
            before: self.before.iter().map(|d| d.to_wire()).collect(),
            after: self.after.iter().map(|d| d.to_wire()).collect(),
        }
    }
}

impl wire::FromWire<wire::Choice> for ActionObservation {
    fn from_wire(wire: wire::Choice) -> Result<Self, ()> {
        let current_player = try!(Player::from_wire(wire.current_player));

        // TODO: Should be able to get a vec of domains from wire directly.
        let mut before = Vec::new();
        let mut after = Vec::new();

        for b in wire.before {
            let domain = try!(Domain::from_wire(b));
            before.push(domain);
        }

        for a in wire.after {
            let domain = try!(Domain::from_wire(a));
            after.push(domain);
        }

        Ok(
            ActionObservation {
                current_player: current_player,
                before: before,
                after: after,
            }
        )
    }
}
