pub mod ws_rs;
pub mod tokio_tungstenite_ws;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MessageEntry {
    msg: String,
    timestamp: String,
    id: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct AssignedIdentityMessage {
    id: String,
}
