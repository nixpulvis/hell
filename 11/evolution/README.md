# Evolution

- [Documentation](https://github.ccs.neu.edu/pages/cs4500sp16/cs4500-nathanl-stroetti/evolution/index.html)


A competition framework for the board game "Evolution" in Rust. The main goals
of this project are to build and test the server component to handle clients
either over the wire or as linked implementations in Rust.

This project was created for CS4500 at Northeastern University. The rules for
our game of Evolution differ from the official rules shipped with the physical
board game.

## Crates

Read the documentation for more information on each crate.

- `evolution` The evolution library, tests and client/server executables.
- `evolution-wire` Intermediate data representations for the JSON data formats.
- `evolution-ui` A UI for displaying/interacting with the evolution game.
- `evolution-test` A compiler plugin to load and run directory of integration tests.

## Server

```fish
cargo run --bin server
```

## Client

Assumes that a server is running.

```fish
cargo run --bin client

# Stress test (fish syntax).
for n in (seq 10); cargo run --bin client &; end
```

## Tests

```fish
env RUST_TEST_THREADS=1 cargo test
```

As part of developing software, especially software designed to interact with
external components, tests are critical. This project has two main varieties of
tests. The first are the library tests which test the individual components of
our system. The second are the integration tests which test a small, yet whole
system's functionality.

### Library Tests

Rust organizes library tests as functions with the attribute `#[test]` in any
module inside `src/`. Generally speaking library tests for a module should
follow this pattern:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn some_descriptive_name_that_might_be_rather_long() {
        // Data needed for the test
        let something = Something::mock();

        // Explicitly state implicit pre-conditions of the test.
        assert_eq!(1, something.foo);

        // Call some function(s), possibly binding the result if needed.
        let out = something.do();

        // Test the output/effect of the function(s).
        assert!(out.is_ok());
    }
}
```

### Integration Tests

Rust puts integration tests into a directory named `tests/`. These tests
should take a black box view of the system, using it from a client perspective
more or less. Our integration tests currently call out to test harnesses to
consume and produce JSON representations of the input data, expectations and
actual results. Our test harnesses are located in `bin`, and are compiled and
linked whenever changes are made to the source code. Test cases are stored in
folders for each harness. For example for the test harness `xattack` there
is a directory named `situation_json/` with `*-in.json` and `*-out.json` file
pairs. The test harness runner macro `each_test!` will compile each test file
into a test case statically. This means that each time a new test is added to
the directory you must recompile the tests. Below is an example of using the
`each_test!`.

```rust
// Argument 1: The directory to compile JSON tests for.
// Argument 2: The wire type to deserialize the input data as.
// Argument 3: The name of the test harness executable.
each_test!("tests/situation_json", evolution_wire::Situation, "xattack");
```
