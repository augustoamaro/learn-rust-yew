use yew::prelude::*;

struct Model {
    value: i64,
}

fn app() -> Html {
    let state = use_state(|| Model { value: 0 });
}