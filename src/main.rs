mod board;

use sycamore::prelude::*;
use board::Board;

fn main() {
    sycamore::render(|cx| view! { cx,
        Board {}
    });
}