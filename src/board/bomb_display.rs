use sycamore::prelude::*;

#[component(inline_props)]
pub fn BombDisplay<'a, G: Html>(cx: Scope<'a>, num_flags: &'a ReadSignal<usize>, num_bombs: usize) -> View<G> {
    view! { cx,
        div (class="bomb display") {
            p { (num_bombs - *num_flags.get()) }
        }
    }
}