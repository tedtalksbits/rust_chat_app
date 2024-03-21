use std::{collections::HashMap, net::TcpStream, sync::Arc};

use futures_channel::mpsc::UnboundedSender;
use futures_util::{
    lock::Mutex,
    stream::{SplitSink, StreamExt},
};
use tokio::net::TcpListener;
use tokio_tungstenite::{
    accept_async,
    tungstenite::{client, Message},
    WebSocketStream,
};

type Tx = UnboundedSender<Message>;
type Rx = SplitSink<WebSocketStream<TcpStream>, Message>;

struct SharedState {
    clients: HashMap<usize, Tx>,
}

#[tokio::main]
async fn main() {
    // define state
    let state = Arc::new(Mutex::new(SharedState {
        clients: HashMap::new(),
    }));

    // bind the server to the address and port
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Listening on: 127.0.0.1:3000");
    // define a client id counter
    let mut client_id: usize = 0;

    // while the server is running, accept incoming connections
    while let Ok((stream, _)) = listener.accept().await {
        // spawn a new task to handle the incoming connection
        tokio::spawn(async move {
            // accept the incoming connection and perform the websocket handshake
            let ws_stream = accept_async(stream)
                .await
                .expect("Error during the websocket handshake");
            println!("New WebSocket connection");
            // split the websocket stream into a write and read half
            let (write, mut read) = ws_stream.split();

            while let Some(message) = read.next().await {
                match message {
                    Ok(msg) => {
                        println!("Received a message: {:?}", msg);
                        // Here you can handle messages, for example, broadcasting them to other clients
                        // or replying to the client
                        // reply to the client
                    }
                    Err(e) => {
                        eprintln!("Error reading message: {:?}", e);
                        break;
                    }
                }
            }
        });
    }
}
