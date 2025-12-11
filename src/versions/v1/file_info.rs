use std::{fs, io::{self, Read}, path::Path};

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

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Copy)]
pub enum DownloadAction {
    KEEP,
    REPLACE,
    CREATE,
    REMOVE
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct DownloadFileInfo {
    pub path: String,
    pub size: Option<u64>,
    pub hash: Option<String>,
    pub action: DownloadAction
}

pub fn file_hash(path: &Path) -> std::io::Result<String> {
    let file = fs::File::open(path)?;

    let mut context = md5::Context::new();
    let mut buffer = [0; 4096];
    let mut reader = io::BufReader::new(file);

    loop {
        let bytes_read = reader.read(&mut buffer).expect("Failed to get bytes from file");
        if bytes_read == 0 {
            break;
        }

        context.consume(&buffer[..bytes_read]);
    }

    Ok(format!("{:x}", context.finalize()))
}