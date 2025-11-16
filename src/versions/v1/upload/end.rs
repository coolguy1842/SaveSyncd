use std::fs::{exists, rename};

use fs_extra::{dir::{CopyOptions, copy, create_all, remove}};
use rocket::{State, http::Status};
use uuid::Uuid;

use crate::{config::Config, versions::v1::ticket::{TicketType, Tickets, clear_ticket_path, ticket_path}};

#[put("/v1/upload/<ticket>/end")]
pub fn upload_end(tickets: &State<Tickets>, config: &State<Config>, ticket: &str) -> Status {
    let uuid = Uuid::try_parse(&ticket);
    if uuid.is_err() {
        return Status::Forbidden
    }

    let res = tickets.lock();
    if res.is_err() {
        return Status::InternalServerError
    }

    let mut ticket_map = res.unwrap();
    if !ticket_map.contains_key(&uuid.clone().unwrap()) {
        return Status::BadRequest
    }

    let ticket = ticket_map.get(&uuid.clone().unwrap()).unwrap().clone();
    if ticket.kind != TicketType::UPLOAD {
        return Status::BadRequest
    }

    ticket_map.remove(&ticket.id);
    if !exists(config.data_directory()).unwrap_or(false) {
        create_all(config.data_directory(), false).expect("Failed to create data directory");
    }
    
    let title_path = config.data_directory().join(format!("{:X}", ticket.title_id));
    let container_path = title_path.join(ticket.container.to_string().to_lowercase());

    if !exists(title_path.clone()).unwrap_or(false) {
        create_all(title_path.clone(), false).expect("Failed to create titles permanent path");
    }
    else if exists(container_path.clone()).unwrap_or(false) {
        remove(container_path.clone()).expect("Failed to create containers permanent path")
    }

    copy(ticket_path(ticket.id), title_path.clone(), &CopyOptions::new()).expect("Failed to copy staging path to title path");
    rename(title_path.join(ticket.id.hyphenated().to_string()), container_path).expect("Failed to rename to container path");
    
    if clear_ticket_path(ticket.id).is_err() {
        println!("Failed to clear ticket path {}", ticket.id.hyphenated());
    }
    
    Status::NoContent
}