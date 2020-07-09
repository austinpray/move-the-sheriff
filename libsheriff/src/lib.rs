use std::cmp;
use std::collections::{HashMap, HashSet};
use std::str::Lines;

use unicode_width::UnicodeWidthStr;

pub struct Entity {
    pub name: String,
    pub model: String,
    pub id: String,
    pub x: i32,
    pub y: i32,
}

/// # Examples
///
/// ```
/// use libsheriff::Entity;
/// let mut e = Entity::new("sheriff", "🤠", "abc123", (10, 11));
/// assert_eq!(e.x, 10);
/// assert_eq!(e.y, 11);
///
/// assert_eq!(e.get_width(), 2);
/// assert_eq!(e.get_height(), 1);
///
/// e.set_pos(20, 21);
///
/// assert_eq!(e.x, 20);
/// assert_eq!(e.y, 21);
/// ```
impl Entity {
    pub fn new(name: &str, model: &str, id: &str, pos: (i32, i32)) -> Self {
        Entity {
            name: name.to_string(),
            model: model.to_string(),
            id: id.to_string(),
            x: pos.0,
            y: pos.1,
        }
    }
    pub fn set_pos(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    pub fn get_lines(self: &Self) -> Lines<'_> {
        self.model.lines()
    }
    pub fn get_width(self: &Self) -> usize {
        let mut max = 0;
        for line in self.model.lines() {
            max = cmp::max(line.width_cjk(), max)
        }
        return max;
    }
    pub fn get_height(self: &Self) -> usize {
        self.get_lines().count()
    }
}

pub struct State {
    pub current: String,
    pub entities: HashMap<String, Entity>,
    pub stage_max_x: i32,
    pub stage_max_y: i32,
}

impl State {
    pub fn handle_move(self: &mut Self, id: &String, requested_x: i32, requested_y: i32) {
        // TODO: filter other entities if they are not within reach

        let mut other_entity_positions: HashSet<(i32, i32)> = HashSet::new();
        for (other_id, other_entity) in self.entities.iter() {
            if other_id == id {
                continue;
            }
            for y in 0..other_entity.get_height() {
                for x in 0..other_entity.get_width() {
                    let hitbox_x = other_entity.x + x as i32;
                    let hitbox_y = other_entity.y + y as i32;
                    other_entity_positions.insert((hitbox_x, hitbox_y));
                }
            }
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
                for hitbox_y in 0..entity.get_height() {
                    for hitbox_x in 0..entity.get_width() {
                        let x = new_x + hitbox_x as i32;
                        let y = new_y + hitbox_y as i32;
                        if other_entity_positions.contains(&(x, y)) {
                            return;
                        }
                    }
                }

                entity.x = new_x;
                entity.y = new_y;
            }
        }
    }
}
