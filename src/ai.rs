//!
//! Connect 4 AI
//!

use rand::{thread_rng, Rng};
use crate::board;


fn random_column() -> usize {
    let mut rng = thread_rng();
    rng.gen_range(0, board::NUM_COLUMNS)
}

// Makes a move on the on the given board
pub fn make_move(board: &mut board::Board) {

    // TODO: Make this smarter!
    while board.check_winner() == None {
        if board.computer_move(random_column()) {
            break;
        }
    }
}
