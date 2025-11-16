use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClientFileInfo {
    pub path: String,
    pub size: u64,
    pub hash: Option<String>
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServerFileInfo {
    pub path: String,
    pub size: u64,
    pub hash: String
}