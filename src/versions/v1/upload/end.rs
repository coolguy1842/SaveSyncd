use fs_extra::dir;
use rocket::{State, http::Status};
use uuid::Uuid;

use crate::{config::Config, versions::v1::ticket::{TicketType, Tickets, clear_ticket_path, copy_dir_all, ticket_path}};

#[put("/v1/upload/<ticket>/end")]
pub fn upload_end(tickets: &State<Tickets>, config: &State<Config>, ticket: &str) -> Result<Status, Status> {
    let uuid = Uuid::try_parse(&ticket).map_err(|_| Status::Forbidden)?;
    let mut ticket_map = tickets.lock().map_err(|_| Status::InternalServerError)?;

    let Some(ticket) = ticket_map.get(&uuid).cloned() else { return Err(Status::BadRequest) };    
    if ticket.kind != TicketType::UPLOAD {
        return Err(Status::BadRequest)
    }

    ticket_map.remove(&ticket.id);
    if !config.data_directory().exists() {
        dir::create_all(config.data_directory(), false).expect("Failed to create data directory");
    }
    
    let title_path = config.data_directory().join(format!("{:04X}", ticket.title_id));
    let container_path = title_path.join(ticket.container.to_string().to_lowercase());

    if !title_path.exists() {
        dir::create_all(&title_path, false).expect("Failed to create titles permanent path");
    }
    else if !container_path.exists() {
        dir::remove(&container_path).expect("Failed to remove containers old permanent path");
    }

    copy_dir_all(ticket_path(ticket.id), &container_path).expect("Failed to copy staging path to title path");
    if clear_ticket_path(ticket.id).is_err() {
        println!("Failed to clear ticket path {}", ticket.id.hyphenated());
    }
    
    Ok(Status::NoContent)
}