use yew::prelude::*;

struct Model {
    value: i64,
}

#[function_component(App)]
fn app() -> Html {
    let state = use_state(|| Model { value: 0 });

    // need to be changed to the code that help us open the "choosing cards" page
    let onclick = {
        let state = state.clone();

        Callback::from(move |_| {
            state.set(Model {
                value: state.value + 1,
            })
        })
    };

    let buttons: Html = (0..5).map(|_| {
        html! {
            <button onclick={onclick.clone()}>{"+"}</button>
        }
    })
    .collect();

    let holes: Html = (0..2).map(|_| {
        html! {
            <button class="holes" onclick={onclick.clone()}>
                <span>{"+"}</span>
            </button>
        }
    })
    .collect();

    html! {
        <div>
            <div id="title">
                {"Texas Hold'em Poker"}
            </div>
            <div id="buttons-containers">
                {buttons}
            </div>
            <div id="holes-containers">
                {holes}
            </div>
            <p>{state.value}</p>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
