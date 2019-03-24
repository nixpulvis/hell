use interact::*;

#[derive(Debug)]
pub struct Silly;

impl Chooser for Silly {
    fn start(&mut self, _: &DealObservation) {
        // Do nothing.
    }

    fn info(&self) -> Option<&str> {
        None
    }
}

mod action;
mod feed;

mod ranked_feed_choice;
pub use self::ranked_feed_choice::RankedFeedChoice;

mod ranked_species;
pub use self::ranked_species::RankedSpecies;
