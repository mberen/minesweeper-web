use sycamore::prelude::*;

use super::GameState;

#[derive(Props)]
pub struct ResetButtonProps<'a> {
    game_state: &'a Signal<GameState>
}

#[component]
pub fn ResetButton<'a, G: Html>(cx: Scope<'a>, props: ResetButtonProps<'a>) -> View<G> {
    view! { cx, 
        div (class="reset button") {
            button(on:click=|_| { change_state(props.game_state) }) {(props.game_state.get().to_string())}
        }
    }
    
}

fn change_state<'a> (game_state: &'a Signal<GameState>) {
    match *game_state.get() {
        GameState::Won => game_state.set(GameState::Lost) ,
        GameState::Lost => game_state.set(GameState::InProgress) ,
        GameState::InProgress => game_state.set(GameState::Won) ,
    }
}