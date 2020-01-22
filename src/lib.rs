//!
//! A Connect 4 Game using wasm and rust
//! 

use seed::{prelude::*, *};
mod board;


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
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::ColumnClick(col) => {
            model.board.player_move(col);  
        }
    }
}

fn view(model: &Model) -> Node<Msg> {

    let cell_view = |row, col| {
        let cell = model.board.get_cell(row, col);
        match cell {
            board::Cell::Empty => "",
            board::Cell::Computer => "🍎", // Red apple
            board::Cell::Player => "🍏", // Green apple
        }
    };

    div![
        table![
            style!{St::Border => 1}, 
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
    ]
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view).build_and_start();
}
