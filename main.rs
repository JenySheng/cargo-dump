use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let modal_title = use_state(|| "".to_string());
    let show_modal = use_state(|| false);

    let toggle_modal_with_title = {
        let modal_title = modal_title.clone();
        let show_modal = show_modal.clone();
        move |title: String| {
            if *show_modal && *modal_title == title {
                show_modal.set(false);
            } else {
                modal_title.set(title);
                show_modal.set(true);
            }
        }
    };

    let close_modal = {
        let show_modal = show_modal.clone();
        move |_| {
            show_modal.set(false);
        }
    };

    let suits = vec!['♣', '♦', '♥', '♠'];
    let ranks = vec!["2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A"];
    let cards: Vec<Html> = suits.iter().map(|&suit| {
        let suit_buttons = ranks.iter().map(move |&rank| {
            html! {
                <button class="card-button">
                    {format!("{}{}", suit, rank)}
                </button>
            }
        }).collect::<Html>();
        html! {
            <div class="suit-line">
                { suit_buttons }
            </div>
        }
    }).collect();

    let modal_content = html! {
        <div class="modal-content">
            <div class="modal-header">
                <h2 class="modal-title">{ modal_title.as_str() }</h2>
                <button class="close-button" onclick={close_modal.clone()}>{"X"}</button>
            </div>
            { for cards }
        </div>
    };

    let modal = if *show_modal {
        html! {
            <div class="modal" onclick={close_modal.clone()}>
                {modal_content}
            </div>
        }
    } else {
        html! {}
    };

    let buttons_top_row: Html = (1..=5).map(|n| {
        let toggle_modal = toggle_modal_with_title.clone();
        html! {
            <button class="top-row-button" onclick={move |_| {
                let title = match n {
                    1..=3 => "Choose the Flop cards",
                    4 => "Choose the Turn card",
                    5 => "Choose the River card",
                    _ => "Unexpected number",
                };
                toggle_modal(title.to_string());
            }}>
                { "+" }
            </button>
        }
    }).collect();

    let buttons_bottom_row: Html = (1..=2).map(|_| {
        let toggle_modal = toggle_modal_with_title.clone();
        html! {
            <button class="bottom-row-button" onclick={move |_| toggle_modal("Choose your Hole cards".to_string())}>
                { "+" }
            </button>
        }
    }).collect();

    html! {
        <div id="background">
            <div id="title">
                {"Texas Hold'em Poker"}
            </div>
            <div id="top-row">
                {buttons_top_row}
            </div>
            <div id="bottom-row">
                {buttons_bottom_row}
            </div>
            {modal}
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
