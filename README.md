# Sudoku Solver

A Rust crate for solving Sudoku puzzles. This crate takes a Sudoku puzzle as input in the form of a string and provides the solved grid if a solution exists. It implements a backtracking algorithm to solve puzzles efficiently.

## Features

- Parses Sudoku puzzles from a string format.
- Validates moves based on Sudoku rules.
- Solves puzzles using a recursive backtracking algorithm.
- Handles invalid or unsolvable puzzles gracefully.

## Usage

To use this crate, include it in your `Cargo.toml`:

```toml
[dependencies]
sudoku_solver = "0.1.0"
```