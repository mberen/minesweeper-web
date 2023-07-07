use sycamore::prelude::*;

use super::GameStatus;
use super::BoardState;

#[derive(Props)]
pub struct ResetButtonProps<'a> {
    board_state: &'a BoardState,
}

#[component]
pub fn ResetButton<'a, G: Html>(cx: Scope<'a>, props: ResetButtonProps<'a>) -> View<G> {
    view! { cx, 
        div (class="resetButton") {
            button(on:click=move |_| { handle_click(cx, &props.board_state.game_status, props.board_state) }) {
                (
                    match *props.board_state.game_status.get() {
                        GameStatus::Won => "😎",
                        GameStatus::Lost => "☹️",
                        GameStatus::InProgress => "🔄"
                    }
                )
            }
        }
    }
}

fn handle_click (cx: Scope, game_status: &RcSignal<GameStatus>, board_state: &BoardState) {
    game_status.set(GameStatus::InProgress);

    let params = *board_state.params.get();
    board_state.reset(cx, &params);
}