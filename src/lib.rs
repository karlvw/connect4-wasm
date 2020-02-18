//!
//! A Connect 4 Game using wasm and rust
//! 

use serde::{Serialize, Deserialize};
use serde_json;
use seed::{prelude::*, *};
mod board;
mod ai;


#[derive(Serialize, Deserialize)]
struct Model {
    pub board: board::Board,
    pub wins: u32,
    pub losses: u32,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            board: board::Board::new(),
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


fn after_mount(_: Url, _orders: &mut impl Orders<Msg>) -> AfterMount<Model> {
    // Load the model at startup
    let load_saved_model = || {
        let storage = seed::storage::get_storage()?;
        let loaded_serialized = storage.get_item("model").ok()??;
        Some(serde_json::from_str(&loaded_serialized).ok()?)
    };

    AfterMount::new(match load_saved_model() {
        Some(model) => model,
        None => Model::default(),
    })
}

async fn make_ai_move(board: board::Board) -> Result<Msg, Msg> {
    let col = ai::best_move(&board);
    Ok(Msg::ComputerMakeMove(col))
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    let mut move_made = false;

    match msg {
        Msg::ColumnClick(col) => {
            if model.board.check_winner() == None {
                if model.board.player_move(col) {
                    move_made = true;
                    orders.perform_cmd(make_ai_move(model.board.clone()));
                }
            }
        },
        Msg::ComputerMakeMove(col) => {
            if model.board.check_winner() == None {
                if model.board.computer_move(col) {
                    move_made = true;
                }
            }
        },
        Msg::ResetGame => {
            model.board = board::Board::new();
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
    if let Some(storage) = seed::storage::get_storage() {
        if let Ok(data) = serde_json::to_string(&model) {
            storage.set_item("model", &data).ok();
        }
    }
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
        attrs!{At::Class => "container"},
        table![
            tr![
                (0..board::NUM_COLUMNS).map(|col|
                    th![ 
                        button![ simple_ev(Ev::Click, Msg::ColumnClick(col)), "↓" ] 
                    ],
                )
            ],

            (0..board::NUM_ROWS).map(|row|
                tr![
                    (0..board::NUM_COLUMNS).map(|col|
                        td![
                            attrs!{At::Class => "board-cell"}, 
                            cell_view(row, col) 
                        ],
                    )
                ],
            )  
        ],
        if game_result != None {
            div![
                attrs!{At::Class => "overlay"},
                div![
                    attrs!{At::Class => "message"},
                    match game_result {
                        Some(board::GameResult::PlayerWins) => "You Won!!",
                        Some(board::GameResult::ComputerWins) => "Oh no, you have lost.",
                        Some(board::GameResult::Draw) => "Looks like this one is a draw.",
                        None => "",
                    },
                    br![],
                    button![ simple_ev(Ev::Click, Msg::ResetGame), "Play Again?" ]
                ]
            ]
        } else {
            empty![]
        },
        div![
            format!("Wins: {} Losses: {}", model.wins, model.losses)
        ]
    ]
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view)
        .after_mount(after_mount)
        .build_and_start();
}
