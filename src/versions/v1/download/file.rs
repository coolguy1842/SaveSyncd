use std::fs;

use rocket::{State, http::Status};
use uuid::Uuid;
use crate::{v1::ticket::Tickets, versions::v1::ticket::{TicketType, ticket_path}};

#[get("/v1/download/<ticket>/file?<path>", format = "application/octet-stream")]
pub fn download_file(tickets: &State<Tickets>, ticket: &str, path: &str) -> Result<Vec<u8>, Status> {
    let uuid = Uuid::try_parse(&ticket).map_err(|_| Status::Forbidden)?;
    let ticket_map = tickets.lock().map_err(|_| Status::InternalServerError)?;

    let Some(ticket) = ticket_map.get(&uuid).cloned() else { return Err(Status::BadRequest) };
    if ticket.kind != TicketType::DOWNLOAD {
        return Err(Status::BadRequest)
    }

    let base_path = ticket_path(uuid);
    let file_path = base_path.join(path.strip_prefix("/").unwrap_or(path));
    if !file_path.starts_with(base_path) {
        return Err(Status::BadRequest)
    }

    let metadata = fs::metadata(&file_path).map_err(|_| Status::Forbidden)?;
    if !metadata.is_file() {
        return Err(Status::Forbidden)
    }
    
    Ok(fs::read(&file_path).map_err(|_| Status::InternalServerError)?)
}
