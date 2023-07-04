mod options_menu;
mod bomb_display;
mod reset_button;
mod timer_display;
mod cell;

use sycamore::prelude::*;
use options_menu::OptionsMenu;
use bomb_display::BombDisplay;
use reset_button::ResetButton;
use timer_display::TimerDisplay;
use cell::Cell;

#[component]
pub fn Board<G: Html>(cx: Scope) -> View<G> {
    let game_state = create_signal(cx, GameState::InProgress);
    
    let default_params = Params {height: 10, width: 10, mines: 10};
    let params = create_signal(cx, default_params);

    let num_flags = create_signal(cx, 0);



    view! { cx,
        div (class="board") {
            div (class="options") { OptionsMenu {} }
            div (class="display panel") {
                BombDisplay (num_flags=num_flags, num_bombs=(params.get().mines)) {}
                ResetButton (game_state=game_state) {}
                TimerDisplay {}
            }            
            div (class="cell grid") {

            }

        }
    }
}

#[derive(Clone, Debug)]
pub enum GameState {
    Won,
    Lost,
    InProgress,
}

impl std::fmt::Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Debug)]
pub struct Params {
    pub height: usize,
    pub width: usize,
    pub mines: usize,
}
