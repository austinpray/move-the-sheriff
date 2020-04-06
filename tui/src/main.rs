use std::process::exit;

use ctrlc;
use pancurses::{curs_set, endwin, initscr, Input, noecho};
use quicli::prelude::*;
use rand::{Rng, thread_rng};
use structopt::StructOpt;
//use tokio::net::TcpStream;
//use tokio::prelude::*;
use uuid::Uuid;


use crate::engine::*;
use libsheriff::{Entity, Game};

mod engine;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(long = "server", short = "s", default_value = "127.0.0.1:6142")]
    server: String,
    #[structopt(long = "username", short = "u", default_value = "anonymous")]
    username: String,
}

#[tokio::main]
pub async fn main() -> CliResult {
    //let args = Cli::from_args();
    //let username = args.username;
    //let addr = args.server;

    //let mut stream = TcpStream::connect(addr).await.unwrap();

    let window = initscr();
    curs_set(0);
    noecho();

    ctrlc::set_handler(move || {
        println!("received Ctrl+C!");
        endwin();
        exit(0)
    })
        .expect("Error setting Ctrl-C handler");

    let player_id = Uuid::new_v4().to_string();

    let sheriff = Entity {
        name: "sheriff".to_string(),
        model: "🤠".to_string(),
        //model: "M".to_string(),
        id: player_id.clone(),
        x: 10,
        y: 10,
    };


    let mut state = Game::new(window.get_max_x(), window.get_max_y());

    state.seed_environment();

    state.entities.insert(sheriff.id.clone(), sheriff);


    loop {
        draw_window(&window, &state);
        match window.getch() {
            Some(Input::Character(c)) => {
                if c == 'q' {
                    break;
                }
                handle_input(&mut state, &player_id, c);
                //stream.write(c.to_string().as_bytes()).await;
                ()
            }
            Some(_input) => {}
            None => {}
        }
    }
    endwin();

    Ok(())
}
