use std::env;

use actix::Actor;
use actix_web::{HttpServer, App, middleware::Logger, web::{scope, Data}};
use dotenvy::dotenv;

use crate::websockets::{endpoint, lobby::Lobby};

pub async fn run() -> Result<(), std::io::Error> {
    dotenv().ok();
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=debug");
    env_logger::init();
    let server_url = env::var("SERVER_URL")
    .expect("SERVER_URL must be set");
    let lobby = Lobby::default().start();


    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(lobby.clone()))
            .wrap(Logger::default())
            .service(endpoint::create_ws_connection)
            .service(endpoint::health_check)
    })
    .bind(server_url)?
    .run()
    .await
}