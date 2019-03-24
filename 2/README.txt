Implement another student's player interface, and finish the implementation
of our take5 game.

Reading the interface of our take5 implementation may be easier if you have
Rust installed on your system, by running `cargo doc --open` from inside
the take5 or take5_cli subdirectory.

Usage
-----

To run the tests execute `cargo test` from within the take5 subdirectory. To
run the program execute `cargo run -- -h` from within the take5_cli
subdirectory. `-h` will print the usage for the executable. `cargo run -- 5`
for example will run a game of 5 AIs.

Files
-----

player.rb - An implementation of the given for-2 player interface.
take5/
    src/
        lib.rs   - Entry point for the take5 crate.
        board.rs         - A type representing all of the stacks in the board.
        card.r           - Playing cards for the game.
        configuration.rs - Loading and accessing game configurations.
        game.rs          - Holds the board, dealer and players.
        stack.rs         - Cards you can add to on and take all of.
        dealer/
            mod.rs             - The dealer trait.
            standard_dealer.rs - One implementation of a dealer.
        player/
            mod.rs       - The player trait.
            ai_player.rs - A stupid ai player implementation.
    Cargo.lock - Cargo's manifest file for locking dependencies.
    Cargo.toml - Cargo's manifest file for describing dependencies.

take5_cli/
    src/
        main.rs - Executable implementation of take5.
        args.rs - Manages the command line arguments for starting the game.
    take6.conf - A config file for the changes described in assignment 3.
    bull.conf  - A comma separated list of 104 bull values describing the deck.
    Cargo.lock - Cargo's manifest file for locking dependencies.
    Cargo.toml - Cargo's manifest file for describing dependencies.
