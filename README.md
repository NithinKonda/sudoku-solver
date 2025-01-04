# Sudosolve

`Sudosolve` is a Rust crate for solving Sudoku puzzles. It takes a Sudoku puzzle as input in string format and provides the solved grid if a solution exists. The crate uses a backtracking algorithm to efficiently solve puzzles.

## Features

- Parses Sudoku puzzles from a string format.
- Validates moves based on Sudoku rules.
- Solves puzzles using a recursive backtracking algorithm.
- Handles invalid or unsolvable puzzles gracefully.

## Installation

You can install `sudosolve` directly from [crates.io](https://crates.io/crates/sudosolve) using Cargo:

```bash
cargo install sudosolve
```
Usage

Example

Here's an example of how to use the crate:
```
use sudosolve::SudokuBoard;

fn main() {
    let input = "53__7____6__195___98____6_8___6___34__8_3__17___2___6_6____28___419__5____8__79"; // Replace with your Sudoku string
    let mut board = SudokuBoard::new(&input);
    
    println!("Original Sudoku:");
    board.print();
    
    if board.solve() {
        println!("\nSolved Sudoku:");
        board.print();
    } else {
        println!("\nNo solution exists.");
    }
}
```
The input string represents the Sudoku grid, where each row is concatenated into a single string. Use _ or 0 for empty cells.

Input Format

Provide the puzzle as an 81-character string, with digits (1 to 9) for filled cells and _ or 0 for empty cells.

Rows are concatenated from left to right, top to bottom.


Example Input
```
53__7____
6__195___
98____6__
8___6___3
4__8_3__1
7___2___6
_6____28_
___419__5
____8__79
```
Should be provided as:
```
53__7____6__195___98____6_8___6___34__8_3__17___2___6_6____28___419__5____8__79
```

---

Development and Testing

Clone the repository and build the crate locally:
```
git clone https://github.com/NithinKonda/sudoku-solver.git
```
-> cd sudosolve

-> cargo build

-> cargo test

## License

This project is licensed under the MIT License.

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests on GitHub.


---

## About the Algorithm

The solver uses a backtracking algorithm that:

1. Searches for the first empty cell.


2. Attempts to place each number (1-9) in the cell.


3. Checks the validity of the placement using Sudoku rules.


4. Recursively continues until the puzzle is solved or deemed unsolvable.




---

Happy solving with sudosolve!
