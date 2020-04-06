use actix_web::{App, get, HttpResponse, HttpServer, middleware, Responder, Result, web};
use actix_web::http::StatusCode;

use libsheriff::Game;
use std::sync::Mutex;

#[get("/state.json")]
async fn state(game_data: web::Data<Mutex<Game>>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(*game_data.lock().unwrap()))
}

#[get("/")]
async fn index() -> Result<HttpResponse> {
    // response
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body("ayy lmao"))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let mut game = Game::new(100, 100);
    game.seed_environment();

    let game_data = web::Data::new(Mutex::new(game));


    HttpServer::new(move || {
        App::new()
            .app_data(game_data.clone())
            .wrap(middleware::Logger::default())
            .service(index)
            .service(state)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
