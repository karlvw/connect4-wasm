//!
//! A simple Connect 4 AI
//!
//! 

use crate::board::{Board, GameResult, NUM_COLUMNS};

/// How many moves into the future we will predict.
const NUM_ITERATION: i32 = 8;

type BoardScore = f32;


/// Generate a score if the board has an outcome, with winning in the least number of moves being the best
fn score_board(board: &Board) -> Option<BoardScore> {
    match board.check_winner() {
        Some(GameResult::ComputerWins) => Some(100.0 - board.num_moves_made() as BoardScore),
        Some(GameResult::Draw) => Some(0.0),
        Some(GameResult::PlayerWins) => Some(board.num_moves_made() as BoardScore - 100.0),
        None => None,
    }
}

/// Finds the best scored move the computer can make for the given board, returning the best column and score
fn best_computer_move(board: &Board, remaining_iterations: i32) -> (usize, BoardScore) {
    if remaining_iterations <= 0 {
        return (0, 0.0);
    }

    let mut max_score = std::f32::NEG_INFINITY;
    let mut best_col = 0;

    for col in 0..NUM_COLUMNS {
        let mut new_board = board.clone();

        if new_board.computer_move(col) {
            let score = match score_board(&new_board) {
                Some(x) => x,
                None => simulate_player_move(&new_board, remaining_iterations-1),
            };
            if score > max_score {
                max_score = score;
                best_col = col;
            }
        }
    }
    return (best_col, max_score);
}

/// Simulates a player move averaging the scores for all valid player moves that don't end the game.
/// If a player wins, it assumes the player will make that move.
fn simulate_player_move(board: &Board, remaining_iterations: i32) -> BoardScore {
    let mut score_sum = 0.0;
    let mut count = 0;

    for col in 0..NUM_COLUMNS {
        let mut new_board = board.clone();

        if new_board.player_move(col) {
            if let Some(score) = score_board(&new_board) {
                // The game has an outcome, assume the player will make that move
                return score;
            } else {
                // Take the average across the possible player moves
                let (_, score) = best_computer_move(&new_board, remaining_iterations-1);
                score_sum += score;
                count += 1;
            }
        }
    }
    return score_sum / count as BoardScore;
}

/// Makes a move on the on the given board
pub fn make_move(board: &mut Board) {
    if board.check_winner() != None {
        return;
    }

    let (best_col, _) = best_computer_move(board, NUM_ITERATION);
    board.computer_move(best_col);
}
