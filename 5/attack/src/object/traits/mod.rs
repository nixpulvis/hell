use std::cmp::Ordering;

/// A trait which a trait card may have. These are modifiers for
/// how a species board acts.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Trait {
    /// Carnivore must attack to eat during the feeding stage.
    Carnivore,
    /// Ambush overcomes a Warning Call during an attack.
    Ambush,
    /// Burrowing deflects an attack when it has a food supply equal to its
    /// population size.
    Burrowing,
    /// Climbing prevents an attack unless the Carnivore also has the
    /// Climbing attribute.
    Climbing,
    /// Cooperation feeds the species to its right one token of food every
    /// time it eats (taken from the common food supply).
    Cooperation,
    /// Fat Tissue allows a species to store as many food tokens as its
    /// body size count.
    FatTissue,
    /// Fertile adds one animal to the population when the food cards
    /// are revealed.
    Fertile,
    /// Foraging enables this species to eat two tokens of food for
    /// every feeding.
    Foraging,
    /// Hard Shell prevents an attack unless the attacker is at least
    /// 4 units larger than this species in body size.
    HardShell,
    /// Herding stops attacks from Carnivore species whose populations
    /// are smaller or equal in size to this species’ population.
    Herding,
    /// Horns kills one animal of an attacking Carnivore species.
    Horns,
    /// Long Neck adds one food token when the food cards are revealed.
    LongNeck,
    /// Pack Hunting adds this species’ population size to its body
    /// size for attacks on other species.
    PackHunting,
    /// Scavenger eats one food token every time a Carnivore eats
    /// another species.
    Scavenger,
    /// Symbiosis prevents an attack if this species has a neighbor
    /// to the right whose body size is larger than this one’s.
    Symbiosis,
    /// Warning Call prevents an attack from a Carnivore on both
    /// neighboring species unless the attacker has the Ambush
    /// property.
    WarningCall,
}

impl Ord for Trait {
    fn cmp(&self, other: &Trait) -> Ordering {
        // NOTE: This is "ok" because the strings `Debug` will generate are
        // ordered lexicographically the same as the wire string
        // representation.
        format!("{:?}", self).cmp(&format!("{:?}", other))
    }
}

impl PartialOrd for Trait {
    fn partial_cmp(&self, other: &Trait) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(feature = "wire")]
mod wire;

#[cfg(test)]
mod tests {
    use object::*;

    #[test]
    fn lexicographical_order() {
        assert!(Trait::Carnivore < Trait::WarningCall);
        assert!(Trait::Cooperation > Trait::Climbing);
        assert!(Trait::FatTissue >= Trait::FatTissue);
        assert!(Trait::FatTissue <= Trait::FatTissue);
    }
}
