Design the data representations for both the Species and Player for the game Evolution.  Implement the can_attack function which takes a Situation and decides if the attack is valid.

player-interface.txt - The interface secification for a Player component.
xattack - Shell script to validate a Situation, reads the Situation JSON from stdin

attack/Cargo.lock - Rust cargo lock file for dependencies.
attack/Cargo.toml - Top level crate manifest file.

attack/src/lib.rs - Entry point to the crate, re-exports public interface.
attack/src/machine - Docs for the game state machine.
attack/src/actor/mod.rs - re-exports for any actors
attack/src/actor/player.rs - Interface for a Player playing the game.
attack/src/mod.rs - Data representation for a Game and shared constants.
attack/src/game/board/mod.rs - Data representation for a game board
attack/src/game/card/mod.rs - Data representation for a trait Card.
attack/src/game/player/mod.rs - Data representation for a Player. 
attack/src/game/species/mod.rs - Data representation for a species.
attack/src/game/species/de.rs - Deserialization implementation for a Species.
attack/src/game/species/situation.rs - Data representation for an attack Situation.
attack/src/game/traits/mod.rs - Data representation for a game Trait.
attack/src/traits/de.rs - Deserialization implementation for the game Trait.
attack/src/message/mod.rs - Data represenation for the messages sent to/from the Player.

attack/client/situation.rs - An example attack Situation
attack/client/src/bin/xattack.rs - Rust binary that is used by the xattack shell script.

Running the code:
To check the validity of a Situation pipe the json representing the Situation to the xattack binary.

ex: Using our example situation file.
./xattack < attack/client/situation.json

Running the test suite:
If rust nightly is installed you can run 'cargo test' from the attck folder.

Otherwise you can use the cargo shell script in the root dir, which will download a linux binary of Rust to use. 
ex: from the attack folder:
../../cargo test