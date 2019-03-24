use wrapper::{Card, Board, Stack};
use starting_hand::StartingHand;

/// A request message is sent from the server to the client, indicating
/// an action for the client to pass to the underlying player.
#[derive(Debug)]
pub enum RequestMessage {
    /// The first message of every round, containing the cards for
    /// a player's hand. This message always has enough cards for
    /// a player to ensure they can make
    /// `take5::Configuration::turn_count()` turns.
    StartRound(StartingHand),
    /// The first message after `StartRound`, requesting a client
    /// to play a card. The server passes the state of the board
    /// in this message.
    TakeTurn(Board),
    /// A message that *may* be sent after a player has recieved a
    /// `TakeTurn` message, and before a new round has started. This
    /// message also passes the state of the board, and the client
    /// must send back a stack the player wishes to choose.
    Choose(Board),
}

/// A message that is sent from the client in response to a valid
/// request. Here "valid" means both that the request was a valid
/// `RequestMessage`, and that it came at a valid time.
#[derive(Debug)]
pub enum ResponseMessage {
    /// The response to a new round starting. This message simply
    /// indicates to the server that the client accepted it's hand.
    StartRound,
    /// A message containing the card a player wishes to play.
    TakeTurn(Card),
    /// A message containing the stack a player wishes to choose.
    Choose(Stack),
}

mod de;
mod ser;
