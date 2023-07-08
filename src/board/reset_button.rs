use sycamore::prelude::*;

use super::GameStatus;
use super::BoardState;


#[component]
pub fn ResetButton<G: Html>(cx: Scope) -> View<G> {
    let board_state = use_context::<BoardState>(cx);

    view! { cx, 
        div (class="resetButton") {
            button(on:click=move |_| { handle_click(cx) }) {
                (
                    match *board_state.game_status.get() {
                        GameStatus::Won => "😎",
                        GameStatus::Lost => "☹️",
                        GameStatus::InProgress => "🔄"
                    }
                )
            }
        }
    }
}

fn handle_click (cx: Scope) {
    let board_state = use_context::<BoardState>(cx);

    board_state.game_status.set(GameStatus::InProgress);

    let params = *board_state.params.get();
    board_state.reset(cx, &params);
}