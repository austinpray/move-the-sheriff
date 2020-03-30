use pancurses::Window;

use chrono::prelude::*;
use crate::engine::draw::draw_main_stage;
use std::collections::HashMap;

#[derive(Copy, Clone)]
pub enum CommandName {
    Continue,
    Pos,
    Quit,
}

#[derive(Copy, Clone)]
pub struct Command<'a> {
    pub name: CommandName,
    id: &'a str,
    pub x: i32,
    pub y: i32,
}

struct StageBounds {
    max_x: i32,
    max_y: i32,
    min_x: i32,
    min_y: i32,
}


fn now_millis() -> i64 {
    let now = Utc::now();
    return now.timestamp_millis();
}

#[derive(Copy, Clone)]
pub struct Entity<'a> {
    id: &'a str,
    model: &'a str,
    width: u8,
    height: u8,
    x: i32,
    y: i32,
}


impl<'a> Entity<'a> {
    pub fn new(id: &'a str, model: &'a str) -> Self {
        Entity {
            id,
            model,
            width: 1,
            height: 1,
            x: 0,
            y: 0
        }
    }

    pub fn model(&self) -> &str {
        return self.model.clone();
    }

    pub fn width(&self) -> u8 {
        return self.width;
    }

    pub fn height(&self) -> u8 {
        return self.height;
    }

    pub fn id(&self) -> &str {
        return self.id.clone();
    }

    pub fn get_x(&self) -> i32 {
        return self.x;
    }

    pub fn get_y(&self) -> i32 {
        return self.y;
    }

    pub fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: i32) {
        self.y = y;
    }
}

pub enum Modes {
    MainMenu,
    Stage,
}

pub struct State<'a> {
    pub mode: Modes,
    pub entities: HashMap<&'a str, Entity<'a>>
}

impl<'a> State<'a> {
    pub fn new() -> Self {
        let mut entities: HashMap<&'a str, Entity> = HashMap::new();
        State {
            mode: Modes::Stage,
            entities
        }
    }

    pub fn add_entity(&mut self, e: Entity<'a>) {
        self.entities.insert(e.id().clone(), e);
    }

    pub fn get_entity(&mut self, k: &str) -> Option<&Entity> {
        return self.entities.get(k);
    }
}

pub fn tick(window: &Window, state: &State) {
    draw_main_stage(window, state);
}

