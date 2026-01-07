use std::str::FromStr;

use fs_extra::dir::create_all;
use rocket::{State, http::Status, serde::{Deserialize, json::Json}};
use serde::Serialize;
use uuid::Uuid;
use crate::{config::Config, v1::ticket::{Container, Ticket, TicketType, Tickets, ticket_path}, versions::v1::file_info::{ClientFileInfo, file_hash}};

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct BeginBody {
    id: u64,
    container: String,
    files: Vec<ClientFileInfo>
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct BeginResponse {
    ticket: String,
    files: Vec<String>
}

#[post("/v1/upload/begin", format = "application/json", data = "<data>")]
pub fn upload_begin(tickets: &State<Tickets>, config: &State<Config>, data: Json<BeginBody>) -> Result<Json<BeginResponse>, Status> {
    let container = Container::from_str(&data.container).map_err(|_| Status::BadRequest)?;
    if data.files.is_empty() {
        return Err(Status::BadRequest)
    }

    let mut ticket_map = tickets.lock().map_err(|_| Status::InternalServerError)?;

    let ticket_id = Uuid::new_v4();
    let ticket = Ticket { id: ticket_id, title_id: data.id, kind: TicketType::UPLOAD, container: container };

    create_all(ticket_path(ticket_id), false).expect("Failed to create directories for ticket");
    ticket_map.insert(ticket_id, ticket);

    let title_path = config.data_directory().join(format!("{:X}", data.id));
    let container_path = title_path.join(container.to_string().to_lowercase());

    if !container_path.exists() {
        return Ok(Json(BeginResponse { ticket: ticket.id.hyphenated().to_string(), files: data.files.iter().map(|f| f.path.clone()).collect() }))
    }

    let mut files: Vec<String> = data.files.iter().map(|f| f.path.clone()).collect();
    files.sort();
    files.dedup();

    for file in &data.files {
        let Some(stripped_path) = file.path.strip_prefix("/") else { continue; };
        let file_path = container_path.join(stripped_path);

        if !file_path.exists() {
            continue;
        }

        let Ok(metadata) = file_path.metadata() else { continue; };

        if file.size != metadata.len() {
            continue;
        }

        let Ok(hash) = file_hash(&file_path) else { continue; };
        if file.hash != Some(hash) {
            continue;
        }

        if let Some(index) = files.iter().position(|path| *path == file.path) {
            files.swap_remove(index);
        }
    }

    if files.is_empty() {
        return Err(Status::NoContent)
    }

    Ok(Json(BeginResponse { ticket: ticket.id.hyphenated().to_string(), files }))
}
