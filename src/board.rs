//!
//! Connect 4 Board
//! 

// Size of the board
pub const NUM_COLUMNS: usize = 7;
pub const NUM_ROWS: usize = 5;

#[derive(Clone, Copy, PartialEq)]
pub enum Cell {
    Empty,
    Computer,
    Player,
}

#[derive(Clone)]
pub struct Board {
    cells: [[Cell; NUM_COLUMNS]; NUM_ROWS],
}

impl Board {
    /// Create a new blank board
    pub fn new() -> Self {
        Self {
            cells: [[Cell::Empty; NUM_COLUMNS]; NUM_ROWS]
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

    fn make_move(&mut self, col: usize, cell: Cell) -> bool {
        if col >= NUM_COLUMNS {
            return false;
        }

        for row in (0..NUM_ROWS).rev() {
            if self.cells[row][col] == Cell::Empty {
                self.cells[row][col] = cell;
                return true;
            }
        }
        false
    }
}
