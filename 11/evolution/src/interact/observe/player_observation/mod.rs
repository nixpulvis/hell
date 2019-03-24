use game::*;
use object::*;
use interact::*;

/// The visable state of a player.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PlayerObservation {
    pub id: Id,
    pub domain: Domain,
    pub bag: u64,
}

impl Observation for PlayerObservation {}

impl Observe<PlayerObservation> for Player {
    fn observe(&self) -> PlayerObservation {
        PlayerObservation {
            id: self.id(),
            domain: self.domain().clone(),
            bag: self.bag().len() as u64,
        }
    }
}

impl_slice_observe!(PlayerObservation, Player);

#[cfg(feature = "wire")]
mod wire;

#[cfg(test)]
mod tests {
    use interact::*;
    use object::*;

    #[test]
    fn observe() {
        let mut player = Player::new(1);
        player.domain_mut().add(Placement::Right);
        player.push_card(Card::mock(2, Trait::Burrowing));

        assert_eq!(1, player.id());
        assert_eq!(1, player.domain().len());
        assert_eq!(1, player.hand().len());

        let observed_player = player.observe();

        assert_eq!(1, observed_player.id);
        assert_eq!(1, observed_player.domain.len());
    }
}
