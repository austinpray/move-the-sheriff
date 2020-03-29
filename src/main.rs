use std::cmp;
use std::process::exit;

use ctrlc;
use pancurses::{curs_set, endwin, initscr, Input, noecho, Window};
use quicli::prelude::*;
use structopt::StructOpt;
use tokio::net::TcpStream;
use tokio::prelude::*;
use uuid::Uuid;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(long = "server", short = "s", default_value = "127.0.0.1:6142")]
    server: String,
    #[structopt(long = "username", short = "u", default_value = "anonymous")]
    username: String,
}

struct Sheriff {
    username: String,
    id: String,
    x: i32,
    y: i32,
}

fn draw_window(window: &Window, sheriff: &Sheriff) {
    window.clear();
    window.printw(format!("Welcome {} {}, Move the Sheriff! press <q> to quit\n", sheriff.username, sheriff.id));
    window.mvprintw(sheriff.y, sheriff.x, "🤠");
    window.refresh();
}

#[tokio::main]
pub async fn main() -> CliResult {
    let args = Cli::from_args();
    let username = args.username;
    let addr = args.server;

    let mut stream = TcpStream::connect(addr).await.unwrap();
    println!("created stream");

    let window = initscr();
    curs_set(0);

    ctrlc::set_handler(move || {
        println!("received Ctrl+C!");
        endwin();
        exit(0)
    })
        .expect("Error setting Ctrl-C handler");

    let mut sheriff = Sheriff { username: username, id: Uuid::new_v4().to_string(), x: 10, y: 10 };

    draw_window(&window, &sheriff);

    noecho();
    loop {
        match window.getch() {
            Some(Input::Character(c)) => {
                if c == 'q' {
                    break;
                }
                match c {
                    'w' | 'k' => sheriff.y = cmp::max(sheriff.y - 1, 0),
                    'a' | 'h' => sheriff.x = cmp::max(sheriff.x - 1, 0),
                    's' | 'j' => sheriff.y = cmp::min(sheriff.y + 1, window.get_max_y() - 1),
                    'd' | 'l' => sheriff.x = cmp::min(sheriff.x + 1, window.get_max_x() - 2),
                    _ => (),
                }
                draw_window(&window, &sheriff);
                stream.write(c.to_string().as_bytes()).await;
                ()
            }
            Some(input) => {}
            None => {}
        }
    }
    endwin();

    Ok(())
}
