use card::Card;
use board::Board;
use player::{Player, Id};

pub use self::standard_dealer::StandardDealer;

/// A dealer facilitates the game dealing and playing.
pub trait Dealer {
    /// Open a new deck of cards and shuffle it.
    ///
    /// # Examples
    ///
    /// ```
    /// use take5::dealer::{Dealer, StandardDealer};
    ///
    /// let mut dealer = StandardDealer::default();
    /// dealer.open();
    /// ```
    fn open(&mut self);

    /// Return the next card off of the deck.
    ///
    /// # Panics
    ///
    /// This function can panic if the dealer runs out of cards in the deck,
    /// or if the dealer never opened a deck of cards to start with.
    ///
    /// # Examples
    ///
    /// ```
    /// use take5::Configuration;
    /// use take5::dealer::{Dealer, StandardDealer};
    ///
    /// let mut dealer = StandardDealer::default();
    /// dealer.open();
    /// let card = dealer.deal();
    /// assert!(1 <= card.face() && card.face() <= Configuration::deck_size());
    /// ```
    fn deal(&mut self) -> Card;

    /// Deal out `Configuration::turn_count()` cards to all given players.
    ///
    /// # Panics
    ///
    /// This function can panic if the dealer runs out of cards in the deck,
    /// or if the dealer never opened a deck of cards to start with.
    ///
    /// # Examples
    ///
    /// ```
    /// use take5::Configuration;
    /// use take5::dealer::{Dealer, StandardDealer};
    /// use take5::player::{Player, AiPlayer};
    ///
    /// let mut dealer = StandardDealer::default();
    /// let mut players: Vec<Box<Player>> = (0..2).map(|id| {
    ///     Box::new(AiPlayer::new(id)) as Box<Player>
    /// }).collect();
    /// dealer.open();
    /// dealer.deals(&mut players);
    /// for player in players.iter() {
    ///     assert_eq!(player.hand().len(), Configuration::turn_count());
    /// }
    /// ```
    fn deals(&mut self, players: &mut [Box<Player>]);

    /// Clear and deal a card to each stack on the given board.
    ///
    /// # Panics
    ///
    /// This function can panic if the dealer runs out of cards in the deck,
    /// or if the dealer never opened a deck of cards to start with.
    ///
    /// # Examples
    ///
    /// ```
    /// use take5::Board;
    /// use take5::dealer::{Dealer, StandardDealer};
    ///
    /// let mut board = Board::default();
    /// let mut dealer = StandardDealer::default();
    /// dealer.open();
    /// dealer.flip(&mut board);
    /// ```
    fn flip(&mut self, board: &mut Board) {
        board.clear();
        for stack in board {
            let card = self.deal();
            stack.push(card);
        }
    }

    /// Stack the given played cards on the given board.
    ///
    /// Stacking cards is done in order of least face valued card to most
    /// valued. Cards are placed on to the stack with the largest face value
    /// which is smaller than the placed card. If this stack is full
    /// (has 5 cards) then the player takes that stack before placing the card
    /// onto the stack. If the played card's face value is smaller than all of
    /// the stacks then the player must *choose* the stack he/she wishes to
    /// take before placing the card into that stack.
    ///
    /// Whenever a player takes a stack, all players look at the taken stack.
    ///
    /// # Examples
    ///
    /// ```
    /// use take5::Board;
    /// use take5::dealer::{Dealer, StandardDealer};
    /// use take5::player::{Player, AiPlayer};
    ///
    /// let mut board = Board::default();
    /// let mut dealer = StandardDealer::default();
    /// let mut players: Vec<Box<Player>> = (0..2).map(|id| {
    ///     Box::new(AiPlayer::new(id)) as Box<Player>
    /// }).collect();
    /// dealer.open();
    /// dealer.flip(&mut board);
    /// let played = (0..2).map(|id| (dealer.deal(), id)).collect();
    /// dealer.stack(&mut board, played, &mut players);
    /// ```
    fn stack(&mut self,
             board: &mut Board,
             mut played: Vec<(Card, Id)>,
             players: &mut [Box<Player>]) {
        played.sort_by(|a, b| a.0.cmp(&b.0));

        for (card, id) in played.into_iter() {
            // Find the stack `card` should be put on, if it exists.
            let stack_id = board.closest_smaller(&card);

            match stack_id {
                // The player's card can be put on a stack.
                Some(i) => {
                    // If the stack is full, give the stack to the player.
                    if board[i].full() {
                        let cards = board[i].give();
                        for player in players.iter_mut() {
                            player.look_at_taken((&cards[..], id));
                        }
                        players.iter_mut()
                               .find(|p| p.id() == id)
                               .expect("player not found.")
                               .take(cards);
                    }

                    board[i].push(card);
                }
                // The player's card is smaller than each stack, and therefor
                // the player must choose a stack to take.
                None => {
                    let choice = {
                        let player = players.iter_mut()
                                            .find(|p| p.id() == id)
                                            .expect("player not found.");
                        player.look_at_board(&board);
                        player.choose()
                    };
                    let cards = board[choice].give();
                    for player in players.iter_mut() {
                        player.look_at_taken((&cards[..], id));
                    }
                    players.iter_mut()
                           .find(|p| p.id() == id)
                           .expect("player not found.")
                           .take(cards);

                    board[choice].push(card);
                }
            }
        }
    }
}

mod standard_dealer;
