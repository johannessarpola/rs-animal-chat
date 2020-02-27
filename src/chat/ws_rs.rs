use std::borrow::{Borrow, BorrowMut};
use std::cell::Cell;
use std::future::Future;
use std::process::id;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

use chrono::{DateTime, Utc};
use futures::executor::block_on;
use futures::TryFutureExt;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use ws::{
    CloseCode,
    Error,
    Handler,
    Handshake,
    listen,
    Message,
    Request,
    Response,
    Result,
    Sender,
};

#[derive(Serialize, Deserialize)]
struct MessageEntry {
    msg: String,
    timestamp: String,
    id: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct AssignedIdentityMessage {
    id: String,
}

// Server web application handler
struct Server {
    out: User,
    count: Arc<Mutex<u32>>,
}

struct User {
    socket: Sender,
    animal: String,
}

async fn analyze_messages(message_entry: &MessageEntry) {
    if (message_entry.msg.contains("fuck")) {
        println!("Cussing alert!");
    }
}

async fn id_assigned_message(server: &mut Server) {
    println!("id_assigned_message");
    // Send ID assigned message
    match serde_json::to_string(&(AssignedIdentityMessage { id: server.out.animal.to_string() })) {
        Ok(json) => {
            server.out.socket.send(Message::Text(json));
            ()
        }
        Err(err) => println!("Could not send identity assigned"),
    }
    return ();
}

async fn connection_opened_message(server: &mut Server) {
    println!("connection_opened_message");
    // We have a new connection, so we increment the connection counter
    //server.count.set(server.count.get() + 1);

    *server.count.lock().unwrap() += 1;

    let number_of_connection = server.count.lock().unwrap();
    let msg = format!("{} entered and the number of live connections is {}", &server.out.animal, &number_of_connection); // Ip address of the connected user
    println!("{}", &msg);
    let now: DateTime<Utc> = Utc::now();


    let outbound_message = MessageEntry {
        msg: msg,
        id: Some(server.out.animal.to_string()),
        timestamp: now.to_rfc2822(),
    };

    match serde_json::to_string(&outbound_message) {
        Ok(json) => {
            server.out.socket.broadcast(Message::Text(json.to_string()));
        }
        Err(err) => println!("Could not serialize outbound message"),
    };

    return ();
}

impl Handler for Server {
    fn on_request(&mut self, req: &Request) -> Result<(Response)> {
        match req.resource() {
            "/ws" => {
                // https://ws-rs.org/api_docs/ws/struct.Request.html
                println!("Browser Request from {:?}", req.origin().unwrap().unwrap());
                println!("Client found is {:?}", req.client_addr().unwrap());

                let resp = Response::from_request(req).map( | mut r| {
                    let h = r.headers_mut();
                    h.append(&mut vec![("id".to_owned(), b"id".to_vec())]);
                    r
                });
                resp
            }

            _ => Ok(Response::new(404, "Not Found", b"404 - Not Found".to_vec())),
        }
    }

    fn on_open(&mut self, handshake: Handshake) -> Result<()> {

        block_on(async {
            id_assigned_message(self).await;
            connection_opened_message(self).await;
        });

        return Ok(()); // ERROR?
    }

    fn on_message(&mut self, message: Message) -> Result<()> {
        let parsed: std::result::Result<MessageEntry, serde_json::error::Error> = serde_json::from_str(message.as_text()?);
        match parsed {
            Err(err) => println!("Invalid message received: {}", err.to_string()),
            Ok(messageEntry) => {
                println!("The message from the client is {:#?}", &messageEntry.msg);
                let identity = self.out.animal.to_string();
                let outbound_message = MessageEntry {
                    msg: messageEntry.msg,
                    id: Some(identity.to_string()),
                    timestamp: messageEntry.timestamp,
                };
                analyze_messages(&outbound_message);

                match serde_json::to_string(&outbound_message) {
                    Ok(json) => return self.out.socket.broadcast(Message::Text(json.to_string())),
                    Err(err) => println!("Could not serialize outbound message"),
                };
            }
        }


        return Ok(()); // ERROR?
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        match code {
            CloseCode::Normal => println!("The client is done with the connection."),
            CloseCode::Away => println!("The client is leaving the site."),
            CloseCode::Abnormal => {
                println!("Closing handshake failed! Unable to obtain closing status from client.")
            }
            _ => println!("The client encountered an error: {}", reason),
        }
        if (*self.count.lock().unwrap() > 0) {
            *self.count.lock().unwrap() -= 1;
        }
    }

    fn on_error(&mut self, err: Error) {
        println!("The server encountered an error: {:?}", err);
    }
}

pub fn choose_random_animal() -> String {
    let animals = vec!["Giraffe", "Hippotamus", "Chimpanzee", "Lion", "Buffalo"];

    return animals
        .choose_multiple(&mut rand::thread_rng(), 1)
        .map(|v| v.to_string())
        .collect();
}

fn create_user(out: Sender) -> User {
    User { socket: out, animal: choose_random_animal() }
}

fn create_server(out: Sender) -> Server {
    // Rc is a reference-counted box for sharing the count between handlers
    // since each handler needs to own its contents.
    // Cell gives us interior mutability so we can increment
    // or decrement the count between handlers.
    let count = Arc::new(Mutex::new(0));
    { Server { out: create_user(out), count: count.clone() } }
}

pub fn websocket() -> () {
    println!("Web Socket Server is ready at ws://127.0.0.1:7777/ws");
    println!("Server is ready at http://127.0.0.1:7777/");
    listen("127.0.0.1:7777", |out| create_server(out)).unwrap()
}