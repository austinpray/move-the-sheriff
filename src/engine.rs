use std::cmp;
use std::collections::HashMap;

use pancurses::Window;

pub struct Entity {
    pub(crate) name: String,
    pub(crate) model: String,
    pub(crate) id: String,
    pub(crate) x: i32,
    pub(crate) y: i32,
}

pub struct State {
    pub current: String,
    pub entities: HashMap<String, Entity>,
}

fn draw_main_stage(window: &Window, state: &State) {
    window.printw(format!("Welcome to “Move the Sheriff”! press <q> to quit\n"));
    for (_id, entity) in state.entities.iter() {
        window.mvprintw(entity.y, entity.x, &entity.model);
    }
}

pub fn draw_window(window: &Window, state: &State) {
    window.erase();
    draw_main_stage(&window, &state);
    window.refresh();
}

pub fn handle_input(state: &mut State, window: &Window, id: &String, input: char) {
    match state.entities.get_mut(id) {
        None => {
            return;
        }
        Some(entity) => {
            match input {
                'w' | 'k' => entity.y = cmp::max(entity.y - 1, 0),
                'a' | 'h' => entity.x = cmp::max(entity.x - 1, 0),
                's' | 'j' => entity.y = cmp::min(entity.y + 1, window.get_max_y() - 1),
                'd' | 'l' => entity.x = cmp::min(entity.x + 1, window.get_max_x() - 2),
                _ => (),
            }
        }
    }
}

