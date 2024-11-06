# Sudoku Generator

This project is a Rust application to generate Sudoku puzzles in your terminal.

Usage:

```sh
sudoku <difficulty_level>
```

- `<difficulty_level>`: An integer representing the puzzle's difficulty. Lower values correspond to harder puzzles.

## Example

```sh
sudoku 17
```

This command will generate a Sudoku puzzle with 17 hints.

**_Fun Fact_**: A Sudoku puzzle needs at least 17 hints for a unique solution.

**Source**: Trust me bro.

## Prerequisites

To build and run this project, you need to have Rust installed. Follow the instructions on
the [official Rust website](https://www.rust-lang.org/tools/install).

## Dependencies

This project uses the following dependencies:

- `clap 4.5.20`: For command-line argument parsing.
- `rand 0.9.0-alpha.2`: For generating random numbers.

These dependencies are defined in the `Cargo.toml` file:

```toml
[dependencies]
clap = "4.5.20"
rand = "0.9.0-alpha.2"
```

## Building the Project

To build the project, navigate to the project directory and run:

```sh
cargo build
```

This command will compile the project and its dependencies.