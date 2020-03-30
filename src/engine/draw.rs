use pancurses::Window;
use crate::engine::state::State;

pub fn draw_main_stage(window: &Window, state: &State) {
    window.erase();
    window.printw(format!("Move the Sheriff! press <q> to quit\n"));
    for (_, e) in state.entities.iter() {
        window.mvprintw(e.get_y(), e.get_x(), e.model());
    }
    window.refresh();
}
