use std::env;
use actix_web::{HttpServer, middleware::Logger, App};
use dotenvy::dotenv;


//================================================================
pub mod models;
pub mod websockets;
pub mod sfu;

//================================================================
#[actix_rt::main] 
async fn main() -> Result<(), std::io::Error> {
    
    dotenv().ok();
    env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
    // env::set_var("RUST_BACKTRACE", "RUST_BACKTRACE=1");
    env_logger::init();

    let server_url = env::var("SERVER_URL")
    .expect("SERVER_URL must be set");



    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
    })
    .bind(server_url)?
    .run()
    .await
}