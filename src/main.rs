use std::io;
struct SudokuBoard {
    grid: [[u8; 9]; 9],
}

impl SudokuBoard {
    fn new(input: &str) -> Self {
        let mut grid = [[0; 9]; 9];
        let input_chars: Vec<char> = input.chars().collect();
        
        for (i, &val) in input_chars.iter().enumerate() {
            let row = i / 9;
            let col = i % 9;
            grid[row][col] = match val {
                '_' => 0,
                '1'..='9' => val.to_digit(10).unwrap() as u8,
                _ => 0
            };
        }
        
        SudokuBoard { grid }
    }

    fn is_valid_move(&self, row: usize, col: usize, num: u8) -> bool {

        for c in 0..9 {
            if c != col && self.grid[row][c] == num {
                return false;
            }
        }

        for r in 0..9 {
            if r != row && self.grid[r][col] == num {
                return false;
            }
        }

        let box_row = (row / 3) * 3;
        let box_col = (col / 3) * 3;
        
        for r in box_row..box_row + 3 {
            for c in box_col..box_col + 3 {
                if (r != row || c != col) && self.grid[r][c] == num {
                    return false;
                }
            }
        }

        true
    }

    fn find_empty_cell(&self) -> Option<(usize, usize)> {
        for row in 0..9 {
            for col in 0..9 {
                if self.grid[row][col] == 0 {
                    return Some((row, col));
                }
            }
        }
        None
    }

    fn solve(&mut self) -> bool {
        if let Some((row, col)) = self.find_empty_cell() {
            for num in 1..=9 {
                if self.is_valid_move(row, col, num) {
                    self.grid[row][col] = num;

                    if self.solve() {
                        return true;
                    }

                    self.grid[row][col] = 0;
                }
            }
            false
        } else {
            true
        }
    }

    fn print(&self) {
        for row in 0..9 {
            for col in 0..9 {
                print!("{} ", self.grid[row][col]);
                if (col + 1) % 3 == 0 && col < 8 {
                    print!("| ");
                }
            }
            println!();
            if (row + 1) % 3 == 0 && row < 8 {
                println!("---------------------");
            }
        }
    }
}

fn main() {

   println!("Input the Sudoku string here:");

   let mut input = String::new();
   io::stdin().read_line(&mut input).expect("Failed to read line");
   let sudoku_input = input.trim().to_string();

   let mut board = SudokuBoard::new(&sudoku_input);
    
    println!("Original Sudoku:");
    board.print();
    
    if board.solve() {
        println!("\nSolved Sudoku:");
        board.print();
    } else {
        println!("\nNo solution exists.");
    }
}
