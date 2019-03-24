extern crate evolution;

use evolution::Game;
use evolution::silly::Silly;

fn main() {
    // Create a game.
    let mut game = Game::<Silly>::new(4).unwrap();
    // Play the game.
    game.play();

    println!("{:#?}", game);
}
