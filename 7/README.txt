Implement the feed function which takes a Feeding and decides which Species to feed
according to the given strategy.

xfeed - Shell script to ask for the feed choice of a given Feeding,
        reads the Feeding JSON from stdin

feeding/Cargo.lock - Rust cargo lock file for dependencies.
feeding/Cargo.toml - Top level crate manifest file.

feeding/src/lib.rs - Entry point to the crate, re-exports public interface.
feeding/src/machine - Docs for the game state machine.
feeding/src/actor/mod.rs - re-exports for any actors
feeding/src/actor/player.rs - Interface for a Player playing the game.
feeding/src/message/mod.rs - re-exports for all message types
feeding/src/message/card_choices.rs - Message type for how player use cards
feeding/src/message/eat_choice.rs - Message type for which species a player feeds.

feeding/src/state/board.rs - Data representation for a game board
feeding/src/state/card.rs - Data representation for a trait Card.
feeding/src/state/player.rs - Data representation for a Player.
feeding/src/state/traits.rs - Data representation for a game Trait.
feeding/src/state/species/mod.rs - Data representation for a species.
feeding/src/state/species/placement.rs -Data representation of how a player places a species.
feeding/src/state/species/situation.rs - Data representation of an attack Situation.

feeding/src/wire/** - Deserialization code for each of the data types above.

Streaming/Cargo.lock - Rust cargo lock file for dependencies.
Streaming/Cargo.toml - Top level crate manifest file.
Streaming/compile - Shell script for compiling the xstream binary.
Streaming/src/main - JSON echo program that reads from stdin.
Streaming/xstream - shell script to execute the xstream.rs binary.

Running the code:

-xstream
Navigate to the Streaming directory.
./compile will compile the binary
./xstream will run the program. This will quietly comiple the program if it has not
been done already.


- xfeed
To get the player's eat choice, run ./xfeed and supply the program with a Feeding.

Running the test suite:
The command ./test will run all unit tests and integration tests in the evolution library.