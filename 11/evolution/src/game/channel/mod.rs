use evolution_wire::{self as wire, ToWire, FromWire, Channel};
use interact::*;

impl Chooser for Channel {
    fn start(&mut self, observation: &DealObservation) {
        // HACK: We don't technically care if this message was sent
        // successfully.
        self.send(&observation.to_wire()).ok();
    }

    fn info(&self) -> Option<&str> {
        Some(self.info())
    }
}

impl Choose<ActionObservation, ActionChoice> for Channel {
    fn choose(&mut self, observation: &ActionObservation) -> Result<Option<ActionChoice>, ()> {
        let wire_observation: (wire::remote::LOB, wire::remote::LOB) = observation.to_wire();
        trace!("Channel.choose<(wire::remote::LOB, wire::remote::LOB)>");
        let wire: wire::remote::Action4 = match self.call(&wire_observation) {
            Ok(w) => w,
            Err(_) => return Err(()),
        };
        ActionChoice::from_wire(wire).map(|c| Some(c))
    }
}

impl Choose<FeedObservation, FeedChoice> for Channel {
    fn choose(&mut self, observation: &FeedObservation) -> Result<Option<FeedChoice>, ()> {
        let wire_observation: wire::remote::State = observation.to_wire();
        trace!("Channel.choose<wire::remote::State>");
        let wire = match self.call(&wire_observation) {
            Ok(w) => w,
            Err(_) => return Err(()),
        };
        FeedChoice::from_wire(wire).map(|c| Some(c))
    }
}
