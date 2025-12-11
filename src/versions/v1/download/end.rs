use rocket::{State, http::Status};
use uuid::Uuid;

use crate::versions::v1::ticket::{TicketType, Tickets, clear_ticket_path};

#[delete("/v1/download/<ticket>")]
pub fn download_end(tickets: &State<Tickets>, ticket: &str) -> Result<Status, Status> {
    let uuid = Uuid::try_parse(&ticket).map_err(|_| Status::Forbidden)?;
    let mut ticket_map = tickets.lock().map_err(|_| Status::InternalServerError)?;
    
    let Some(ticket) = ticket_map.get(&uuid).cloned() else { return Err(Status::BadRequest) };
    if ticket.kind != TicketType::DOWNLOAD {
        return Ok(Status::BadRequest)
    }
    
    if clear_ticket_path(uuid).is_err() {
        println!("Failed to clear ticket path {}", ticket.id.hyphenated());
    }
    
    ticket_map.remove(&uuid);
    Ok(Status::NoContent)
}