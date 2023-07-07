use gloo_timers::future::TimeoutFuture;
use sycamore::futures::spawn_local_scoped;
use sycamore::prelude::*;

#[component]
pub fn TimerDisplay<G: Html>(cx: Scope) -> View<G> {
    let time = create_signal(cx, 0);

    spawn_local_scoped(cx, async {
        loop {
            TimeoutFuture::new(1000).await;
            time.set(*time.get() + 1);
        }
    });

    view! { cx,
        div (class="timer") {
            (time.get()) 
        }
    }
}