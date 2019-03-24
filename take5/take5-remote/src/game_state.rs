use error::Error;
use message::RequestMessage;

/// Represents the possible states a client can be in while playing
/// take5.
///
/// The three states here are the complete set of states for the
/// client. If the protocol ever gets setup, and teardown then
/// more states will be added. The game state determains the
/// possible valid next messages.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum GameState {
    /// Expects a `start-round` message next.
    StartingRound,
    /// Expects a `take-turn` message next.
    FirstTurn,
    /// Expects either a `take-turn` or `choose` message next. This varient's
    /// value must be in the range `[1, 9]`, because the 0th turn is covered
    /// by the `FirstTurn` varient, and there are a total of 10 turns.
    /// The boolean represents whether or not you have chosen already.
    Turns(usize, bool),
    /// State for immediately after the last card was played for a round.
    /// Expects either a `choose` message, or a `start-round`.
    TookLastTurn,
}

impl GameState {
    /// Given a message, returns the next state for the game. If the
    /// message is not valid for the current state of the game this
    /// function returns `Err`.
    pub fn next_state(&self, message: &RequestMessage) -> Result<GameState, Error> {
        match (self, message) {
            (&GameState::StartingRound, &RequestMessage::StartRound(_)) => {
                Ok(GameState::FirstTurn)
            },
            (&GameState::FirstTurn, &RequestMessage::TakeTurn(_)) => {
                Ok(self.take_turn(0))
            },
            (&GameState::Turns(n, _), &RequestMessage::TakeTurn(_)) => {
                Ok(self.take_turn(n))
            },
            (&GameState::Turns(n, false), &RequestMessage::Choose(_)) => {
                Ok(GameState::Turns(n, true))
            },
            (&GameState::TookLastTurn, &RequestMessage::StartRound(_)) => {
                Ok(GameState::FirstTurn)
            },
            (&GameState::TookLastTurn, &RequestMessage::Choose(_)) => {
                Ok(GameState::StartingRound)
            },
            _ => {
                let s = format!("no valid next state for {:?} from {:?}", message, self);
                Err(Error::GameState(s))
            },
        }
    }

    fn take_turn(&self, taken: usize) -> GameState {
        // We are going to take the turn, and be in the starting round state.
        if taken == 9 {
            GameState::TookLastTurn
        } else {
            GameState::Turns(taken + 1, false)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use message::RequestMessage;
    use starting_hand::StartingHand;
    use wrapper::{Card, Stack, Board};

    fn start_round_msg() -> RequestMessage {
        let mut deck = Card::deck(|_| 2);
        deck.split_off(10);
        let hand = StartingHand::new(deck).unwrap();
        RequestMessage::StartRound(hand)
    }

    fn take_turn_msg() -> RequestMessage {
        let mut deck = Card::deck(|_| 2);
        let stacks = [
            Stack::new(vec![deck.remove(0)]),
            Stack::new(vec![deck.remove(0)]),
            Stack::new(vec![deck.remove(0)]),
            Stack::new(vec![deck.remove(0)]),
        ];

        RequestMessage::TakeTurn(Board::new(stacks))
    }

    fn choose_msg() -> RequestMessage {
        let mut deck = Card::deck(|_| 2);
        let stacks = [
            Stack::new(vec![deck.remove(0)]),
            Stack::new(vec![deck.remove(0)]),
            Stack::new(vec![deck.remove(0)]),
            Stack::new(vec![deck.remove(0)]),
        ];

        RequestMessage::Choose(Board::new(stacks))
    }

    #[test]
    fn test_transition_from_start_to_first() {
        let state = GameState::StartingRound;
        assert_eq!(GameState::FirstTurn, state.next_state(&start_round_msg()).unwrap());
    }

    #[test]
    fn test_transition_from_first_to_turns() {
        let state = GameState::FirstTurn;
        assert_eq!(GameState::Turns(1, false), state.next_state(&take_turn_msg()).unwrap());
    }

    #[test]
    fn test_transition_from_turns_with_choice_to_turns() {
        let state = GameState::Turns(8, false);
        assert_eq!(GameState::Turns(8, true), state.next_state(&choose_msg()).unwrap());
    }

    #[test]
    fn test_transition_from_turns_with_take_turn_to_turns() {
        let state = GameState::Turns(8, false);
        assert_eq!(GameState::Turns(9, false), state.next_state(&take_turn_msg()).unwrap());
    }

    #[test]
    fn test_transition_from_last_turn_to_first_turn_new_round() {
        let state = GameState::TookLastTurn;
        assert_eq!(GameState::FirstTurn, state.next_state(&start_round_msg()).unwrap());
    }

    #[test]
    fn test_10_transitions_from_first_turn_to_starting_round() {
        let ref mut state = GameState::FirstTurn;
        // 9 transitions into turns.
        for i in 0..9 {
            *state = state.next_state(&take_turn_msg()).unwrap();
            assert_eq!(GameState::Turns(1 + i, false), *state);
        }
        *state = state.next_state(&take_turn_msg()).unwrap();
        assert_eq!(GameState::TookLastTurn, *state);

        // 1 transition to starting round.
        let state = state.next_state(&choose_msg()).unwrap();
        assert_eq!(GameState::StartingRound, state);
    }

    #[test]
    fn test_bad_transition_from_starting_round() {
        let state = GameState::StartingRound;
        assert!(state.next_state(&take_turn_msg()).is_err());
        assert!(state.next_state(&choose_msg()).is_err());
    }

    #[test]
    fn test_bad_transition_from_first_turn() {
        let state = GameState::FirstTurn;
        assert!(state.next_state(&choose_msg()).is_err());
        assert!(state.next_state(&start_round_msg()).is_err());
    }

    #[test]
    fn test_bad_transition_from_turns_with_remaining() {
        let state = GameState::Turns(8, false);
        assert!(state.next_state(&start_round_msg()).is_err());
    }

    #[test]
    fn test_choose_on_last_turn() {
        let ref mut state = GameState::Turns(9, false);
        *state = state.next_state(&take_turn_msg()).unwrap();
        assert!(state.next_state(&choose_msg()).is_ok());
    }

    #[test]
    fn test_double_choose() {
        let ref mut state = GameState::Turns(5, false);
        *state = state.next_state(&choose_msg()).unwrap();
        assert!(state.next_state(&choose_msg()).is_err())
    }
}
