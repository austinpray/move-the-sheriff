use std::process::exit;

use ctrlc;
use pancurses::{curs_set, endwin, initscr, Input, noecho};
use quicli::prelude::*;
use rand::{Rng, thread_rng};
use structopt::StructOpt;
//use tokio::net::TcpStream;
//use tokio::prelude::*;
use uuid::Uuid;

use indoc::indoc;

use crate::engine::*;
use libsheriff::{Entity, State};

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


    let mut state = State {
        current: "game".to_string(),
        stage_max_x: window.get_max_x(),
        stage_max_y: window.get_max_y(),
        entities: Default::default(),
    };

    state.entities.insert(sheriff.id.clone(), sheriff);

    let id = Uuid::new_v4().to_string();
    state.entities.insert(id.clone(), Entity {
        name: "cactus".to_string(),
        model: "🌵".to_string(),
        //model: "X".to_string(),
        id: id.clone(),
        x: 16,
        y: 10,
    });

    let id = Uuid::new_v4().to_string();
    state.entities.insert(id.clone(), Entity {
        name: "cow".to_string(),
        model: "🐄".to_string(),
        //model: "X".to_string(),
        id: id.clone(),
        x: 20,
        y: 12,
    });

    let id = Uuid::new_v4().to_string();
    state.entities.insert(id.clone(), Entity {
        name: "desert".to_string(),
        model: indoc!("🦂🌵🌵
                       🌵🦂🌵️").to_string(),
        //model: indoc!("XXX
        //               MMM").to_string(),
        id: id.clone(),
        x: 16,
        y: 14,
    });

    let mut rng = thread_rng();

    let id = Uuid::new_v4().to_string();
    state.entities.insert(id.clone(), Entity {
        name: "horse".to_string(),
        model: "🐎".to_string(),
        //model: "X".to_string(),
        id: id.clone(),
        x: rng.gen_range(30, window.get_max_x() - 2),
        y: rng.gen_range(1, window.get_max_y() - 2),
    });

    for _ in 1..=15 {
        let id = Uuid::new_v4().to_string();
        let x = rng.gen_range(0, window.get_max_x() - 2);
        let y = rng.gen_range(1, window.get_max_y() - 2);

        state.entities.insert(id.clone(), Entity {
            name: "cactus".to_string(),
            model: "🌵".to_string(),
            id,
            x,
            y,
        });
    }

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
