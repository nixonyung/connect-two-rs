# Connect Two

My Rust implementation of the simplified version of the game Connect Four.

In this version, the board is just a single row of cells (the default size is `4`, configurable via env `BOARD_SIZE`). The player wins if they can occupy any 2 adjacent cells.

A solver is also implemented.

## Getting Started

- try the game:
  - run `cargo run --bin cli`
- try the solver:
  - run `cargo run --bin solver`
- test the game (requires [Bats](https://github.com/bats-core/bats-core)):
  - run `bats --jobs $(nproc) --verbose-run ./tests`
