use yew::prelude::*;
// enum Suit {
//     Hearts,
//     Diamonds,
//     Clubs,
//     Spades,
// }

struct Model {
    value : i64
    // rank: vec<Suit>; 
    // number: vec<i64>; 
}

#[function_component(App)]
fn app() -> Html {
    let state = use_state(|| Model {
        value : 0
    }); 

    let onclick = {
        let state = state.clone(); 

        Callback::from(move |_| {
            state.set(Model {
                value:state.value + 1
            })
        })
    }; 

    let buttons: Html = (0..5).map(|_| {
        html! {
            <button onclick = {onclick.clone()}>{"+"}</button>
        }
    }).collect(); 

    let holes: Html = (0..2).map(|_| {
        html! {
            <button onclick = {onclick.clone()}>{"hole"}</button>
        }
    }).collect(); 

    html! {
        <div>
            <div id="title">
                {"welcome to our website!"}
            </div>
            <div id = "buttons-containers">
                {buttons}
            </div>
            <div id = "holes-containers">
                {holes}
            </div>
            <p>{state.value}</p>
        </div>
    }
}

fn main() {
    yew::start_app::<App>(); 
}