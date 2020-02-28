use std::{
    collections::HashMap,
    env,
    io::Error as IoError,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use rand::seq::SliceRandom;

use futures::stream::StreamExt;
use futures_channel::mpsc::{unbounded, UnboundedSender};
use futures_util::{
    future, pin_mut,
    stream::TryStreamExt,
};
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;
use tungstenite::protocol::Message;
use crate::chat::{MessageEntry, AssignedIdentityMessage};
use serde_json::Error;
use futures::executor::block_on;
use std::thread::sleep;
use std::time::Duration;
use std::borrow::Borrow;

type Tx = UnboundedSender<Message>;
type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;

async fn analyze_messages(message_entry: MessageEntry) {
    sleep(Duration::from_millis(5000)); // TODO remove
    if message_entry.msg.contains("fuck") {
        println!("Cussing alert!");
    }
}


fn send_id_assigned(socket: &UnboundedSender<Message>, id: &str) {
    let msg = AssignedIdentityMessage { id : id.to_string()};
    serde_json::to_string(&msg).map(|txt| {
        socket.unbounded_send(Message::Text(txt));
    });
}

fn broadcast_message(message: Message, peer_map: &PeerMap) {

    let peers = peer_map.lock().unwrap();

    // We want to broadcast the message to everyone except ourselves.
    let broadcast_recipients = peers
        .iter()
        //.filter(|(peer_addr, _)| peer_addr != &&user.addr)
        .map(|(_, ws_sink)| ws_sink);

    for recp in broadcast_recipients {
        recp.unbounded_send(message.clone()).unwrap();
    }
}

async fn handle_connection(peer_map: PeerMap, raw_stream: TcpStream, user: User) {
    println!("Incoming TCP connection from: {}", user.addr);

    let ws_stream = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake occurred");
    println!("WebSocket connection established: {}", user.addr);

    // Insert the write part of this peer to the peer map.
    let (write, read) = unbounded();

    send_id_assigned(&write, &user.id);
    peer_map.lock().unwrap().insert(user.addr, write);

    let (outgoing, incoming) = ws_stream.split();

    let broadcast_incoming = incoming.try_for_each(|msg: Message| {
        let message_entry: std::result::Result<MessageEntry, serde_json::error::Error> = serde_json::from_str(&msg.to_string());
        match message_entry {
                Err(err) => println!("Invalid message received: {}", err.to_string()),
            Ok(message_entry) => {
                println!("The message from the client is {:#?}", &message_entry.msg);

                let outbound_message = MessageEntry {
                    msg: message_entry.msg,
                    id: Some(user.id.to_string()),
                    timestamp: message_entry.timestamp,
                };

                println!(
                    "Received a message from {}: {}",
                    user.addr,
                    msg.to_text().unwrap()
                );

                broadcast_message(msg, &peer_map);
                // Move message to analysis pipeline
                tokio::spawn( analyze_messages(outbound_message));


            },

        }
        future::ok(())
    });

    let receive_from_others = read.map(Ok).forward(outgoing);

    pin_mut!(broadcast_incoming, receive_from_others);
    future::select(broadcast_incoming, receive_from_others).await;

    println!("{} disconnected", &user.addr);
    peer_map.lock().unwrap().remove(&user.addr);
}

fn choose_random_animal() -> String {
    let animals = vec!["Giraffe", "Hippotamus", "Chimpanzee", "Lion", "Buffalo"];

    return animals
        .choose_multiple(&mut rand::thread_rng(), 1)
        .map(|v| v.to_string())
        .collect();
}

struct User {
    id: String,
    addr: SocketAddr,
}

#[tokio::main]
pub async fn websocket() -> Result<(), IoError> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());

    let state = PeerMap::new(Mutex::new(HashMap::new()));

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let mut listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", addr);

    // Let's spawn the handling of each connection in a separate task.
    while let Ok((stream, addr)) = listener.accept().await {
        let user = User { id : choose_random_animal(), addr : addr };
        tokio::spawn(handle_connection(state.clone(), stream, user));
    }

    Ok(())
}