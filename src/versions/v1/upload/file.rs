use std::fs;

use rocket::{State, http::Status};
use uuid::Uuid;
use crate::{v1::ticket::Tickets, versions::v1::ticket::{TicketType, ticket_path}};

#[put("/v1/upload/<ticket>/file?<path>", format = "application/octet-stream", data = "<data>")]
pub fn upload_file(tickets: &State<Tickets>, ticket: &str, path: &str, data: Vec<u8>) -> Result<Status, Status> {
    let uuid = Uuid::try_parse(&ticket).map_err(|_| Status::Forbidden)?;
    let ticket_map = tickets.lock().map_err(|_| Status::InternalServerError)?;

    let Some(ticket) = ticket_map.get(&uuid).cloned() else { return Err(Status::BadRequest) };    
    if ticket.kind != TicketType::UPLOAD {
        return Err(Status::BadRequest)
    }

    let base_path = ticket_path(uuid);
    let file_path = base_path.join(path.strip_prefix("/").unwrap_or(path));
    if !file_path.starts_with(base_path) {
        return Err(Status::BadRequest)
    }

    let created = !file_path.exists();
    if fs::write(file_path, data).is_err() {
        return Err(Status::InternalServerError)
    }

    Ok(
        match created {
        true => Status::Created,
        false => Status::NoContent
        }
    )
}
