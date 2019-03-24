use std::cmp::{PartialOrd, Ordering};
use std::ops::Deref;
use object::Species;

/// A ranked species, where lower ranked species are best.
///
/// Rankings of species follow the following order, from best to worst.
///
/// 1. Population value.
/// 2. Food value.
/// 3. body size value.
pub struct RankedSpecies<'a>(&'a Species);

impl<'a> Ord for RankedSpecies<'a> {
    // TODO: Is population 4 < population 3?
    fn cmp(&self, other: &RankedSpecies<'a>) -> Ordering {
        match other.population().cmp(&self.population()) {
            Ordering::Equal => {
                match (other.food().len() as u64).cmp(&(self.food().len() as u64)) {
                    Ordering::Equal => {
                        other.body_size().cmp(&self.body_size())
                    },
                    food_cmp => food_cmp
                }
            },
            pop_cmp => pop_cmp
        }
    }
}

impl<'a> PartialOrd for RankedSpecies<'a> {
    fn partial_cmp(&self, other: &RankedSpecies<'a>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Eq for RankedSpecies<'a> {}

impl<'a> PartialEq for RankedSpecies<'a> {
    fn eq(&self, other: &RankedSpecies<'a>) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl<'a> Deref for RankedSpecies<'a> {
    type Target = Species;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> From<&'a Species> for RankedSpecies<'a> {
    fn from(other: &'a Species) -> Self {
        RankedSpecies(other)
    }
}

impl<'a> From<RankedSpecies<'a>> for &'a Species {
    fn from(other: RankedSpecies<'a>) -> Self {
        other.0
    }
}

#[cfg(test)]
mod tests {
    use object::*;
    use silly::*;

    #[test]
    fn test_ranked_species_ord() {
        let mut s0 = Species::default();
        let mut s1 = Species::default();
        let mut board = Board::default();
        let food_tokens = vec![FoodToken, FoodToken];
        board.push_foods(food_tokens);

        {
            let spec0 = RankedSpecies::from(&s0);
            let spec1 = RankedSpecies::from(&s1);
            assert!( spec0 == spec1);
        }

        s0.breed().unwrap();
        {
            let spec0 = RankedSpecies::from(&s0);
            let spec1 = RankedSpecies::from(&s1);
            assert!(spec0 < spec1);
        }

        s1.breed().unwrap();
        {
            let spec0 = RankedSpecies::from(&s0);
            let spec1 = RankedSpecies::from(&s1);
            assert!(spec0 == spec1);
        }

        s0.eat(FoodToken).unwrap();
        {
            let spec0 = RankedSpecies::from(&s0);
            let spec1 = RankedSpecies::from(&s1);
            assert!(spec0 < spec1);
        }

        s1.eat(FoodToken).unwrap();
        {
            let spec0 = RankedSpecies::from(&s0);
            let spec1 = RankedSpecies::from(&s1);
            assert!(spec0 == spec1);
        }

        s0.grow().unwrap();
        {
            let spec0 = RankedSpecies::from(&s0);
            let spec1 = RankedSpecies::from(&s1);
            assert!(spec0 < spec1);
        }

        s1.grow().unwrap();
        {
            let spec0 = RankedSpecies::from(&s0);
            let spec1 = RankedSpecies::from(&s1);
            assert!(spec0 == spec1);
        }
    }
}
