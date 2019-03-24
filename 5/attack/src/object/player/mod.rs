use game::Id;
use object::*;

/// The internal state of a player, including private information.
#[derive(Debug, Clone)]
pub struct Player {
    id: Id,
    domain: Domain,
    bag: Vec<FoodToken>,
    hand: Vec<Card>,
}

/// General player functions.
impl Player {
    /// Create a new player with the given identifier. The player's domain will be
    /// empty.
    ///
    /// Note: A Player's id is not guarenteed to be unique.
    ///
    /// # Examples
    ///
    /// ```
    /// use evolution::object::*;
    ///
    /// let player = Player::new(2);
    /// assert_eq!(2, player.id());
    /// ```
    // TODO: This function should not allow 0 IDs.
    pub fn new(id: Id) -> Self {
        assert!(id != 0);

        Player {
            id: id,
            domain: Domain::default(),
            bag: Vec::new(),
            hand: Vec::new(),
        }
    }

    /// Returns a player's identifier.
    ///
    /// # Examples
    ///
    /// ```
    /// use evolution::object::*;
    ///
    /// let player = Player::new(2);
    /// assert_eq!(2, player.id());
    /// ```
    pub fn id(&self) -> Id {
        self.id
    }

    /// Returns the player's score.
    pub fn score(&self) -> u64 {
        (self.bag.len() as u64) +
        self.domain().iter().fold(0, |a, s| a + s.population()) +
        self.domain().iter().fold(0, |a, s| a + (s.traits().len() as u64))
    }
}

/// Domain functions.
impl Player {
    /// Returns a reference to the player's `Domain` containing their species.
    ///
    /// # Examples
    ///
    /// ```
    /// use evolution::object::*;
    ///
    /// let player = Player::new(2);
    /// assert_eq!(0, player.domain().len());
    /// ```
    pub fn domain(&self) -> &Domain {
        &self.domain
    }

    /// Returns a mutable reference to the player's domain.
    ///
    /// # Examples
    ///
    /// ```
    /// use evolution::object::*;
    ///
    /// let mut player = Player::new(2);
    /// player.domain_mut().add(Placement::Left);
    /// assert_eq!(1, player.domain().len());
    /// assert_eq!(1, player.domain()[0].population());
    /// ```
    pub fn domain_mut(&mut self) -> &mut Domain {
        &mut self.domain
    }
}

/// Bag functions.
impl Player {
    /// Returns a view of this player's food tokens.
    pub fn bag(&self) -> &[FoodToken] {
        &self.bag
    }


    /// Digests all of the food in this player's domain, and moves the food
    /// into the bag.
    pub fn bag_food(&mut self) {
        let food = self.domain_mut().take_food();
        self.bag.extend(food);
    }
}

/// Hand functions.
impl Player {
    /// Returns this player's hand.
    pub fn hand(&self) -> &[Card] {
        &self.hand
    }

    /// Accepts a single card into this player's hand, taking ownership of the card instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use evolution::object::*;
    /// use evolution::ext::*;
    ///
    /// // let mut player = Player::default();
    /// // assert_eq!(0, player.hand().len());
    /// // let mut deck = Card::deck();
    /// // let card = deck.dequeue().expect("unable to get the first card on the deck");
    /// // let (expected_food, expected_trait) = (card.food_value(), card.trait_type());
    /// // player.take_card(card);
    /// // assert_eq!(1, player.hand().len());
    /// // assert_eq!(expected_food, player.hand()[0].food_value());
    /// // assert_eq!(expected_trait, player.hand()[0].trait_type());
    /// ```
    pub fn push_card(&mut self, card: Card) {
        self.hand.push(card);
    }

    /// Adds the given cards to this player's hand.
    pub fn push_cards(&mut self, cards: Vec<Card>) {
        for card in cards.into_iter().rev() {
            self.hand.insert(0, card);
        }
    }

    /// Remove the given card from this player's hand, returning it.
    pub fn remove_card(&mut self, idx: usize) -> Card {
        self.hand.remove(idx)
    }
}

#[cfg(feature = "wire")]
mod wire;

#[cfg(test)]
mod tests {
    use object::*;

    #[test]
    fn push_card() {
        let mut player = Player::new(1);

        assert_eq!(0, player.hand().len());

        player.push_card(Card::mock(3, Trait::Ambush));

        assert_eq!(1, player.hand().len());
    }

    #[test]
    fn push_card_preserves_order() {
        let mut player = Player::new(1);

        assert!(player.hand().is_empty());

        player.push_card(Card::mock(1, Trait::Ambush));
        player.push_card(Card::mock(2, Trait::Burrowing));

        assert_eq!(1, player.hand()[0].food_value());
        assert_eq!(Trait::Ambush, player.hand()[0].trait_type());

        assert_eq!(2, player.hand()[1].food_value());
        assert_eq!(Trait::Burrowing, player.hand()[1].trait_type());
    }

    #[test]
    fn push_cards() {
        let mut player = Player::new(1);

        assert_eq!(0, player.hand().len());

        player.push_cards(vec![
            Card::mock(3, Trait::Ambush),
            Card::mock(3, Trait::Fertile),
        ]);

        assert_eq!(2, player.hand().len());
    }

    #[test]
    fn push_cards_preserves_order() {
        let mut player = Player::new(1);

        assert!(player.hand().is_empty());

        player.push_cards(vec![
            Card::mock(1, Trait::Ambush),
            Card::mock(2, Trait::Burrowing),
        ]);

        assert_eq!(1, player.hand()[0].food_value());
        assert_eq!(Trait::Ambush, player.hand()[0].trait_type());

        assert_eq!(2, player.hand()[1].food_value());
        assert_eq!(Trait::Burrowing, player.hand()[1].trait_type());
    }

    #[test]
    fn remove_card_non_empty() {
        let mut player = Player::new(1);
        player.push_cards(vec![
            Card::mock(3, Trait::Ambush),
            Card::mock(3, Trait::Fertile),
        ]);

        assert_eq!(2, player.hand().len());

        player.remove_card(0);
        assert_eq!(1, player.hand().len());

        player.remove_card(0);
        assert_eq!(0, player.hand().len());
    }

    #[test]
    #[should_panic]
    fn remove_card_empty() {
        let mut player = Player::new(1);

        assert_eq!(0, player.hand().len());

        player.remove_card(3);
    }

    #[test]
    fn bag() {
        let mut player = Player::new(1);
        player.domain_mut().add(Placement::Right);
        player.domain_mut().add(Placement::Right);
        player.domain_mut()[1].breed().unwrap();
        player.domain_mut()[0].eat(FoodToken).unwrap();
        player.domain_mut()[1].eat(FoodToken).unwrap();
        player.domain_mut()[1].eat(FoodToken).unwrap();

        assert_eq!(0, player.bag().len());

        player.bag_food();

        assert_eq!(3, player.bag().len());
    }
}
