use std::fs;

use rocket::{State, http::Status};
use uuid::Uuid;
use crate::{v1::ticket::Tickets, versions::v1::ticket::{TicketType, ticket_path}};

#[put("/v1/upload/<ticket>/file?<path>", format = "application/octet-stream", data = "<data>")]
pub fn upload_file(tickets: &State<Tickets>, ticket: &str, path: &str, data: Vec<u8>) -> Status {
    let uuid = Uuid::try_parse(&ticket);
    if uuid.is_err() {
        return Status::Forbidden
    }

    let res = tickets.lock();
    if res.is_err() {
        return Status::InternalServerError
    }

    let ticket_map = res.unwrap();
    if !ticket_map.contains_key(&uuid.clone().unwrap()) || ticket_map.get(&uuid.clone().unwrap()).unwrap().kind != TicketType::UPLOAD {
        return Status::BadRequest
    }

    let base_path = ticket_path(uuid.unwrap());
    let file_path = base_path.join(path.strip_prefix("/").unwrap_or(path));

    if !file_path.starts_with(base_path) {
        return Status::BadRequest
    }

    let mut creating = true;
    if fs::exists(file_path.clone()).unwrap_or(false) {
        creating = false;
    }

    if fs::write(file_path, data).is_err() {
        return Status::InternalServerError
    }

    if creating {
        return Status::Created
    }

    Status::NoContent
}
