use object::*;

/// An owned board with food tokens, and cards which were played as food for
/// the round.
#[derive(Debug, Default, Clone)]
pub struct Board {
    food: Vec<FoodToken>,
    cards: Option<Vec<Card>>,
}

/// Food functions.
impl Board {
    /// Returns the amount of food available on the board. This is the
    /// watering hole value as described in the spec.
    ///
    /// # Arguments
    ///
    /// * `&self` - The board to get food from.
    ///
    /// # Returns
    ///
    /// An immutable slice of type `&[FoodToken]`, containing the food
    /// tokens on the board currently.
    pub fn food(&self) -> &[FoodToken] {
        &self.food
    }

    /// Move a token onto the board.
    ///
    /// # Arguments
    ///
    /// * `&mut self` - The board taking food.
    /// * `food_token` - A food token for the board to take.
    pub fn push_food(&mut self, food_token: FoodToken) {
        self.food.push(food_token);
    }

    /// Move many tokens onto the board.
    ///
    /// # Arguments
    ///
    /// * `&mut self` - The board taking food.
    /// * `food_tokens` - A vector of food tokens for the board to take.
    pub fn push_foods(&mut self, food_tokens: Vec<FoodToken>) {
        self.food.extend(food_tokens);
    }

    /// Get a single food token, if available. An empty watering hole will
    /// return `None`.
    ///
    /// # Arguments
    ///
    /// * `&mut self` - The board to giving food.
    ///
    /// # Return
    ///
    /// This function returns a food token from the board as `Some` or `None`
    /// if there are no more food tokens left.
    pub fn pop_food(&mut self) -> Option<FoodToken> {
        self.food.pop()
    }

    /// Removes many food tokens from the board. This function returns
    /// `None` if there are no food tokens to take. If this function is
    /// asked for more food than the board has this function will return
    /// as much as it can.
    // TODO: The fact that this method returns as much as it can makes me
    // think we should do this for all pop_*s functions, and that the
    // `Option<...>` type is not correct.
    pub fn pop_foods(&mut self, requested: u64) -> Option<Vec<FoodToken>> {
        let split = self.food.len().saturating_sub(requested as usize);
        if split != self.food.len() {
            Some(self.food.split_off(split))
        } else {
            None
        }
    }
}

/// Card functions
impl Board {
    /// Get the cards used as food for this round.
    ///
    /// # Arguments
    ///
    /// * `&self` - The board we are asking for the cards of.
    ///
    /// # Return
    ///
    /// This function returns `None` before the cards are flipped. Cards are
    /// flipped after the play phase but before the eat phase, when flipped
    /// then this function returns a slice of the cards.
    pub fn cards(&self) -> Option<&[Card]> {
        self.cards.as_ref().map(|v| v.as_slice())
    }

    /// Get the cards used as food for this round.
    ///
    /// # Arguments
    ///
    /// * `&mut self` - The board we are asking for the cards of.
    ///
    /// # Return
    ///
    /// This function returns `None` before the cards are flipped. Cards are
    /// flipped after the play phase but before the eat phase, when flipped
    /// then this function returns a slice of the cards.
    pub fn cards_mut(&mut self) -> Option<&mut [Card]> {
        self.cards.as_mut().map(|v| v.as_mut_slice())
    }

    /// Adds a single card to the board to be used as food.
    ///
    /// # Arguments
    ///
    /// * `&mut self` - The board to add a card to.
    /// * `card` - The card to add to the board.
    pub fn add_card(&mut self, card: Card) {
        if let Some(ref mut cards) = self.cards {
            cards.push(card);
        } else {
            self.set_cards(vec![card]);
        }
    }

    /// Set the cards used as food for this round.
    ///
    /// # Arguments
    ///
    /// * `&mut self` - The board to set cards for.
    ///
    /// # Panics
    ///
    /// This function will panic if there are already cards on the board.
    pub fn set_cards(&mut self, cards: Vec<Card>) {
        assert!(self.cards.is_none());
        self.cards = Some(cards);
    }

    /// Clears the cards used as food for this round.
    ///
    /// # Arguments
    ///
    /// * `&mut self` - The board to clear cards off of.
    pub fn clear_cards(&mut self) {
        self.cards = None;
    }
}

#[cfg(feature = "wire")]
mod wire;

#[cfg(test)]
mod tests {
    use object::*;

    #[test]
    fn push_and_pop_foods_some() {
        let mut board = Board::default();
        board.push_foods(vec![FoodToken, FoodToken, FoodToken]);

        assert_eq!(vec![FoodToken, FoodToken, FoodToken], board.food());

        assert_eq!(Some(vec![FoodToken]), board.pop_foods(1));
        assert_eq!(vec![FoodToken, FoodToken], board.food());
    }

    #[test]
    fn push_and_pop_foods_all() {
        let mut board = Board::default();
        board.push_foods(vec![FoodToken, FoodToken, FoodToken]);

        assert_eq!(vec![FoodToken, FoodToken, FoodToken], board.food());

        assert_eq!(Some(vec![FoodToken, FoodToken, FoodToken]), board.pop_foods(3));
        assert!(board.food().is_empty());
    }

    #[test]
    fn push_and_pop_foods_none() {
        let mut board = Board::default();

        assert_eq!(Vec::<FoodToken>::new(), board.food());

        assert_eq!(None, board.pop_foods(1));
    }

    #[test]
    fn pop_too_many() {
        let mut board = Board::default();
        board.push_foods(vec![FoodToken]);

        assert_eq!(Some(vec![FoodToken]), board.pop_foods(4));
    }

    #[test]
    fn set_cards_for_empty_board() {
        let mut board = Board::default();

        assert!(board.cards().is_none());

        board.set_cards(vec![
            Card::mock(3, Trait::Burrowing),
            Card::mock(2, Trait::Climbing),
        ]);

        assert_eq!(2, board.cards().unwrap().len());
    }

    #[test]
    #[should_panic]
    fn set_cards_for_non_empty_board() {
        let mut board = Board::default();
        board.set_cards(vec![Card::mock(3, Trait::LongNeck)]);

        assert_eq!(1, board.cards().unwrap().len());

        board.set_cards(vec![Card::mock(7, Trait::Carnivore)]);
    }

    #[test]
    fn clear_cards() {
        let mut board = Board::default();
        board.set_cards(vec![Card::mock(3, Trait::Foraging)]);

        assert!(board.cards.is_some());

        board.clear_cards();

        assert!(board.cards.is_none());
    }

    #[test]
    fn add_card_to_empty_board() {
        let mut board = Board::default();

        assert!(board.cards().is_none());

        board.add_card(Card::mock(2, Trait::Ambush));

        assert_eq!(1, board.cards().unwrap().len());
    }

    #[test]
    fn add_card_to_non_empty_board() {
        let mut board = Board::default();
        board.set_cards(vec![Card::mock(2, Trait::HardShell)]);

        assert_eq!(1, board.cards().unwrap().len());

        board.add_card(Card::mock(7, Trait::Carnivore));

        assert_eq!(2, board.cards().unwrap().len());
    }
}
