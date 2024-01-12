use std::env;

use actix_web::{HttpServer, App, middleware::Logger};

use crate::websockets::endpoint;

pub async fn run() -> Result<(), std::io::Error> {
    let server_url = env::var("SERVER_URL")
    .expect("SERVER_URL must be set");



    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(endpoint::create_ws_connection)
            
    })
    .bind(server_url)?
    .run()
    .await
}