//================================================================
pub mod models;
pub mod websockets;
pub mod sfu;
pub mod signal;
//================================================================
#[actix_rt::main] 
async fn main() -> Result<(), std::io::Error> {
    signal::server::run().await
}