//!
//! A Connect 4 Game using wasm and rust
//! 

use serde::{Serialize, Deserialize};
use seed::{prelude::*, *};
use gloo_timers::future::TimeoutFuture;

mod board;
mod ai;

const STORAGE_KEY: &str = "model";

#[derive(Serialize, Deserialize, PartialEq, Clone)]
enum Turn {
    Player,
    Computer,
}

#[derive(Serialize, Deserialize)]
struct Model {
    pub board: board::Board,
    pub turn: Turn,
    pub who_starts: Turn,
    pub wins: u32,
    pub losses: u32,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            board: board::Board::new(),
            turn: Turn::Player,
            who_starts: Turn::Player,
            wins: 0,
            losses: 0,
        }
    }
}


#[derive(Clone)]
enum Msg {
    ColumnClick(usize),
    ComputerMakeMove(usize),
    ResetGame,
}


fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    // Load the model at startup
    let model: Model = LocalStorage::get(STORAGE_KEY).unwrap_or_default();

    if model.turn == Turn::Computer {
        // Trigger the computer to make a move if it is its turn
        orders.perform_cmd(make_ai_move(model.board.clone()));
    }
    model
}

async fn make_ai_move(board: board::Board) -> Msg {
    TimeoutFuture::new(20).await;  // Wait for the screen to redraw
    let col = ai::best_move(&board);
    Msg::ComputerMakeMove(col)
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    let mut move_made = false;

    match msg {
        Msg::ColumnClick(col) => {
            if model.turn == Turn::Player && model.board.check_winner().is_none() {
                if model.board.player_move(col) {
                    move_made = true;
                    model.turn = Turn::Computer;
                    orders.perform_cmd(make_ai_move(model.board.clone()));
                }
            }
        },
        Msg::ComputerMakeMove(col) => {
            if model.turn == Turn::Computer && model.board.check_winner().is_none() {
                if model.board.computer_move(col) {
                    move_made = true;
                    model.turn = Turn::Player;
                }
            }
        },
        Msg::ResetGame => {
            model.board = board::Board::new();
            // Take turns starting
            model.who_starts = match model.who_starts {
                Turn::Player => Turn::Computer,
                Turn::Computer => Turn::Player
            };
            model.turn = model.who_starts.clone();
            if model.turn == Turn::Computer {
                orders.perform_cmd(make_ai_move(model.board.clone()));
            }
        },
    }

    if move_made {
        match model.board.check_winner() {
            Some(board::GameResult::PlayerWins) => model.wins += 1,
            Some(board::GameResult::ComputerWins) => model.losses +=1,
            _ => (),
        }
    }

    // Save the state of the model
    LocalStorage::insert(STORAGE_KEY, &model).ok();
}

fn view(model: &Model) -> Node<Msg> {

    let game_result = model.board.check_winner();

    let cell_view = |row, col| {
        let cell = model.board.get_cell(row, col);
        match cell {
            board::Cell::Empty => "",
            board::Cell::Computer => "🔴",
            board::Cell::Player => "🔵",
        }
    };

    div![
        C!["container"],
        table![
            tr![
                (0..board::NUM_COLUMNS).map(|col|
                    th![ 
                        button![ ev(Ev::Click, move |_| Msg::ColumnClick(col)), "↓" ] 
                    ],
                )
            ],

            (0..board::NUM_ROWS).map(|row|
                tr![
                    (0..board::NUM_COLUMNS).map(|col|
                        td![
                            C!["board-cell"], 
                            cell_view(row, col) 
                        ],
                    )
                ],
            )  
        ],
        IF!(game_result.is_some() =>
            div![
                C!["overlay"],
                div![
                    C!["message"],
                    match game_result {
                        Some(board::GameResult::PlayerWins) => "You Won!!",
                        Some(board::GameResult::ComputerWins) => "Oh no, you have lost.",
                        Some(board::GameResult::Draw) => "Looks like this one is a draw.",
                        None => "",
                    },
                    br![],
                    button![ ev(Ev::Click, |_| Msg::ResetGame), "Play Again?" ]
                ]
            ]
        ),
        div![
            format!("Wins: {} Losses: {}", model.wins, model.losses)
        ]
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
