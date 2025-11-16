use std::{collections::HashMap, env, path::PathBuf, str::FromStr, sync::{Arc, Mutex}};

use fs_extra::dir::remove;
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum TicketType {
    UPLOAD,
    DOWNLOAD
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Container {
    SAVE,
    EXTDATA
}

impl FromStr for Container {
    type Err = ();
    fn from_str(input: &str) -> Result<Container, Self::Err> {
        match input {
            "SAVE"    => Ok(Container::SAVE),
            "save"    => Ok(Container::SAVE),
            "EXTDATA" => Ok(Container::EXTDATA),
            "extdata" => Ok(Container::EXTDATA),
            _         => Err(()),
        }
    }
}

impl ToString for Container {
    fn to_string(&self) -> String {
        match self {
            Container::SAVE    => "SAVE".to_string(),
            Container::EXTDATA => "EXTDATA".to_string()
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Ticket {
    pub id: Uuid,
    pub title_id: u64,
    pub kind: TicketType,
    pub container: Container
}

pub type Tickets = Arc<Mutex<HashMap<Uuid, Ticket>>>;

pub fn tickets_path() -> PathBuf {
    env::temp_dir().join("SaveSyncdv1")
}

pub fn ticket_path(ticket: Uuid) -> PathBuf {
    tickets_path().join(ticket.hyphenated().to_string())
}

pub fn clear_tickets_path() -> Result<(), fs_extra::error::Error> {
    remove(tickets_path())
}

pub fn clear_ticket_path(ticket: Uuid) -> Result<(), fs_extra::error::Error> {
    remove(ticket_path(ticket))
}
