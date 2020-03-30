use crate::engine::state::{Command, Entity};
use crate::engine::state::CommandName::*;

fn move_cmd(mut cmd: &mut Command, new_x: i32, new_y: i32) {
    cmd.name = Pos;
    cmd.x = new_x;
    cmd.y = new_y;
}

pub fn handle_input<'a>(c: char, sheriff: &'a Entity) -> Command<'a> {
    let id = sheriff.id();
    let mut cmd = Command{
        name: Continue,
        id: sheriff.id().clone(),
        x: 0,
        y: 0,
    };
    if c == 'q' {
        cmd.name = Quit;
        return cmd;
    }
    match c {
        'w' | 'k' => move_cmd(&mut cmd, sheriff.get_x(), sheriff.get_y() + 1),
        'a' | 'h' => move_cmd(&mut cmd, sheriff.get_x() - 1, sheriff.get_y()),
        's' | 'j' => move_cmd(&mut cmd, sheriff.get_x(), sheriff.get_y() - 1),
        'd' | 'l' => move_cmd(&mut cmd, sheriff.get_x() + 1, sheriff.get_y()),
        _ => (),
    }

    return cmd.clone();
}

