use std::cmp;
use std::collections::{HashMap, HashSet};
use std::str::Lines;

use pancurses::Window;
use unicode_width::UnicodeWidthStr;
use libsheriff::State;


fn draw_main_stage(window: &Window, state: &State) {
    window.printw(format!("Welcome to “Move the Sheriff”! press <q> to quit\n"));
    for (_id, entity) in state.entities.iter() {
        let mut line_number = 0;
        for line in entity.get_lines() {
            window.mvaddstr(entity.y + line_number, entity.x, line);
            line_number += 1;
        }
    }
}

pub fn draw_window(window: &Window, state: &State) {
    window.erase();
    draw_main_stage(&window, &state);
    window.refresh();
}

pub fn handle_input(state: &mut State, id: &String, input: char) {
    match input {
        'w' | 'k' => state.handle_move(id, 0, -1),
        'a' | 'h' => state.handle_move(id, -1, 0),
        's' | 'j' => state.handle_move(id, 0, 1),
        'd' | 'l' => state.handle_move(id, 1, 0),
        _ => (),
    }
}

