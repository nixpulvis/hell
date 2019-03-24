/// An observation of some data.
pub trait Observation {}

/// Asking some object for an observation of itself.
///
/// This is used, for example to get an observation of the game for a player to
/// choose a response for feeding.
pub trait Observe<O: Observation> {
    /// Return the observation of this data.
    fn observe(&self) -> O;
}

/// Define implementations of observe for slices containing observable data.
macro_rules! impl_slice_observe {
    ($observation:ident, $data_type:ty) => {
        impl Observation for Vec<$observation> {}

        impl<'a> Observe<Vec<$observation>> for &'a [$data_type] {
            fn observe(&self) -> Vec<$observation> {
                self.iter().map(|t| t.observe()).collect()
            }
        }
    };
}

mod player_observation;
pub use self::player_observation::PlayerObservation;

mod board_observation;
pub use self::board_observation::BoardObservation;

mod deal_observation;
pub use self::deal_observation::DealObservation;

mod action_observation;
pub use self::action_observation::ActionObservation;

mod feed_observation;
pub use self::feed_observation::FeedObservation;
