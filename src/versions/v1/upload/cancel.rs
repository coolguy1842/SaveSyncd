use rocket::{State, http::Status};
use uuid::Uuid;

use crate::versions::v1::ticket::{TicketType, Tickets, clear_ticket_path};

#[delete("/v1/upload/<ticket>")]
pub fn upload_cancel(tickets: &State<Tickets>, ticket: &str) -> Status {
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

    let ticket = ticket_map.get(&uuid.clone().unwrap()).unwrap();
    if ticket.kind != TicketType::UPLOAD {
        return Status::BadRequest
    }
    
    if clear_ticket_path(uuid.clone().unwrap()).is_err() {
        println!("Failed to clear ticket path {}", ticket.id.hyphenated());
    }
    
    ticket_map.remove(&uuid.unwrap());

    Status::NoContent
}