Task 2: The main interfaces to look at here are in evolution/choose and
evolution/step. Choose holds the code for getting a Choice from an
Observation, and step holds the entry points for the logic for each step of the
game. A note about steps. We have 5 not 4, as we make a distinction between
reveal and feed. This was originally to accommodate a state machine which we
currently are not using. The state of the code at the deadline is not 100%
cleaned up, things have moved recently to start preparing for the final
code-walk. One neat result of the Choose interface was that we can implement
choose for a choice, and pass that anywhere we expect to need to talk to an
external player. This has, and will make testing and debugging easier.

Task 3: The code for card ordering is defined in card/mod.rs and is rather nice,
though simple. The silly player is implemented by `SimpleChooser` in
src/test_helpers/simple_chooser.rs. We do not currently have a server
implementation of choosing an action because the simple chooser itself works
and there is no logic the server needs to do. This is obviously incorrect for
actually hooking up external code and will be addressed.
