use seed::{prelude::*, *};

type Model = i32;

#[derive(Clone)]
enum Msg {
    Increment,
    Decrement,
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => *model += 1,
        Msg::Decrement => *model -= 1,
    }
}

fn view(model: &Model) -> Node<Msg> {
    div![
        button![ simple_ev(Ev::Click, Msg::Decrement), "-" ],
        div![ model.to_string() ],
        button![ simple_ev(Ev::Click, Msg::Increment), "+" ],
    ]
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view).build_and_start();
}
