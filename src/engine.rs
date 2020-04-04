use std::cmp;
use std::collections::{HashMap, HashSet};

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
    pub stage_max_x: i32,
    pub stage_max_y: i32
}

impl State {
    pub fn handle_move(self: &mut Self, id: &String, requested_x: i32, requested_y: i32) {
        let mut other_entity_positions: HashSet<(i32, i32)> = HashSet::new();
        for (_id, other_entity) in self.entities.iter() {
            other_entity_positions.insert((other_entity.x, other_entity.y));
            //other_entity_positions.insert((other_entity.x+1, other_entity.y+1));
        }

        match self.entities.get_mut(id) {
            None => {
                return;
            }
            Some(entity) => {
                let mut new_x = entity.x + requested_x;
                let mut new_y = entity.y + requested_y;

                // don't escape the stage!
                new_x = cmp::max(new_x, 0);
                new_x = cmp::min(new_x, self.stage_max_x - 2);
                new_y = cmp::max(new_y, 0);
                new_y = cmp::min(new_y, self.stage_max_y - 1);

                // don't collide with other entities!
                if other_entity_positions.contains(&(new_x, new_y)) {
                    return;
                }

                entity.x = new_x;
                entity.y = new_y;
            }
        }
    }
}

fn draw_main_stage(window: &Window, state: &State) {
    window.printw(format!("Welcome to “Move the Sheriff”! press <q> to quit\n"));
    for (_id, entity) in state.entities.iter() {
        let model = &entity.model;
        let mut line_number = 0;
        for line in model.split('\n') {
            window.mvprintw(entity.y + line_number, entity.x, line);
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

