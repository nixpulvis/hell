use itertools::Itertools;
use board::Board;
use configuration::Configuration;
use dealer::Dealer;
use player::Player;

/// Running of the game of take5.
pub struct Game {
    pub board: Board,
    pub dealer: Box<Dealer>,
    pub players: Vec<Box<Player>>,
}

impl Game {
    /// Creates a new game, parsing arguments from the command line.
    pub fn new(dealer: Box<Dealer>, players: Vec<Box<Player>>) -> Self {
        let board = Board::default();
        Game {
            board: board,
            dealer: dealer,
            players: players,
        }
    }

    /// Run the game.
    pub fn run(&mut self) {
        // The rounds.
        while self.players.iter().all(|p| p.score() < Configuration::win_score()) || self.players[0].score() == self.players[1].score() {
            self.dealer.open();
            self.dealer.deals(&mut self.players);
            self.dealer.flip(&mut self.board);

            // The turns.
            for _ in 0..Configuration::turn_count() {
                let mut played = Vec::new();
                for player in self.players.iter_mut() {
                    player.look_at_board(&mut self.board);
                    let card = player.play();
                    played.push((card, player.id()));
                }

                for player in self.players.iter_mut() {
                    player.look_at_played(&played);
                }

                self.dealer.stack(&mut self.board, played, &mut self.players);
            }

            self.players.sort_by(|a, b| a.score().cmp(&b.score()));
        }
    }

    /// Report on the game to stdout.
    pub fn report(&mut self) {
        println!("Game ended with: {}",
                 self.players.iter().map(|p| format!("{} at {}", p, p.score())).join(", "));
    }
}
