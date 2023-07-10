use sycamore::prelude::*;
use super::BoardState;
use super::Params;

#[component]
pub fn OptionsMenu<G: Html>(cx: Scope) -> View<G> {
    let board_state = use_context::<BoardState>(cx);

    let option_clicked = create_signal(cx, false);
    let height = create_signal(cx, "10".to_string());
    let width = create_signal(cx, "10".to_string());
    let mines = create_signal(cx, "10".to_string());

    let max_bombs = create_memo(cx, || {
        let height: usize = height.get().parse().unwrap();
        let width: usize = width.get().parse().unwrap();
        height * width - 1
    });


    let handle_options_click = |_| {
        option_clicked.set(true);
    };

    let handle_cancel = |_| {
        option_clicked.set(false);
    };

    let handle_new_game = move |_| {
        let height: usize = height.get().parse().unwrap();
        let width: usize = width.get().parse().unwrap();
        let mines: usize = mines.get().parse().unwrap();

        board_state.reset(cx, &Params{height, width, mines});
        option_clicked.set(false);
    };

    view! { cx, 
        (
            if !*option_clicked.get() {
                view! { cx, 
                    ul {
                        li (on:click=handle_options_click) {"Options"}
                    }
                }
            }
            else {
                view! { cx, 
                    div (class="game_options") {
                        div {
                            label (for="height") {"Height"}
                            input (type= "number", min="1", id="height", bind:value=height)
                        }   
                        div {
                            label (for="width") {"Width"}
                            input (type = "number", min="1", id="width", bind:value=width)
                        }
                        div {
                            label (for="mines") {"Mines"}
                            input (type = "number", id="mines", min = "1", max=*max_bombs.get(), bind:value=mines)
                        }
                        button (on:click=handle_new_game){"New Game"}
                        button (on:click=handle_cancel){"Cancel"}
                    }
                }
            }
        )
    }
}