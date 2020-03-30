use std::process::exit;

use ctrlc;
use pancurses::{curs_set, endwin, initscr, Input, noecho};
use quicli::prelude::*;
use structopt::StructOpt;
//use tokio::net::TcpStream;
use uuid::Uuid;

use crate::engine::input::handle_input;
use crate::engine::state::{Entity, State, tick};
use crate::engine::state::CommandName::*;

mod engine;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(long = "join", short = "j")]
    join: String,
    #[structopt(long = "username", short = "u", default_value = "anonymous")]
    username: String,
}

#[tokio::main]
pub async fn main() -> CliResult {
    //let args = Cli::from_args();
    //let username = String::from(&args.username[..15]);
    //let addr = args.join;

    //let stream;
    //if addr.len() > 0 {
    //    stream = TcpStream::connect(addr).await.unwrap();
    //}

    let window = initscr();
    window.nodelay(true);
    curs_set(0);
    noecho();

    ctrlc::set_handler(move || {
        endwin();
        exit(0)
    }).expect("Error setting Ctrl-C handler");

    let cid = Uuid::new_v4().to_string();
    let cactus = Entity::new(cid.as_str(), "🌵");
    let sid = Uuid::new_v4().to_string();
    let sheriff = Entity::new(sid.as_str(), "🤠");

    let mut state = State::new();
    state.add_entity(sheriff);
    state.add_entity(cactus);

    loop {
        match window.getch() {
            Some(Input::Character(c)) => {
                match state.get_entity(&sheriff.id()) {
                    None => {},
                    Some(&player) => {
                        match handle_input(c, &player).name {
                            Quit => break,
                            Continue => {}
                            Pos => {}
                        }
                    },
                }
                //stream.write(c.to_string().as_bytes()).await;
            }
            Some(_input) => {}
            None => {}
        }
        tick(&window, &state);
    }
    endwin();

    Ok(())
}
