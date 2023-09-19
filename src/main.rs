//!
//! A Connect 4 Game using wasm and rust
//! 

use std::time::Duration;
use yew::prelude::*;
use yew::platform::spawn_local;
use yew::platform::time::sleep;
use serde::{Serialize, Deserialize};
use gloo::storage::{LocalStorage, Storage};

mod board;
mod ai;

const STORAGE_KEY: &str = "model";

#[derive(Serialize, Deserialize, PartialEq, Clone)]
enum Turn {
    Player,
    Computer,
}

#[derive(Serialize, Deserialize)]
struct App {
    pub board: board::Board,
    pub turn: Turn,
    pub who_starts: Turn,
    pub wins: u32,
    pub losses: u32,
}

impl Default for App {
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

enum Msg {
    ColumnClick(usize),
    ComputerMakeMove(usize),
    ResetGame,
}


impl App {
    fn make_ai_move(&self, callback: Callback<usize>) {
        let board = self.board.clone();

        spawn_local(async move {
            // Wait for the screen to redraw
            sleep(Duration::from_millis(50)).await;

            let col = ai::best_move(&board);
            callback.emit(col);
        });
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        // Load the saved state at startup
        let app: App = LocalStorage::get(STORAGE_KEY).unwrap_or_default();

        if app.turn == Turn::Computer {
            // Trigger the computer to make a move if it is its turn
            let callback = ctx.link().callback(Msg::ComputerMakeMove);
            app.make_ai_move(callback);
        }
        app
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let mut move_made = false;

        match msg {
            Msg::ColumnClick(col) => {
                if self.turn == Turn::Player && self.board.check_winner().is_none() {
                    if self.board.player_move(col) {
                        move_made = true;
                        self.turn = Turn::Computer;
                        let callback = ctx.link().callback(Msg::ComputerMakeMove);
                        self.make_ai_move(callback);
                    }
                }
            },
            Msg::ComputerMakeMove(col) => {
                if self.turn == Turn::Computer && self.board.check_winner().is_none() {
                    if self.board.computer_move(col) {
                        move_made = true;
                        self.turn = Turn::Player;
                    }
                }
            },
            Msg::ResetGame => {
                self.board = board::Board::new();
                // Take turns starting
                self.who_starts = match self.who_starts {
                    Turn::Player => Turn::Computer,
                    Turn::Computer => Turn::Player
                };
                self.turn = self.who_starts.clone();
                if self.turn == Turn::Computer {
                    let callback = ctx.link().callback(Msg::ComputerMakeMove);
                    self.make_ai_move(callback);
                }
            },
        }

        if move_made {
            match self.board.check_winner() {
                Some(board::GameResult::PlayerWins) => self.wins += 1,
                Some(board::GameResult::ComputerWins) => self.losses +=1,
                _ => (),
            }
        }

        // Save the state of the model
        LocalStorage::set(STORAGE_KEY, &self).ok();

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let column_buttons = (0..board::NUM_COLUMNS)
            .map(|col|
                html! {
                    <th>
                        <button onclick={ctx.link().callback(move |_| Msg::ColumnClick(col))}>{ "↓" }</button>
                    </th>
                }
            );


        let cells = (0..board::NUM_ROWS)
            .map(|row| {
                let row = (0..board::NUM_COLUMNS)
                    .map(|col| {
                        let cell = match self.board.get_cell(row, col) {
                            board::Cell::Empty => "",
                            board::Cell::Computer => "🔴",
                            board::Cell::Player => "🔵",
                            };
                        html! {
                            <td class="board-cell">
                                { cell }
                            </td>
                        }
                    });

                html! {
                    <tr>
                        { for row }
                    </tr>
                }
            });

        let message = match self.board.check_winner() {
            Some(board::GameResult::PlayerWins) => "You Won!!",
            Some(board::GameResult::ComputerWins) => "Oh no, you have lost.",
            Some(board::GameResult::Draw) => "Looks like this one is a draw.",
            None => "",
        };

        html! {
            <div class="container">
                <table>
                    <tr>
                        { for column_buttons}
                    </tr>

                    { for cells }
                </table>

                if !message.is_empty() {
                    <div class="overlay">
                        <div class="message">
                            { message }
                            <br/>
                            <button onclick={ctx.link().callback(|_| Msg::ResetGame)}>{ "Play Again?" }</button>
                        </div>
                    </div>
                }
                <div>
                    { format!("Wins: {} Losses: {}", self.wins, self.losses) }
                </div>
            </div>
        }
    }
}


fn main() {
    yew::Renderer::<App>::new().render();
}
