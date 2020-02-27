use std::io;
use rocket::response::{NamedFile};

#[get("/")]
pub fn index() -> &'static str {
    "Visit http://localhost:8000/chat"
}

#[get("/chat")]
pub fn chat() -> io::Result<NamedFile> {
    NamedFile::open("static/chat/index.html")
}

#[get("/small")]
pub fn small_window() -> io::Result<NamedFile> {
    NamedFile::open("static/small_window_chat/index.html")
}
