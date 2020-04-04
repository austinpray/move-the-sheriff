use std::cmp;
use std::process::exit;

use ctrlc;
use pancurses::{curs_set, endwin, initscr, Input, noecho, Window};
use quicli::prelude::*;
use structopt::StructOpt;
//use tokio::net::TcpStream;
//use tokio::prelude::*;
use uuid::Uuid;

use crate::engine::*;

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

    let mut sheriff = Entity {
        name: "sheriff".to_string(),
        model: "🤠".to_string(),
        id: player_id.clone(),
        x: 10,
        y: 10,
    };

    let mut state = State {
        current: "game".to_string(),
        entities: Default::default(),
    };

    state.entities.insert(sheriff.id.clone(), sheriff);


    loop {
        draw_window(&window, &state);
        match window.getch() {
            Some(Input::Character(c)) => {
                if c == 'q' {
                    break;
                }
                handle_input(&mut state, &window, &player_id, c);
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
