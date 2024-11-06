use clap::Parser;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

const N: usize = 9;
const UNASSIGNED: i32 = 0;

#[derive(Parser)]
struct Cli {
    level: usize,
}

fn main() {
    let args = Cli::parse();
    let mut board: [[i32; N]; N] = [[UNASSIGNED; N]; N];

    fill_diagonal_blocks(&mut board);
    solve_sudoku(&mut board);
    remove_random_cells(&mut board, args.level); // requires at least 17 for unique solution

    print_board(&board);
}

/// Function to check if a number can be placed in a cell
fn is_safe(board: &[[i32; N]; N], row: usize, col: usize, num: i32) -> bool {
    // Check if the number is not repeating in the current row, column and 3x3 subgrid
    for i in 0..N {
        if board[row][i] == num || board[i][col] == num {
            return false;
        }
    }

    let start_row = row - row % 3;
    let start_col = col - col % 3;

    for i in 0..3 {
        for j in 0..3 {
            if board[i + start_row][j + start_col] == num {
                return false;
            }
        }
    }

    true
}

/// Utility function to fill the diagonal 3x3 boxes
fn fill_diagonal_blocks(board: &mut [[i32; N]; N]) {
    for i in (0..N).step_by(3) {
        fill_3x3_box(board, i, i);
    }
}

/// Function to fill a 3x3 box
fn fill_3x3_box(board: &mut [[i32; N]; N], row: usize, col: usize) {
    let mut nums: Vec<i32> = (1..=9).collect();
    nums.shuffle(&mut thread_rng());

    for i in 0..3 {
        for j in 0..3 {
            board[row + i][col + j] = nums[i * 3 + j];
        }
    }
}

/// Utility function to check if we can solve the board
fn solve_sudoku(board: &mut [[i32; N]; N]) -> bool {
    let (mut row, mut col) = (0, 0);

    if !find_unassigned_location(board, &mut row, &mut col) {
        return true;
    }

    for num in 1..=9 {
        if is_safe(board, row, col, num) {
            board[row][col] = num;
            if solve_sudoku(board) {
                return true;
            }

            board[row][col] = UNASSIGNED;
        }
    }

    false
}

/// Find an unassigned location on the board
fn find_unassigned_location(board: &[[i32; N]; N], row: &mut usize, col: &mut usize) -> bool {
    for r in 0..N {
        for c in 0..N {
            if board[r][c] == UNASSIGNED {
                *row = r;
                *col = c;
                return true;
            }
        }
    }

    false
}

/// Function to remove some cells to create the puzzle
fn remove_random_cells(board: &mut [[i32; N]; N], level: usize) {
    let mut remaining_cells = N * N;
    let mut rng = thread_rng();

    while remaining_cells > level {
        let row = rng.gen_range(0..N);
        let col = rng.gen_range(0..N);

        if board[row][col] != UNASSIGNED {
            board[row][col] = UNASSIGNED;
            remaining_cells -= 1;
        }
    }
}

/// Function to print the board
fn print_board(board: &[[i32; N]; N]) {
    for row in board.iter() {
        for &cell in row.iter() {
            if cell == UNASSIGNED {
                print!("_ ");
            } else {
                print!("{} ", cell);
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_safe() {
        let mut board: [[i32; N]; N] = [[UNASSIGNED; N]; N];
        board[0][0] = 5;

        assert!(is_safe(&board, 0, 1, 3));
        assert!(!is_safe(&board, 0, 1, 5));
    }

    #[test]
    fn test_fill_3x3_box() {
        let mut board: [[i32; N]; N] = [[UNASSIGNED; N]; N];
        fill_3x3_box(&mut board, 0, 0);

        for i in 0..3 {
            for j in 0..3 {
                assert!(board[i][j] > 0 && board[i][j] <= 9);
            }
        }
    }

    #[test]
    fn test_find_unassigned_location() {
        let mut board: [[i32; N]; N] = [[1; N]; N];
        let mut row = 0;
        let mut col = 0;
        board[8][8] = UNASSIGNED;

        assert!(find_unassigned_location(&board, &mut row, &mut col));
        assert_eq!(row, 8);
        assert_eq!(col, 8);
    }
}
