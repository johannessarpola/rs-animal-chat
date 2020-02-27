#![feature(
proc_macro_hygiene,
decl_macro,
rustc_private,
type_ascription
)]
#[macro_use]
extern crate rocket;

extern crate ws;

use std::thread;


mod route;
use crate::route::{get, static_files};

mod chat;
use crate::chat::ws_rs;

fn rocket() -> rocket::Rocket {
    let rocket_routes = routes![
        static_files::file,
        get::index,
        get::chat,
        get::small_window
    ];

    rocket::ignite()
        .mount("/", rocket_routes)
}

fn main() {
    // 1.
    thread::Builder::new()
        .name("Thread for Rust Chat with ws-rs".into())
        // 2.
        .spawn(|| {
            ws_rs::websocket();
        })
        .unwrap();

    rocket().launch();
}