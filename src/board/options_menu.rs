use sycamore::prelude::*;

#[component]
pub fn OptionsMenu<G: Html>(cx: Scope) -> View<G> {
    view! { cx, 
        ul {
            li {"Game"}
            li {"Display"}
        }
    }
}