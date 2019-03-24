use take5::Player;
use message::{RequestMessage, ResponseMessage};
use starting_hand::StartingHand;
use wrapper::{Card, Board, Stack};

pub struct PlayerProxy {
	pub player: Box<Player>
}

impl PlayerProxy {
	pub fn process_message(&mut self, msg: RequestMessage) -> ResponseMessage {
        match msg {
			RequestMessage::StartRound(hand) => {
				self.start_round(hand);
				ResponseMessage::StartRound
			},
			RequestMessage::TakeTurn(board) => {
				ResponseMessage::TakeTurn(self.take_turn(board))
			},
			RequestMessage::Choose(board) => {
				ResponseMessage::Choose(self.choose(board))
			},
		}
	}

	fn start_round(&mut self, hand: StartingHand) {
		for card in hand.take() {
            self.player.draw(card.into());
		}
	}

	fn take_turn(&mut self, board: Board) -> Card {
		self.player.look_at_board(&*board);
		self.player.play().into()
	}

	fn choose(&mut self, board: Board) -> Stack {
		self.player.look_at_board(&*board);
		board.take(self.player.choose()).into()
	}
}
