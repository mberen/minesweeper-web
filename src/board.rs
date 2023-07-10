mod options_menu;
mod bomb_display;
mod reset_button;
mod timer_display;


use sycamore::prelude::*;
use rand::thread_rng;
use rand::seq::SliceRandom;
use web_sys::{MouseEvent, console};

use options_menu::OptionsMenu;
use bomb_display::BombDisplay;
use reset_button::ResetButton;
use timer_display::TimerDisplay;

#[derive(Debug, Clone)]
pub struct BoardState {
    cells: RcSignal<Vec<RcSignal<Cell>>>,
    params: RcSignal<Params>,
    game_status: RcSignal<GameStatus>
}

#[component]
pub fn Board<G: Html>(cx: Scope) -> View<G> {    
    let default_params = Params {height: 4, width: 4, mines: 1};

    let board_state = BoardState::new(default_params);
    provide_context(cx, board_state);
    let board_state = use_context::<BoardState>(cx);

    let num_flags = create_signal(cx, 0isize);
    provide_context_ref(cx, num_flags);

    let style = create_memo(cx, || {
        let width =  (*board_state.params.get()).width;
        format!("--row-length: {}", width)
    });

    view! { cx,
        div (class="board", 
            on:contextmenu=|e: MouseEvent| e.prevent_default(),
            style=style,
        ) {
            div (class="options") { OptionsMenu {} }
            div (class="displayPanel") {
                BombDisplay {}
                ResetButton {}
                TimerDisplay {}
            }            
            div (class="cellGrid") {
                (BoardState::view(cx))
            }
        }
    }
}


impl BoardState {
    fn new (params: Params) -> BoardState {
        let Params {height, width, mines} = params;
        let mut cells = vec!(Cell::Empty{cell_status: CellStatus::Hidden, mines: 0, id: 0}; height*width);
    
        for i in 0..mines {
            cells[i] = Cell::Mine{cell_status: CellStatus::Hidden, id: 0};
        }
        cells.shuffle(&mut thread_rng());
        for (i, cell) in cells.iter_mut().enumerate() {
            match cell {
                Cell::Mine{cell_status: s, id: _} => *cell=Cell::Mine{cell_status: (*s).clone(), id: i},
                Cell::Empty{cell_status: s, mines:m, id: _} => *cell=Cell::Empty{cell_status: (*s).clone(), mines: *m, id: i},
            }
        }

        let cells: Vec<RcSignal<Cell>> = 
            cells
            .into_iter()
            .map(|c|create_rc_signal(c))
            .collect();
    
        let board_state = BoardState {
            cells: create_rc_signal(cells),
            params: create_rc_signal(params),
            game_status: create_rc_signal(GameStatus::InProgress),
        };
    
        for i in 0..board_state.cells.get().len() {
            if let Cell::Empty{..} = *board_state.cells.get()[i].get() {
                let adjacent_bombs = board_state.get_adjacent_bombs(i);
                board_state.cells.get()[i].set(Cell::Empty{cell_status: CellStatus::Hidden, mines: adjacent_bombs, id: i});
            }
        }
    
        board_state
    }

    fn reset(&self, cx: Scope, params: &Params) {
        let num_flags = use_context::<Signal<isize>>(cx);
        num_flags.set(0);
        
        let new_state = Self::new(*params);
        self.cells.set_rc(new_state.cells.get());
        self.params.set_rc(new_state.params.get());
        self.game_status.set_rc(new_state.game_status.get());
    }

    //converts the cell vec to our desired html view (list of cells)
    fn view<G: Html> (cx: Scope) -> View<G> {

        let board_state = use_context::<BoardState>(cx);

        //let cells: &RcSignal<Vec<RcSignal<Cell>>> = create_ref(cx, board_state.cells.clone());

        let cells = create_memo(cx, || {
            board_state.cells
                .get()
                .iter()
                .cloned()
                .collect::<Vec<_>>()
        });

        view! { cx,
            ul {
                Indexed(
                    iterable=cells,
                    view=|cx, cell| {
                        let cell_ref = create_ref(cx, cell.clone());
                        view! { cx, 
                            li (class=(cell_ref.get().get_id()),
                                on:click =move |_| BoardState::cell_click(cx, cell_ref),
                                on:auxclick =move |click| BoardState::cell_aux_click(cx, cell_ref, click),) {
                                (
                                    match *cell.get() {
                                        Cell::Mine{cell_status: CellStatus::Flagged, ..} => "🚩".to_string(),
                                        Cell::Empty{cell_status: CellStatus::Flagged, ..} => "🚩".to_string(),
                                        Cell::Mine{cell_status: CellStatus::Hidden, ..} => "🔳".to_string(),
                                        Cell::Empty{cell_status: CellStatus::Hidden, ..} => "🔳".to_string(),
                                        Cell::Mine{cell_status: CellStatus::Revealed, ..} => "💥".to_string(),
                                        Cell::Empty{cell_status: CellStatus::Revealed, mines: n, ..} => {
                                            let c = char::from_digit(n, 10).expect("Adjacency always 9 or less");
                                            format!("{c} ")
                                        }
                                    }
                                )
                            }
                        }
                    }
                )
            }
        }
    }

    fn cell_click(cx: Scope, cell: &RcSignal<Cell>) {
        let board_state = use_context::<BoardState>(cx);

        if *board_state.game_status.get() == GameStatus::InProgress {
            match *cell.get() {
                ref c @ Cell::Empty{cell_status: CellStatus::Hidden, mines: 0, id: self_idx} => {
                    cell.set(c.new_status(CellStatus::Revealed));

                    let adjacent_list = board_state.get_adjacent(self_idx);
                    for (x, y) in adjacent_list {
                        let adj_idx = board_state.get_coord_index(&Coordinate(x, y));
                        if self_idx != adj_idx {
                            BoardState::cell_click(cx, &board_state.cells.get()[adj_idx]);
                        }
                    } 
                },
                ref c @ Cell::Empty{cell_status: CellStatus::Hidden, ..} => cell.set(c.new_status(CellStatus::Revealed)),
                ref c @ Cell::Mine{cell_status: CellStatus::Hidden, ..}=> {
                    cell.set(c.new_status(CellStatus::Revealed));
                    board_state.game_status.set(GameStatus::Lost);
                },
                _ => (),
            }
        }

        if board_state.game_won() { board_state.game_status.set(GameStatus::Won) }
    }

    fn cell_aux_click(cx: Scope, cell: &RcSignal<Cell>, click: MouseEvent) {
        let button = click.button();
        let num_flags = use_context::<Signal<isize>>(cx);
        match (&*cell.get(), button) {
            (c @ Cell::Mine{cell_status: CellStatus::Flagged, ..}, 2) => {
                cell.set(c.new_status(CellStatus::Hidden));
                num_flags.set(*num_flags.get() - 1);
            },
            (c @ Cell::Empty{cell_status: CellStatus::Flagged, ..}, 2) => {
                cell.set(c.new_status(CellStatus::Hidden));
                num_flags.set(*num_flags.get() - 1);
            },
            (c @ Cell::Mine{cell_status: CellStatus::Hidden, ..}, 2) => {
                cell.set(c.new_status(CellStatus::Flagged));
                num_flags.set(*num_flags.get() + 1);
            },
            (c @ Cell::Empty{cell_status: CellStatus::Hidden, ..}, 2) => {
                cell.set(c.new_status(CellStatus::Flagged));
                num_flags.set(*num_flags.get() + 1);
            },
            _ => (),
        };
    }

    pub fn get_coord_index(&self, Coordinate(x, y): &Coordinate) -> usize {
        y*self.params.get().width + x % self.params.get().height
    }

    pub fn get_coord_from_index(&self, index: usize) -> Coordinate {
        let x = index % self.params.get().width;
        let y = index / self.params.get().height;

        Coordinate(x, y)
    }

    pub fn get_adjacent(&self, i: usize) -> Vec<(usize, usize)>{
        let Coordinate(x, y) = self.get_coord_from_index(i);

        let adjacent_matrix: [(isize, isize); 9] = [ (-1, -1), (0, -1), (1, -1),
                                                    (-1, 0), (0, 0), (1, 0),
                                                    (-1, 1), (0, 1), (1, 1)];
    
        adjacent_matrix
            .iter()
            .map(|(dx, dy)| (x as isize+dx, y as isize+dy))
            .filter(|(x, y)| *x >= 0 && *y >= 0)
            .map(|(x, y)| { (x as usize, y as usize) })
            .filter(|(x, y)| *x < self.params.get().width && *y < self.params.get().height)
            .collect()
    }    
    
    pub fn get_adjacent_bombs(&self, i: usize) -> u32 {
        let adjacent_list =self.get_adjacent(i);

        let mut count = 0;
        for (x, y) in adjacent_list {
            let c = Coordinate(x, y);
            if let Cell::Mine{..} = *self.cells.get()[self.get_coord_index(&c)].get()  {
                count += 1;
            };
        };
        count
    }

    fn game_won (&self) -> bool {
        let cells = self.cells.get();
        for cell in cells.iter() {
            if let Cell::Empty{cell_status: CellStatus::Hidden, ..} = *cell.get() { return false }
        }
        true
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum GameStatus {
    Won,
    Lost,
    InProgress,
}
#[derive(Copy, Clone, Debug)]
struct Params {
    pub height: usize,
    pub width: usize,
    pub mines: usize,
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
enum Cell {
    Mine {cell_status: CellStatus, id: usize},
    Empty {cell_status: CellStatus, mines: u32, id: usize},
}

impl Cell {
    fn new_status(&self, status: CellStatus) -> Self {
        match self {
            Cell::Mine {cell_status: _, id: i} => Cell::Mine {cell_status: status, id: *i},
            Cell::Empty {cell_status: _, mines: n, id: i} => Cell::Empty {cell_status: status, mines: *n, id: *i},
        }
    }
    fn get_id(&self) -> usize {
        match self {
            Cell::Mine {id: i, ..} => *i,
            Cell::Empty {id: i, ..} => *i,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum CellStatus {
    Hidden,
    Revealed,
    Flagged,
}

#[derive(Debug, Clone)]
pub struct Coordinate(pub usize, pub usize);