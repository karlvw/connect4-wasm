//!
//! A Connect 4 Game using wasm and rust
//! 

use seed::{prelude::*, *};
mod board;
mod ai;


struct Model {
    pub board: board::Board,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            board: board::Board::new(),
        }
    }
}


#[derive(Clone)]
enum Msg {
    ColumnClick(usize),
    ResetGame,
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::ColumnClick(col) => {
            if model.board.check_winner() == None {
                if model.board.player_move(col) {
                    ai::make_move(&mut model.board);
                }
            }
        },
        Msg::ResetGame => {
            model.board = board::Board::new();
        },
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
        }
    ]
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view).build_and_start();
}
