use serde::Serialize;

#[derive(Serialize)]
pub struct Message {
    pub status_code: u16,
    pub message: String
}