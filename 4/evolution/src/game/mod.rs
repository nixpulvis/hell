use std::collections::{HashMap, HashSet};
// TODO: <refactor> Shouldn't need this if we make new polymorphic.
use evolution_wire::Channel;
use ext::Dequeue;
use interact::*;
use object::*;
use silly::*;

/// A unique identifier of a player in the game.
pub type Id = u64;

/// The minimum number of players allowed to play evolution at once.
pub const MIN_PLAYERS: usize = 3;

/// The maximum number of players allowed to play evolution at once.
pub const MAX_PLAYERS: usize = 8;

/// A game of evolution that contains all the players, the board, and the
/// deck of cards.
///
/// All player ids must be unique. When a game is created the players must
/// have an empty domain, hand, and food bag.
///
/// There are two notions of ordering for players in the game. Round order, and
/// turn order.
///
/// ### Round Order
///
/// The starting player of each round is the first element in the `players`.
/// This value will be rotated each round such that second player in the vector
/// is the starting player.
///
/// ### Turn Order
///
/// The current player represents the next player to make a choice. After a
/// choice has been made, the current player is updated to the next player
/// who is not being skipped for this round. At the start of a round, no
/// players are set to be skipped.
///
/// The deck must conatin no more than 17 carnivore trait cards and no more
/// than 7 of each non-carnivore trait card.
///
/// # Examples
///
/// ```rust
/// use evolution::Game;
/// use evolution::silly::Silly;
///
/// // Create a game.
/// let mut game = Game::<Silly>::new(4).unwrap();
/// // Play the game.
/// game.play();
/// ```
#[derive(Debug)]
pub struct Game<C: Chooser> {
    players: Vec<Player>,
    choosers: HashMap<Id, C>,
    current_player: Option<usize>,
    skip_set: HashSet<usize>,
    board: Board,
    deck: Vec<Card>,
}

/// Implementation of `Game` that communicates with clients over TCP using `Chanel`s.
impl Game<Channel> {
    /// Creates a new instance of a `Game` object. Also allocates a `Player` for every `Channel`
    /// supplied.
    ///
    /// # Returns
    ///
    /// This function returns an `Err` result if the number of players supplied is not legal to
    /// start a game with, otherwise, it returns an `Ok` result containing the `Game`.
    pub fn new(channels: Vec<Channel>) -> Result<Self, ()> {
        let mut players = Vec::new();
        let mut choosers = HashMap::new();

        for (i, channel) in channels.into_iter().enumerate() {
            let id = (i + 1) as u64;
            players.push(Player::new(id));
            choosers.insert(id, channel);
        }

        if players.len() < MIN_PLAYERS || players.len() > MAX_PLAYERS {
            return Err(())
        }

        let current_player = Some(0);

        Ok(Game {
            board: Board::default(),
            players: players,
            choosers: choosers,
            current_player: current_player,
            skip_set: HashSet::default(),
            deck: Card::deck(),
        })
    }
}

/// Implementation of `Game` that communicates internally with `Silly` player instances.
impl Game<Silly> {
    /// Creates a new game with the specified number of `Player`s, all represented by the `Silly`
    /// strategy implementation.
    ///
    /// # Returns
    ///
    /// This function returns an `Err` result if the number of players supplied is not legal to
    /// start a game with, otherwise, it returns an `Ok` result containing the `Game`.
    pub fn new(n: usize) -> Result<Self, ()> {
        let mut players = Vec::new();
        let mut choosers = HashMap::new();

        for i in 0..n {
            let id = (i + 1) as u64;
            players.push(Player::new(id));
            choosers.insert(id, Silly);
        }

        if players.len() < MIN_PLAYERS || players.len() > MAX_PLAYERS {
            return Err(())
        }

        let current_player = Some(0);

        Ok(Game {
            board: Board::default(),
            players: players,
            choosers: choosers,
            current_player: current_player,
            skip_set: HashSet::default(),
            deck: Card::deck(),
        })
    }
}

/// General functions.
impl<C: Chooser> Game<C> {
    /// Writes the scores of all players in the game to standard out.
    pub fn print_scores(&self) {
        let mut players_ref: Vec<&Player> = self.players.iter().collect();
        players_ref.sort_by(|a, b| b.score().cmp(&a.score()));
        for (i, player) in players_ref.into_iter().enumerate() {
            let chooser = self.choosers.get(&player.id()).expect("failed to get chooser");
            let score_message = match chooser.info() {
                Some(info) => format!("{} player id: {} ({}) score: {}",
                    i + 1,
                    player.id(),
                    info,
                    player.score()),
                None => format!("{} player id: {} score: {}", i + 1, player.id(), player.score())
            };
            println!("{}", score_message);
            // println!("{} player id: {} score {}", i + 1, player.id(), player.score());
        }
    }
}

/// Board functions.
impl<C: Chooser> Game<C> {
    /// Returns a reference to the game board.
    pub fn board(&self) -> &Board {
        &self.board
    }

    /// Returns a mutable reference to the game board.
    pub fn board_mut(&mut self) -> &mut Board {
        &mut self.board
    }
}

/// Player functions.
impl<C: Chooser> Game<C> {
    /// Returns the players of this game in the order they are playing.
    pub fn players(&self) -> &[Player] {
        &self.players
    }

    /// Returns mutable references to the players of this game in the order
    /// they are playing.
    pub fn players_mut(&mut self) -> &mut [Player] {
        &mut self.players
    }

    /// Returns the index of the player in this game representing the current
    /// player in this round's turn order. This function returns `None` if
    /// there is no longer a current player because all players are skipped.
    pub fn current_player(&self) -> &Player {
        &self.players[self.current_player.expect("no current player")]
    }

    pub fn current_player_idx(&self) -> usize {
        self.current_player.expect("no current player")
    }

    /// Shifts the current starting player to the last position, setting the
    /// next player immediately after to be the new starting player.
    /// This function is intended to be called at round start, so it will
    /// also clear the collection of skipped players.
    pub fn advance_starting_player(&mut self) {
        if self.players.len() > 1 {
            let predecessor = self.players.remove(0);
            self.players.push(predecessor);
        }
        self.current_player = Some(0);
        self.skip_set.clear();
    }

    /// Advances the current player by one, wrapping to the start of the
    /// ordering if the last player is current.
    ///
    /// ### Panics
    ///
    /// Panics if all players are currently skipped.
    pub fn advance_current_player(&mut self) {
        if self.skip_set.len() >= self.players.len() {
            self.current_player = None;
            return
        }
        if let Some(index) = self.current_player {
            let index = (index + 1) % self.players.len();
            self.current_player = Some(index);
            if self.skip_set.contains(&index) {
                self.advance_current_player();
            }
        } else {
            panic!("no current player")
        }
    }

    /// Marks the current player as skipped and advances the current player
    /// by one, wrapping to the start of the ordering if the last player is
    /// current.
    ///
    /// ### Panics
    ///
    /// Panics if all players are currently skipped.
    pub fn skip_advance_current_player(&mut self) {
        if let Some(index) = self.current_player {
            self.skip_set.insert(index);
            self.advance_current_player();
        } else {
            panic!("no current player")
        }
    }

    /// Ejects the current player from the game.
    pub fn eject_current_player(&mut self) {
        warn!("Ejecting player {}.", self.current_player().id());
        let idx = self.current_player_idx();
        self.players.remove(idx);
    }

    /// Returns `true` if not players are in the skip set.
    pub fn turn_is_over(&self) -> bool {
        self.board().food().len() == 0 ||
        self.skip_set.len() >= self.players.len()
    }

    /// Returns `true` if there are no longer enough cards remaining to deal to all remaining
    /// players, or if every player has been kicked from the game.
    pub fn is_over(&self) -> bool {
        self.deck().len() < (self.players().len() * 4) as usize ||
        self.players().len() <= 1
    }
}

/// Deck functions.
impl<C: Chooser> Game<C> {
    /// Returns a reference to the deck of this game.
    pub fn deck(&self) -> &[Card] {
        &self.deck
    }

    /// Returns a mutable reference to the deck of this game.
    pub fn deck_mut(&mut self) -> &mut [Card] {
        &mut self.deck
    }

    /// Attempts to remove a single card from the top of the deck, optionally returning the `Card`
    /// if successful.
    ///
    /// # Returns
    ///
    /// `Some(Card)` if successful, `None` otherwise.
    pub fn deal(&mut self) -> Option<Card> {
        self.deck.dequeue()
    }

    /// Deals a given number of cards into a `Vec`. Each card is taken from
    /// the top of the deck, and pushed to the end of the `Vec`.
    ///
    /// Builds a `Vec` containing up to the given number of cards, and will
    /// only return less cards if the deck is empty.
    pub fn deals(&mut self, number: usize) -> Vec<Card> {
        let mut cards = vec![];
        for _ in 0..number {
            match self.deal() {
                Some(card) => cards.push(card),
                None => break,
            }
        }
        cards
    }
}

/// Step functions.
impl<C: Chooser> Game<C> {
    /// Plays through an entire game of Evolution until one of the stop criteria are met.
    pub fn play(&mut self) {
        unsafe {
            let game: *mut Game<C> = self;

            'game: while !(&*game).is_over() {
                info!("New round the starting player is {:?}.", (*game).players()[0].id());
                info!("The rotation is: {:?}", (*game).players().iter().map(|p| p.id()).collect::<Vec<_>>());

                self.step_deal().expect("deal failed");

                'action_steps: for _ in (&*game).players() {
                    if (*game).current_player.is_some() {
                        self.step_action().expect("action failed");
                    } else {
                        break 'action_steps
                    }
                }

                self.step_reveal().expect("reveal failed");

                'feed_steps: while !(&*game).turn_is_over() {
                    if (*game).current_player.is_some() {
                        self.step_feed().expect("feed failed");
                    } else {
                        break 'feed_steps
                    }
                }

                self.step_bag().expect("bag failed");
            }
        }
    }

    /// Executes the dealing step, providing all players with cards and species boards at the
    /// start of a round, as specified in the rules of the Evolution game.
    // TODO: Rename /step_//s
    pub fn step_deal(&mut self) -> Result<(), ()> {
        info!("Dealing.");
        try!(step::Deal(self).step());
        // NOTE: I need to learn to deal with mutable loops in Rust.
        for i in 0..self.players().len() {
            let player = self.players()[i].clone(); // HACK: clone to get around self borrow.
            let board = self.board().clone();
            let chooser = self.choosers.get_mut(&player.id()).expect("failed to get chooser");
            chooser.start(&(board, player).observe());
        }
        Ok(())
    }

    /// Executes the action step, requesting an exchange of cards from the current player, and
    /// applying the necessary modifications to the player's species boards.
    ///
    /// # Note
    ///
    /// Players who submit invalid choices are considered cheaters and will be ejected during this
    /// step.
    // TODO: Rename /step_//s
    pub fn step_action(&mut self) -> Result<(), ()> {
        info!("Action turn for player {}.", self.current_player().id());
        // HACK: <refactor> We really want a dealer and a game state, to
        // seperate these components. The game could do this all, but we'd
        // need to figure out another way to pass an arbitrary `Choose` to a
        // step for test harnesses.
        let mut chooser = unsafe {
            let id = self.current_player().id();
            &mut *(self.choosers.get_mut(&id).expect("failed to get chooser") as *mut C)
        };
        step::Action(self, &mut Auto(chooser)).step()
    }

    /// Executes the reveal step, turning over all cards given to the watering hole to be used as
    /// food. This step also triggers automatic feed and breed operations associated with certain
    /// species' traits.
    // TODO: Rename /step_//s
    pub fn step_reveal(&mut self) -> Result<(), ()> {
        info!("Revealing board.");
        let result = step::Reveal(self).step();
        info!("Food count is now {}", self.board().food().len());
        result
    }

    /// Executes the feeding step, requesting and applying valid feeding choices the current
    /// player if food is available at the watering hole. Feeding will also automatically select a
    /// valid choice for a player if (1) it is the player's *only* valid choice, **and** (2) the
    /// choice is *not* a self-targeted attack.
    ///
    /// # Note
    ///
    /// Players who submit invalid choices are considered cheaters and will be ejected during this
    /// step.
    // TODO: Rename /step_//s
    pub fn step_feed(&mut self) -> Result<(), ()> {
        info!("Feed turn for player {}.", self.current_player().id());
        // HACK: <refactor> See comment in `step_action`.
        let mut chooser = unsafe {
            let id = self.current_player().id();
            &mut *(self.choosers.get_mut(&id).expect("failed to get chooser") as *mut C)
        };
        step::Feed(self, &mut Auto(chooser)).step()
    }

    /// Moves the food tokens on each species' boards to their owning players' food bag, securing
    /// that food as part of the respective player's score.
    // TODO: Rename /step_//s
    pub fn step_bag(&mut self) -> Result<(), ()> {
        info!("Bagging.");
        step::Bag(self).step()
    }
}

/// Channel choices.
mod channel;

/// Auto choices.
mod auto;
pub use self::auto::Auto;

/// Objects with knowledge of the game.
pub mod context;
pub use self::context::Context;

/// Steps of playing the game.
pub mod step;
pub use self::step::Step;

#[cfg(feature = "wire")]
mod wire;

/// Return a game with the given number of players, where each player was
/// given to a function for setting them up.
// TODO: Delete this weird test helper.
#[cfg(test)]
pub fn game_with_players(players: usize, each_player: &Fn(&mut Player)) -> Game<Silly> {
    let mut game = Game::<Silly>::new(players).expect("given invalid number of players");
    for player in game.players_mut() {
        each_player(player);
    }
    game
}

#[cfg(test)]
mod tests {
    use game::*;
    use silly::*;

    #[test]
    fn too_few_players() {
        assert!(Game::<Silly>::new(2).is_err());
    }

    #[test]
    fn too_many_players() {
        assert!(Game::<Silly>::new(9).is_err());
    }

    #[test]
    fn current_player() {
        let game = Game::<Silly>::new(3).unwrap();

        assert_eq!(1, game.current_player().id());
    }

    #[test]
    fn advance_current_player() {
        let mut game = Game::<Silly>::new(4).unwrap();

        assert_eq!(0, game.current_player_idx());

        game.advance_current_player();

        assert_eq!(1, game.current_player_idx());
    }

    #[test]
    fn advance_current_player_skips() {
        let mut game = Game::<Silly>::new(5).unwrap();
        game.skip_set.insert(2);
        game.skip_set.insert(3);
        game.advance_current_player();

        assert_eq!(1, game.current_player_idx());

        game.advance_current_player();

        assert_eq!(4, game.current_player_idx());
    }

    #[test]
    fn skip_advance() {
        let mut game = Game::<Silly>::new(4).unwrap();
        game.skip_set.insert(1);

        assert_eq!(0, game.current_player_idx());

        game.skip_advance_current_player();

        assert!(game.skip_set.contains(&0));
        assert!(game.skip_set.contains(&1));
        assert_eq!(2, game.current_player_idx());
    }

    #[test]
    fn advance_starting_player() {
        let mut game = Game::<Silly>::new(4).unwrap();
        game.advance_current_player();
        game.advance_current_player();

        assert_eq!(1, game.players[0].id());
        assert_eq!(2, game.players[1].id());
        assert_eq!(3, game.players[2].id());
        assert_eq!(4, game.players[3].id());
        assert_eq!(2, game.current_player_idx());

        game.advance_starting_player();

        assert_eq!(2, game.players[0].id());
        assert_eq!(3, game.players[1].id());
        assert_eq!(4, game.players[2].id());
        assert_eq!(1, game.players[3].id());
        assert_eq!(0, game.current_player_idx());
    }

    #[test]
    fn advance_starting_player_also_clears_skip_set() {
        let mut game = Game::<Silly>::new(4).unwrap();
        game.skip_advance_current_player();
        game.skip_advance_current_player();

        assert!(game.skip_set.contains(&0));
        assert!(game.skip_set.contains(&1));

        game.advance_starting_player();

        assert!(game.skip_set.is_empty());
    }

    #[test]
    fn advance_starting_does_nothing_without_players() {
        let mut game = Game::<Silly>::new(4).unwrap();
        game.players.clear();

        assert!(game.players().is_empty());

        game.advance_starting_player();
    }

    #[test]
    fn eject_current_player() {
        let mut game = Game::<Silly>::new(4).unwrap();

        assert_eq!(4, game.players().len());
        assert!(game.players().iter().map(|p| p.id()).collect::<Vec<_>>().contains(&1));

        game.eject_current_player();

        assert_eq!(3, game.players().len());
        assert!(!game.players().iter().map(|p| p.id()).collect::<Vec<_>>().contains(&1));
    }

    #[test]
    fn eject_current_player_advances_the_current_player() {
        let mut game = Game::<Silly>::new(4).unwrap();

        assert_eq!(4, game.players().len());
        assert_eq!(1, game.current_player().id());

        game.eject_current_player();

        assert_eq!(3, game.players().len());
        assert_eq!(2, game.current_player().id());
    }

    #[test]
    fn eject_current_player_wraps_index() {
        let mut game = Game::<Silly>::new(3).unwrap();
        game.advance_starting_player();
        game.advance_starting_player();

        assert_eq!(3, game.players().len());
        assert_eq!(3, game.current_player().id());

        game.eject_current_player();

        assert_eq!(2, game.players.len());
        assert_eq!(1, game.current_player().id());

        game.eject_current_player();

        assert_eq!(1, game.players().len());
        assert_eq!(2, game.current_player().id());
    }

    #[test]
    #[ignore]
    fn turn_is_over() {
        unimplemented!()
    }

    #[test]
    fn deals_correct_number_of_cards() {
        let mut game = game_with_players(3, &|_| {});

        assert!(game.deals(0).is_empty());
        assert_eq!(1, game.deals(1).len());
        assert_eq!(5, game.deals(5).len());
    }

    #[test]
    fn deals_cannot_take_more_than_deck() {
        let mut game = game_with_players(3, &|_| {});
        let amount = 200;

        assert!(game.deck().len() < amount);
        assert!(game.deals(amount).len() < amount);
    }
}
