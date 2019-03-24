Refactors to put the codebase in a presentable state.

[1] Directory Map:
./compile - Compiles the project so it can be run.
./test - Runs all tests across entire project.
./evolution - Evolution library source.
./evolution/evolution-client - Player-side binaries and tests (xfeed, xattack).
./evolution/evolution-logger - Logging/debugging library used in development.
./evolution/evolution-server - Server-side binaries and tests (xstep).
./evolution/evolution-test - Generic test harness library.
./evolution/evolution-ui - GUI libraries and binaries (xgui).
./evolution/evolution-wire - Serialization/deserialization library for Evolution.

[2] How to Run:
Invoke ./compile to build.
Invoke ./test to run all tests.

[3] Road Map:
Begin in the ./evolution/src directory to view the library. It is recommended to
start with lib.rs to get an overview of the library contents. From there, any
additional module code may be found in the like-named subdirectory within the
./evolution/src directory.
