

use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::accept_async;
use futures::StreamExt;
use tokio::time::{interval};

#[tokio::main]
async fn main() {
    let myAdress = "127.0.0.1:8080";
    let listener = TcpListener::bind(&myAdress).await.unwrap_or_else(|err| {panic!("can't resolve this shit with that fucking reason {}",err)});
    println!("Server is running on {}", &myAdress );

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_connection(stream));
    }
}

async fn handle_connection(stream: TcpStream) {
    if let Ok(ws_stream) = accept_async(stream).await {
        println!("WebSocket connection established");
        let (write, read) = ws_stream.split();
        let mut write = interval(std::time::Duration::from_secs(1)).zip(write); 
        
        while let Some(result) = read.next().await {
            if let Ok(msg) = result {
                let (interval, mut sender) = write.next().await.unwrap();
                sender.send(msg).await.unwrap();
            }
        }
    } else {
        println!("error");
    }
}
