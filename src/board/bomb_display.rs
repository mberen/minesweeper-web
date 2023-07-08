use sycamore::prelude::*;
use super::BoardState;

#[component]
pub fn BombDisplay<G: Html>(cx: Scope) -> View<G> {
    let num_flags = use_context::<Signal<isize>>(cx);
    let num_bombs = (*use_context::<BoardState>(cx).params.get()).mines;
    view! { cx,
        div (class="bomb display") {
            ((num_bombs as isize) - *num_flags.get())
        }
    }
}