env LOG=trace cargo run --bin server 3 & env LOG=trace cargo run --bin client a & env LOG=trace cargo run --bin client b & env LOG=trace cargo run --bin client c
