use std::str::FromStr;

use fs_extra::dir::create_all;
use rocket::{State, http::Status, serde::{Deserialize, json::Json}};
use serde::Serialize;
use uuid::Uuid;
use crate::{v1::ticket::{Container, Ticket, TicketType, Tickets, ticket_path}, versions::v1::file_info::ClientFileInfo};


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
pub fn upload_begin(tickets: &State<Tickets>, data: Json<BeginBody>) -> Result<Json<BeginResponse>, Status> {
    let container = Container::from_str(&data.container).map_err(|_| Status::BadRequest)?;
    if data.files.is_empty() {
        return Err(Status::BadRequest)
    }

    let mut ticket_map = tickets.lock().map_err(|_| Status::InternalServerError)?;

    let ticket_id = Uuid::new_v4();
    let ticket = Ticket { id: ticket_id, title_id: data.id, kind: TicketType::UPLOAD, container: container };

    create_all(ticket_path(ticket_id), false).expect("Failed to create directories for ticket");
    ticket_map.insert(ticket_id, ticket);

    Ok(Json(BeginResponse { ticket: ticket.id.hyphenated().to_string(), files: data.files.iter().map(|f| f.path.clone()).collect() }))
}
