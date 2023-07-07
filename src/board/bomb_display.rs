use sycamore::prelude::*;

#[component(inline_props)]
pub fn BombDisplay<'a, G: Html>(cx: Scope<'a>, num_bombs: usize) -> View<G> {
    let num_flags = use_context::<Signal<isize>>(cx);
    view! { cx,
        div (class="bomb display") {
            ((num_bombs as isize) - *num_flags.get())
        }
    }
}