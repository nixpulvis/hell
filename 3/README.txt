Implement a player, and make minor modifications to the game of take5. Also
create a JSON oriented protocol for client/server playing of the game.

The following diffs were created from the bare minimum sensible changes needed to modify the game described in the assignment. We've made more changes since these diffs to clean up our code further.

See ../2/README.txt for how to run a game of take5. It's also recommended that
you read the documentation for take5, as describe in the aforementioned
README.

take5_ext/
    src/
        lib.rs           - Entry point for this crate.
        custom_dealer.rs - A dealer who's bull values can be mapped.
        stdin_player.rs  - A human player who interacts with STDIN.
    Cargo.lock - Cargo's manifest file for locking dependencies.
    Cargo.toml - Cargo's manifest file for describing dependencies.
    main.diff
        In order to statically link to the player definitions in the 3/ directory, we refactored our code base into three different crates. The take5 crate now only contains basic game logic and default implementations of a player and dealer. Our take5_ext crate contains implementations of other players and dealers. Lastly, we created a take5_cli which parses command line arguments to create the appropriate dealer and players which are passed into the modified game new function. Almost all of these changes are simple copying code into new locations. The main change was the addition of two arguments to `state::new`.
    1.diff
        Changing the definition of a full stack from 5 to 6 cards.
    2.diff
        Changing deck size to 210.
    3.diff
        As the number of turns in a round was used in several places, we refactored to use a `TURNS` constant and set that constant to 9. These constants are now being handled with a configuration struct.
    4.diff
        Changed validation of cards to disallow cards with bull value less than 3 as well as changing our StandardDealer to create cards with bull value 3 instead of 2.
    5.diff
        Changed the AiPlayer algorithm to play cards in increasing order.
    tests.diff
        Changes necessary to get tests passing.
