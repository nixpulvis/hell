Display the state of a dealer and current player given a configuration.

[1] Directory Map:
./compile - The build script for this project
./xgui - Reads a dealer configuration and produces a view of the dealer and
         current player.
./gui/evolution/evolution-ui - Top level directory for UI crate.

The following files are within ./gui/evolution/evolution-ui:
./assets/fonts/FiraMono-Regular.ttf - Font asset used by the GUI.
./assets/fonts/OFL.txt - The license agreement for the "Fira Mono" font.
./src/bin/xgui_dealer.rs - Source for the dealer display window.
./src/bin/xgui_player.rs - Source for the player display window.
./board_widget.rs - Source for the component that displays a game board.
./dealer_widget.rs - Source for the component that displays a dealer.
./debug_widget.rs - Source for the component that displays text in a block.
./deck_widget.rs - Source for the component that displays a deck.
./game_widget.rs - Source for the component that displays a game state.
./lib.rs - Library source file.
./player_widget.rs - Source for the component that displays a player.
./species_widget.rs - Source for the component that displays a species board.
./tests - Test directory.
./Cargo.lock - Dependency management metadata.
./Cargo.toml - Dependency management metadata.


[2] How to run:

Invoke ./compile, followed by ./xgui, it reads a configuration from STDIN.

[3] Road Map:

This application relies on a GUI library called Conrod. All the widget types in
./gui/evolution/evolution-ui render using Conrod. The display executables,
namely xgui_dealer and xgui_player generate Conrod windows in a thin wrapper
application. The application xgui shares the contents of the input between the
two display executables, and terminates once both subprocesses terminate. 
