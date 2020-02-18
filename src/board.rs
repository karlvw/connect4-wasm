//!
//! Connect 4 Board
//! 

use serde::{Serialize, Deserialize};

// Size of the board
pub const NUM_COLUMNS: usize = 7;
pub const NUM_ROWS: usize = 6;


#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)] 
pub enum Cell {
    Empty,
    Computer,
    Player,
}

#[derive(Debug, PartialEq)]
pub enum GameResult {
    PlayerWins,
    ComputerWins,
    Draw,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Board {
    cells: [[Cell; NUM_COLUMNS]; NUM_ROWS],
    moves_made: usize,
}

impl Board {
    /// Create a new blank board
    pub fn new() -> Self {
        Self {
            cells: [[Cell::Empty; NUM_COLUMNS]; NUM_ROWS],
            moves_made: 0,
        }
    }

    /// Gets the state of the cell at the given location
    pub fn get_cell(&self, row: usize, col: usize) -> &Cell {
        &self.cells[row][col]
    }

    /// Player makes a move in the given column
    pub fn player_move(&mut self, col:usize) -> bool {
        self.make_move(col, Cell::Player)
    }

    /// Computer makes a move in the given column
    pub fn computer_move(&mut self, col:usize) -> bool {
        self.make_move(col, Cell::Computer)
    }

    fn make_move(&mut self, col: usize, cell: Cell) -> bool {
        if col >= NUM_COLUMNS {
            return false;
        }

        for row in (0..NUM_ROWS).rev() {
            if self.cells[row][col] == Cell::Empty {
                self.cells[row][col] = cell;
                self.moves_made += 1;
                return true;
            }
        }
        false
    }

    /// Returns the number of moves made on this board
    pub fn num_moves_made(&self) -> usize {
        self.moves_made
    }

    /// Check to see if the game has a winner or a draw
    pub fn check_winner(&self) -> Option<GameResult> {
        let check_group = |&c1, &c2, &c3, &c4| -> Option<GameResult> {
            if (c1 == c2) && (c2 == c3) && (c3 == c4) {
                match c1 {
                    Cell::Player => Some(GameResult::PlayerWins),
                    Cell::Computer => Some(GameResult::ComputerWins),
                    Cell::Empty => None,
                }
            } else {
                None
            }
        };

        for row in 0..NUM_ROWS {
            for col in 0..NUM_COLUMNS {
                let c1 = &self.cells[row][col];

                if *c1 == Cell::Empty {
                    continue;
                }

                // Check for horizontal sequences
                if col < (NUM_COLUMNS-3) {
                    if let Some(result) = check_group(c1, &self.cells[row][col+1], &self.cells[row][col+2], &self.cells[row][col+3]) {
                        return Some(result);
                    }
                }

                if row < (NUM_ROWS-3) {
                    // Check for vertical sequences
                    if let Some(result) = check_group(c1, &self.cells[row+1][col], &self.cells[row+2][col], &self.cells[row+3][col]) {
                        return Some(result);
                    }

                    // Check for diagonal sequences
                    if col < (NUM_COLUMNS-3) {
                        if let Some(result) = check_group(c1, &self.cells[row+1][col+1], &self.cells[row+2][col+2], &self.cells[row+3][col+3]) {
                            return Some(result);
                        }
                    }
                    if col >= 3 {
                        if let Some(result) = check_group(c1, &self.cells[row+1][col-1], &self.cells[row+2][col-2], &self.cells[row+3][col-3]) {
                            return Some(result);
                        }
                    }
                }
            }
        }

        // Check for a draw
        if self.moves_made >= (NUM_COLUMNS * NUM_ROWS) {
            return Some(GameResult::Draw);
        }
        
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fill_column() {
        let mut board = Board::new();
        for _ in 0..NUM_ROWS {
            assert_eq!(board.player_move(0), true);
        }
        assert_eq!(board.player_move(0), false);
    }

    #[test]
    fn check_win_vertical() {
        let mut board = Board::new();
        for _ in 0..4 {
            assert_eq!(board.check_winner(), None);
            board.player_move(5);
        }
        assert_eq!(board.check_winner(), Some(GameResult::PlayerWins));
    }

    #[test]
    fn check_win_horizontal() {
        let mut board = Board::new();
        for col in 1..5 {
            assert_eq!(board.check_winner(), None);
            board.computer_move(col);
        }
        assert_eq!(board.check_winner(), Some(GameResult::ComputerWins));
    }

    #[test]
    fn check_win_diagonal1() {
        let mut board = Board::new();
        for col in vec![2, 3, 3, 4, 4, 4] {
            board.player_move(col);
        }
        for col in 1..5 {
            assert_eq!(board.check_winner(), None);
            board.computer_move(col);
        }
        assert_eq!(board.check_winner(), Some(GameResult::ComputerWins));
    }

    #[test]
    fn check_win_diagonal2() {
        let mut board = Board::new();
        for col in vec![0, 0, 0, 1, 1, 2] {
            board.computer_move(col);
        }
        for col in 0..4 {
            assert_eq!(board.check_winner(), None);
            board.player_move(col);
        }
        assert_eq!(board.check_winner(), Some(GameResult::PlayerWins));
    }

    #[test]
    fn check_draw() {
        let mut board = Board::new();
        for row in 0..NUM_ROWS {
            for col in 0..NUM_COLUMNS {
                assert_eq!(board.check_winner(), None);
                if col % 2 == (row / 2) % 2 {
                    board.player_move(col);
                } else {
                    board.computer_move(col);
                }
            }
        }
        assert_eq!(board.check_winner(), Some(GameResult::Draw));
    }
}
