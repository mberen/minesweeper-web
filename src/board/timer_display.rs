use gloo_timers::future::TimeoutFuture;
use sycamore::futures::spawn_local_scoped;
use sycamore::prelude::*;

use super::BoardState;
use super::GameStatus;

#[component]
pub fn TimerDisplay<G: Html>(cx: Scope) -> View<G> {
    let time = create_signal(cx, 0);
    let board_state = use_context::<BoardState>(cx);

    spawn_local_scoped(cx, async {
        loop {
            TimeoutFuture::new(1000).await;
            if let GameStatus::InProgress= *board_state.game_status.get() {
                time.set(*time.get() + 1);
            }
        }
    });

    create_effect(cx, || {
        board_state.cells.track();
        time.set(0);
    });

    view! { cx,

        div (class="timer") {
            (

                time.get()
            ) 
        }
    }
}