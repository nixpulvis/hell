use std::cmp::Ordering;
use std::ops::{Deref, DerefMut};
use object::*;
use interact::*;
use silly::*;

/// A ranked feed choice, where lower ranked choices are best.
///
/// Rankings of feed choices follow the following order, from best to worst.
///
/// 1. Fat Tissue by need then by species.
/// 2. Vegetarian by species.
/// 3. Carnivore by species.
///
/// See `rank::Species` for more information on the species ranking.
#[derive(Debug)]
pub struct RankedFeedChoice<'a>(pub FeedChoice, pub &'a FeedObservation);

impl<'a> RankedFeedChoice<'a> {
    fn cmp_species(self_species: &'a Species, other_species: &'a Species) -> Ordering {
        let self_rank_species = RankedSpecies::from(self_species);
        let other_rank_species = RankedSpecies::from(other_species);
        self_rank_species.cmp(&other_rank_species)
    }

    fn cmp_attack_choices(a: (&Species, &Species),
                          b: (&Species, &Species)) -> Ordering
    {
        match Self::cmp_species(a.0, b.0) {
            Ordering::Equal => Self::cmp_species(a.1, b.1),
            ordering => ordering,
        }
    }
}

impl<'a> Ord for RankedFeedChoice<'a> {
    fn cmp(&self, other: &RankedFeedChoice<'a>) -> Ordering {
        let current_player = &self.1.current_player;
        match (self.0, other.0) {
            (FeedChoice::Store(self_species_idx, self_amount),
             FeedChoice::Store(other_species_idx, other_amount)) =>
            {
                let self_species = &current_player.domain()[self_species_idx];
                let other_species = &current_player.domain()[other_species_idx];
                match other_amount.cmp(&self_amount) {
                    Ordering::Equal => Self::cmp_species(self_species, other_species),
                    ordering => ordering,
                }
            },
            (FeedChoice::Store(_, _), _) => Ordering::Less,
            (_, FeedChoice::Store(_, _)) => Ordering::Greater,
            (FeedChoice::Feed(self_species_idx),
             FeedChoice::Feed(other_species_idx)) =>
            {
                let self_species = &current_player.domain()[self_species_idx];
                let other_species = &current_player.domain()[other_species_idx];
                Self::cmp_species(self_species, other_species)
            },
            (FeedChoice::Feed(_), _) => Ordering::Less,
            (_, FeedChoice::Feed(_)) => Ordering::Greater,
            (FeedChoice::Attack(self_species_idx,
                                             self_target_idx,
                                             self_defender_idx),
             FeedChoice::Attack(other_species_idx,
                                             other_target_idx,
                                             other_defender_idx)) =>
            {
                let self_species = &current_player.domain()[self_species_idx];
                let other_species = &current_player.domain()[other_species_idx];
                let self_defender = &self.1.opponents[self_target_idx].domain[self_defender_idx];
                let other_defender = &self.1.opponents[other_target_idx].domain[other_defender_idx];
                Self::cmp_attack_choices((self_species, self_defender),
                                         (other_species, other_defender))
            },
            (FeedChoice::Attack(_, _, _), _) => Ordering::Less,
            (_, FeedChoice::Attack(_, _, _)) => Ordering::Greater,
            _ => Ordering::Equal,
        }
    }
}

impl<'a> PartialOrd for RankedFeedChoice<'a> {
    fn partial_cmp(&self, other: &RankedFeedChoice<'a>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Eq for RankedFeedChoice<'a> {}

impl<'a> PartialEq for RankedFeedChoice<'a> {
    fn eq(&self, other: &RankedFeedChoice<'a>) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl<'a> Deref for RankedFeedChoice<'a> {
    type Target = FeedChoice;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for RankedFeedChoice<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> From<RankedFeedChoice<'a>> for FeedChoice {
    fn from(other: RankedFeedChoice<'a>) -> Self {
        other.0
    }
}

#[cfg(test)]
mod tests {
    use game::*;
    use object::*;
    use interact::*;
    use silly::*;

    #[test]
    fn test_ranked_eat() {
        let mut game = Game::<Silly>::new(4).unwrap();
        {
            let fat = game.players_mut()[0].domain_mut().add(Placement::Left);
            fat.evolve(Trait::FatTissue).unwrap();
            fat.grow().unwrap();
        }
        game.players_mut()[0].domain_mut().add(Placement::Right);
        {
            let carnivore = game.players_mut()[0].domain_mut().add(Placement::Right);
            carnivore.evolve(Trait::Carnivore).unwrap();
        }
        game.players_mut()[2].domain_mut().add(Placement::Left);
        let observed_game = game.observe();

        let feed0 = RankedFeedChoice(FeedChoice::Store(0, 1), &observed_game);
        let feed1 = RankedFeedChoice(FeedChoice::Feed(1), &observed_game);
        let feed2 = RankedFeedChoice(FeedChoice::Attack(2, 1, 0), &observed_game);
        assert!(feed0 < feed1);
        assert!(feed1 < feed2);
        assert!(feed0 < feed2);
    }

    #[test]
    fn test_fat_before_carnivore() {
        let mut game = Game::<Silly>::new(4).unwrap();
        {
            let fat = game.players_mut()[0].domain_mut().add(Placement::Left);
            fat.evolve(Trait::FatTissue).unwrap();
            fat.grow().unwrap();
        }
        {
            let carnivore = game.players_mut()[0].domain_mut().add(Placement::Right);
            carnivore.evolve(Trait::Carnivore).unwrap();
        }
        game.players_mut()[1].domain_mut().add(Placement::Left);
        game.players_mut()[2].domain_mut().add(Placement::Left);
        let observed_game = game.observe();

        let feed0 = RankedFeedChoice(FeedChoice::Store(0, 1), &observed_game);
        let feed1 = RankedFeedChoice(FeedChoice::Attack(1, 1, 0), &observed_game);
        assert!(feed0 < feed1);
    }
}
